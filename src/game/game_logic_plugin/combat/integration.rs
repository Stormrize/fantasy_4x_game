use super::ai;
use super::model::{CombatCommand, CombatOutcome, CombatPlayer, Combatant};
use super::simulation;
use crate::app_state::AppState;
use crate::world::components::{Id, PlayerType};
use crate::world::game_configs::{GameLogicConfig, Player, Players};
use crate::world::game_resources::data::unit::{
    CombatBasicStats, CurrentHp, LevelAbilities, UnitLevel, UnitType,
};
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;
use bevy::app::{App, Plugin, Update};
use bevy::math::IVec2;
use bevy::prelude::{
    Commands, IntoScheduleConfigs, Message, MessageReader, OnEnter, Query, Res, ResMut, Resource,
    in_state,
};
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use std::collections::{HashMap, HashSet};

/// State of the combat loop (SimulationTime).
#[derive(Resource, Debug, Default)]
pub struct CombatCycle {
    pub phase: CombatPhase,
}

#[derive(Default, Debug)]
pub enum CombatPhase {
    /// No combat is running.
    #[default]
    Idle,
    /// Combat was declared; the local player queues orders before the simulation.
    PlayerPlanning,
    /// Combat declared; the world snapshot must be collected and simulated.
    Planning,
    /// Bot AI is thinking and the simulation is computed in the background.
    Resolving(Task<CombatOutcome>),
}

/// Orders queued by the local player during [`CombatPhase::PlayerPlanning`].
/// The UI fills this resource; combat consumes and clears it.
#[derive(Resource, Default)]
pub struct CombatOrders(pub Vec<CombatCommand>);

/// Sent by the combat UI when the player wants to resolve the planned round.
#[derive(Message)]
pub struct ResolveCombat;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatCycle>()
            .init_resource::<CombatOrders>()
            .add_message::<ResolveCombat>()
            .add_systems(OnEnter(AppState::InGameAssetLoading), reset_combat_state)
            .add_systems(
                Update,
                (request_resolution, begin_combat, poll_combat)
                    .chain()
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}

/// A fresh game must not inherit an unfinished combat or stale orders.
fn reset_combat_state(mut commands: Commands) {
    commands.insert_resource(CombatCycle::default());
    commands.insert_resource(CombatOrders::default());
}

/// The UI can only request a simulation while the player is still planning.
fn request_resolution(mut cycle: ResMut<CombatCycle>, mut requests: MessageReader<ResolveCombat>) {
    if requests.read().count() == 0 {
        return;
    }
    if matches!(cycle.phase, CombatPhase::PlayerPlanning) {
        cycle.phase = CombatPhase::Planning;
        println!("[combat] simulation requested by the player");
    }
}

/// Gathers combat units and starts planning + simulation in the background.
fn begin_combat(
    mut cycle: ResMut<CombatCycle>,
    mut orders: ResMut<CombatOrders>,
    units_map: Res<UnitsMap>,
    fog_map: Res<FogMap>,
    combat_query: Query<(
        &Id,
        &UnitLevel,
        &UnitType,
        &CombatBasicStats,
        &CurrentHp,
        &LevelAbilities,
    )>,
    players: Res<Players>,
    game_logic_config: Res<GameLogicConfig>,
) {
    match cycle.phase {
        CombatPhase::Planning => {}
        CombatPhase::PlayerPlanning => {
            // Never stall combat when there is nothing to command: the player
            // needs at least one combat unit and at least one visible enemy.
            let local_id = game_logic_config.player.id;
            let size = units_map.size;
            let mut has_own = false;
            let mut has_enemy = false;
            for (index, slot) in units_map.map.iter().enumerate() {
                let Some(entity) = *slot else { continue };
                let Ok((owner, ..)) = combat_query.get(entity) else {
                    continue;
                };
                if *owner == local_id {
                    has_own = true;
                } else {
                    let (row, col) = (index / size, index % size);
                    if fog_map.debug_full_visibility || fog_map.is_general_visible_player(row, col)
                    {
                        has_enemy = true;
                    }
                }
            }
            if !has_own || !has_enemy {
                cycle.phase = CombatPhase::Planning;
            }
            return;
        }
        _ => return,
    }

    let combatants = collect_combatants(&units_map, &combat_query);
    let players_state = collect_players(&players);
    let bot_ids: Vec<Id> = game_logic_config.players.iter().map(|p| p.id).collect();

    // Fairness: each bot only sees enemies through its fog of war (full visibility).
    let local_player_id = game_logic_config.player.id;
    let visible_enemies = collect_visible_enemies(
        &units_map,
        &fog_map,
        &combat_query,
        &bot_ids,
        local_player_id,
    );

    let human_commands = std::mem::take(&mut orders.0);

    let task = AsyncComputeTaskPool::get().spawn(async move {
        plan_and_simulate(
            combatants,
            players_state,
            bot_ids,
            human_commands,
            visible_enemies,
        )
    });

    cycle.phase = CombatPhase::Resolving(task);
}

