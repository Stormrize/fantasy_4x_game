use super::ai;
use super::model::{CombatOutcome, CombatPlayer, Combatant};
use super::simulation;
use crate::ui::menus::AppState;
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
use bevy::prelude::{Commands, IntoScheduleConfigs, Query, Res, ResMut, Resource, in_state};
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use std::collections::{HashMap, HashSet};

/// Состояние боевого цикла (SimulationTime).
#[derive(Resource, Default)]
pub struct CombatCycle {
    pub phase: CombatPhase,
}

#[derive(Default)]
pub enum CombatPhase {
    /// Бой не идёт.
    #[default]
    Idle,
    /// Бой объявлен, нужно собрать мир и запустить фоновое планирование.
    Planning,
    /// ИИ ботов думает и симуляция считается в фоне.
    Resolving(Task<CombatOutcome>),
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatCycle>().add_systems(
            Update,
            (begin_combat, poll_combat)
                .chain()
                .run_if(in_state(AppState::InGameRenderer)),
        );
    }
}

/// Собирает боевых юнитов и запускает планирование + симуляцию в фоне.
fn begin_combat(
    mut cycle: ResMut<CombatCycle>,
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
    if !matches!(cycle.phase, CombatPhase::Planning) {
        return;
    }

    let combatants = collect_combatants(&units_map, &combat_query);
    let players_state = collect_players(&players);
    let bot_ids: Vec<Id> = game_logic_config.players.iter().map(|p| p.id).collect();

    // Честность: каждый бот видит только врагов в тумане войны (full visibility).
    let local_player_id = game_logic_config.player.id;
    let visible_enemies = collect_visible_enemies(
        &units_map,
        &fog_map,
        &combat_query,
        &bot_ids,
        local_player_id,
    );

    let task = AsyncComputeTaskPool::get().spawn(async move {
        // Человек пока не готовит атаки (боевой UI — позже), его команды пусты.
        plan_and_simulate(
            combatants,
            players_state,
            bot_ids,
            Vec::new(),
            visible_enemies,
        )
    });

    cycle.phase = CombatPhase::Resolving(task);
}

/// Когда симуляция готова — применяет исход и возвращает ход человеку.
fn poll_combat(
    mut cycle: ResMut<CombatCycle>,
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

    // Бой окончен — ход возвращается стартовому игроку (человеку).
    turn_manager.current_player = Player::new(Id(turn_manager.starting_index), PlayerType::Human);
    cycle.phase = CombatPhase::Idle;
    println!("Combat simulation resolved");
}

/// Боты готовят команды и разрешают бой.
fn plan_and_simulate(
    combatants: Vec<Combatant>,
    players: Vec<CombatPlayer>,
    bot_ids: Vec<Id>,
    human_commands: Vec<super::model::CombatCommand>,
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

/// Для каждого бота — позиции вражеских юнитов, видимые ему (full visibility).
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
