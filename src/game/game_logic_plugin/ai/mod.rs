pub mod memory;
pub mod perception;
pub mod planning;
pub mod profile;

use crate::game::game_logic_plugin::action::action_systems::action_router;
use crate::game::game_logic_plugin::action::action_systems::end_turn::end_turn;
use crate::game::game_logic_plugin::ai::memory::{BotMemories, BotMemory};
use crate::game::game_logic_plugin::ai::perception::build_snapshot;
use crate::game::game_logic_plugin::ai::planning::plan_bot_turn;
use crate::game::game_logic_plugin::ai::profile::AiProfile;
use crate::game::game_logic_plugin::events_resources_structs::action_messages::{Action, EndTurn};
use crate::ui::menus::AppState;
use crate::world::components::{AiDifficulty, Id};
use crate::world::game_configs::{GameLogicConfig, Player, Players};
use crate::world::game_resources::WorldMaps;
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType};
use crate::world::game_resources::data::unit::{
    HelperBasicStats, MovementPoints, UnitLevel, UnitType,
};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, MessageWriter, Query, Res, ResMut, Resource, in_state};
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};

/// Active bot turn. Inserted by `end_turn` when entering `TriggerBotAI`
/// and removed once the plan has been executed.
pub struct ActiveBotTurn {
    pub player: Player,
    pub phase: AiPhase,
}

/// Result of bot turn planning.
#[derive(Debug, Default)]
pub struct BotTurnPlan {
    pub actions: Vec<Action>,
}

pub enum AiPhase {
    /// The world is not inspected yet; the plan has not started.
    Planning,
    /// The plan is being computed in a background task and polled by the system.
    Executing(Task<BotTurnPlan>),
}

/// Resource wrapper for `Option<ActiveBotTurn>` (`Option` itself is not a Resource).
#[derive(Resource, Default)]
pub struct ActiveBotTurnState(pub Option<ActiveBotTurn>);

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BotMemories>()
            .init_resource::<ActiveBotTurnState>()
            .add_systems(
                Update,
                (
                    start_bot_turn,
                    // The executor writes messages before the action handlers so
                    // bot actions are applied in the same frame.
                    execute_bot_turn.before(action_router).before(end_turn),
                )
                    .chain()
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}

/// Inspects the world through the bot's eyes and starts planning in the
/// background. The planner sees only what the bot has really seen (fog of war
/// plus memory).
#[allow(clippy::too_many_arguments)]
fn start_bot_turn(
    mut active_bot_turn: ResMut<ActiveBotTurnState>,
    world_maps: WorldMaps,
    buildings_query: Query<(&Id, &BuildingType, &BuildingLevel)>,
    units_query: Query<(&Id, &UnitType)>,
    helpers_query: Query<(&MovementPoints, &HelperBasicStats)>,
    combat_query: Query<(&Id, &UnitType, &UnitLevel, &MovementPoints)>,
    players: Res<Players>,
    game_logic_config: Res<GameLogicConfig>,
    mut memories: ResMut<BotMemories>,
) {
    let Some(state) = active_bot_turn.0.as_mut() else {
        return;
    };
    if !matches!(state.phase, AiPhase::Planning) {
        return;
    }

    let bot_id = state.player.id;
    let profile = AiProfile::from_difficulty(
        game_logic_config
            .ai_difficulty
            .unwrap_or(AiDifficulty::Normal),
    );

    let memory = memories.0.entry(bot_id).or_insert_with(|| {
        BotMemory::default_with_id(
            bot_id,
            game_logic_config.player.id,
            &world_maps,
            &buildings_query,
            &units_query,
        )
    });

    let snapshot = build_snapshot(
        bot_id,
        game_logic_config.player.id,
        &world_maps,
        &buildings_query,
        &units_query,
        &helpers_query,
        &combat_query,
        &players,
        &game_logic_config,
        memory,
        &profile,
    );

    let task = AsyncComputeTaskPool::get().spawn(async move { plan_bot_turn(snapshot, profile) });
    state.phase = AiPhase::Executing(task);
}

/// Once the plan is ready, turns it into regular game messages — the bot acts
/// through the same interface as a human player and goes through the same rules.
fn execute_bot_turn(
    mut active_bot_turn: ResMut<ActiveBotTurnState>,
    mut actions: MessageWriter<Action>,
    mut end_turn: MessageWriter<EndTurn>,
) {
    let Some(state) = active_bot_turn.0.as_mut() else {
        return;
    };
    let AiPhase::Executing(task) = &mut state.phase else {
        return;
    };

    let Some(mut plan) = future::block_on(future::poll_once(task)) else {
        return;
    };

    for action in plan.actions.drain(..) {
        match action {
            Action::Move {
                unit_pos,
                target_pos,
            } => {
                actions.write(Action::Move {
                    unit_pos,
                    target_pos,
                });
            }
            Action::Gather { resource_pos } => {
                actions.write(Action::Gather { resource_pos });
            }
            Action::Build {
                entity_pos,
                building_type,
            } => {
                actions.write(Action::Build {
                    entity_pos,
                    building_type,
                });
            }
            Action::SpawnUnit {
                city_pos,
                unit_type,
            } => {
                actions.write(Action::SpawnUnit {
                    city_pos,
                    unit_type,
                });
            }
            Action::Upgrade { entity_pos, layer } => {
                actions.write(Action::Upgrade { entity_pos, layer });
            }
            Action::RevealMap {
                center,
                full,
                reveal_source,
            } => {
                actions.write(Action::RevealMap {
                    center,
                    full,
                    reveal_source,
                });
            }
            _ => {}
        }
    }

    end_turn.write(EndTurn);
    active_bot_turn.0 = None;
}