/// Applies the finished simulation and hands the turn back to the player.
fn poll_combat(
    mut cycle: ResMut<CombatCycle>,
    mut orders: ResMut<CombatOrders>,
    mut turn_manager: ResMut<TurnManager>,
    mut units_map: ResMut<UnitsMap>,
    mut current_hp_query: Query<&mut CurrentHp>,
    mut players: ResMut<Players>,
    mut commands: Commands,
) {
    let CombatPhase::Resolving(task) = &mut cycle.phase else {
        return;
    };
    let Some(outcome) = future::block_on(future::poll_once(task)) else {
        return;
    };

    for line in &outcome.log {
        println!("[combat] {line}");
    }

    apply_outcome(
        outcome,
        &mut units_map,
        &mut current_hp_query,
        &mut players,
        &mut commands,
    );

    // Combat is over: return the turn to the starting (human) player.
    turn_manager.current_player = Player::new(Id(turn_manager.starting_index), PlayerType::Human);
    cycle.phase = CombatPhase::Idle;
    orders.0.clear();
    println!("Combat simulation resolved");
}

/// Bots prepare their commands and resolve the round.
fn plan_and_simulate(
    combatants: Vec<Combatant>,
    players: Vec<CombatPlayer>,
    bot_ids: Vec<Id>,
    human_commands: Vec<CombatCommand>,
    visible_enemies: HashMap<Id, HashSet<IVec2>>,
) -> CombatOutcome {
    let mut commands = human_commands;

    for bot_id in bot_ids {
        let own: Vec<Combatant> = combatants
            .iter()
            .filter(|c| c.owner == bot_id)
            .cloned()
            .collect();
        let visible: HashSet<IVec2> = visible_enemies.get(&bot_id).cloned().unwrap_or_default();
        let enemies: Vec<Combatant> = combatants
            .iter()
            .filter(|c| c.owner != bot_id && visible.contains(&c.pos))
            .cloned()
            .collect();
        let mana = players
            .iter()
            .find(|p| p.id == bot_id)
            .map(|p| p.mana)
            .unwrap_or(0);
        commands.extend(ai::plan_combat(&own, &enemies, mana));
    }

    simulation::simulate(combatants, players, &commands)
}

fn collect_combatants(
    units_map: &UnitsMap,
    combat_query: &Query<(
        &Id,
        &UnitLevel,
        &UnitType,
        &CombatBasicStats,
        &CurrentHp,
        &LevelAbilities,
    )>,
) -> Vec<Combatant> {
    let size = units_map.size;
    let mut combatants = Vec::new();

    for (index, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok((owner, level, unit_type, basic_stats, current_hp, abilities)) =
            combat_query.get(entity)
        else {
            continue;
        };
        let pos = bevy::math::IVec2::new((index / size) as i32, (index % size) as i32);
        let mut unit = Combatant::new(
            *owner,
            pos,
            *unit_type,
            *level,
            *basic_stats,
            abilities.0.clone(),
        );
        unit.max_health = basic_stats.health;
        unit.health = current_hp.0.min(unit.max_health);
        combatants.push(unit);
    }

    combatants
}

fn collect_players(players: &Players) -> Vec<CombatPlayer> {
    players
        .0
        .iter()
        .map(|p| CombatPlayer {
            id: p.id,
            mana: p.current_mana,
        })
        .collect()
}

/// For each bot: positions of enemy units visible to it (full visibility).
fn collect_visible_enemies(
    units_map: &UnitsMap,
    fog_map: &FogMap,
    combat_query: &Query<(
        &Id,
        &UnitLevel,
        &UnitType,
        &CombatBasicStats,
        &CurrentHp,
        &LevelAbilities,
    )>,
    bot_ids: &[Id],
    local_player_id: Id,
) -> HashMap<Id, HashSet<IVec2>> {
    let size = units_map.size;
    let mut visible: HashMap<Id, HashSet<IVec2>> = HashMap::new();

    for (index, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok((owner, ..)) = combat_query.get(entity) else {
            continue;
        };
        let (row, col) = (index / size, index % size);
        for bot_id in bot_ids {
            if *owner == *bot_id {
                continue;
            }
            if fog_map.is_full_visible_for(row, col, *bot_id, local_player_id) {
                visible
                    .entry(*bot_id)
                    .or_default()
                    .insert(IVec2::new(row as i32, col as i32));
            }
        }
    }

    visible
}

fn apply_outcome(
    outcome: CombatOutcome,
    units_map: &mut UnitsMap,
    current_hp_query: &mut Query<&mut CurrentHp>,
    players: &mut Players,
    commands: &mut Commands,
) {
    let size = units_map.size;
    let health_map: HashMap<(i32, i32), u16> = outcome
        .combatants
        .iter()
        .map(|c| ((c.pos.x, c.pos.y), c.health))
        .collect();

    let mut updates = Vec::new();
    for (index, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let pos = ((index / size) as i32, (index % size) as i32);
        if let Some(&health) = health_map.get(&pos) {
            updates.push((index, entity, health));
        }
    }

    for (index, entity, health) in updates {
        if health == 0 {
            units_map.set(index / size, index % size, None);
            commands.entity(entity).despawn();
        } else if let Ok(mut current_hp) = current_hp_query.get_mut(entity) {
            current_hp.0 = health;
        }
    }

    for player in outcome.players {
        if let Some(data) = players.0.iter_mut().find(|p| p.id == player.id) {
            data.current_mana = player.mana;
        }
    }
}
