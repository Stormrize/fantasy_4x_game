pub mod memory;
pub mod perception;
pub mod plan;
pub mod planning;
pub mod profile;

use crate::game::game_logic_plugin::action::action_systems::end_turn::end_turn;
use crate::game::game_logic_plugin::action::action_systems::move_entity::move_entity;
use crate::game::game_logic_plugin::ai::memory::BotMemories;
use crate::game::game_logic_plugin::ai::perception::build_snapshot;
use crate::game::game_logic_plugin::ai::plan::{AiAction, BotTurnPlan};
use crate::game::game_logic_plugin::ai::planning::plan_bot_turn;
use crate::game::game_logic_plugin::ai::profile::AiProfile;
use crate::game::game_logic_plugin::events_resources_structs::action_messages::{
    ActionWriters, Build, EndTurn, Gather, Move, RevealMap, SpawnUnitCommand, Upgrade,
};
use crate::game::game_logic_plugin::events_resources_structs::components::TileLayer;
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

/// Активный ход бота. Вставляется в `end_turn` при переходе `TriggerBotAI`
/// и убирается, когда план выполнен.
pub struct ActiveBotTurn {
    pub player: Player,
    pub phase: AiPhase,
}

pub enum AiPhase {
    /// Мир ещё не осмотрен, план не начат.
    Planning,
    /// План считается в фоновой задаче, система его опрашивает.
    Executing(Task<BotTurnPlan>),
}

/// Ресурс-обёртка для `Option<ActiveBotTurn>` (Option сам по себе не Resource).
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
                    // Исполнитель пишет сообщения до обработчиков ходов,
                    // чтобы действия бота применялись в этом же кадре.
                    execute_bot_turn.before(move_entity).before(end_turn),
                )
                    .chain()
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}

/// Осматривает мир глазами бота и запускает планирование в фоне.
/// Планировщик видит только то, что бот реально видел (туман войны + память).
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
    let memory = memories.0.entry(bot_id).or_default();

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

/// Когда план готов, превращает его в обычные игровые сообщения — бот играет
/// через тот же интерфейс, что и человек, и те же проверки правил.
fn execute_bot_turn(
    mut active_bot_turn: ResMut<ActiveBotTurnState>,
    mut action_writers: ActionWriters,
    mut move_writer: MessageWriter<Move>,
    mut reveal_writer: MessageWriter<RevealMap>,
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
            AiAction::Move { from, to } => {
                move_writer.write(Move {
                    entity_pos: from,
                    target_pos: to,
                });
            }
            AiAction::Gather { pos } => {
                action_writers.gather_cmd.write(Gather(pos));
            }
            AiAction::Build { pos, building_type } => {
                action_writers.build_cmd.write(Build {
                    entity_pos: pos,
                    building_type: Some(building_type),
                });
            }
            AiAction::SpawnUnit {
                city_pos,
                unit_type,
            } => {
                action_writers.spawn_cmd.write(SpawnUnitCommand {
                    city_pos,
                    unit_type,
                });
            }
            AiAction::UpgradeBuilding { pos } => {
                action_writers.upgrade.write(Upgrade {
                    entity_pos: pos,
                    layer: TileLayer::Building,
                });
            }
            AiAction::RevealMap { center } => {
                reveal_writer.write(RevealMap { center });
            }
        }
    }

    action_writers.end_turn.write(EndTurn);
    active_bot_turn.0 = None;
}
