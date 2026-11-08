//! Single entry point for all player / AI actions.
pub mod build_entity;
pub mod collect_income;
pub mod end_turn;
pub mod fog_update;
pub mod gather_entity;
pub mod move_entity;
pub mod reset_movement_points;
pub mod spawn_unit;
pub mod upgrade_entity;

use crate::game::game_logic_plugin::action::action_systems::build_entity::build_entity;
use crate::game::game_logic_plugin::action::action_systems::fog_update::{
    apply_debug_visibility, fog_update,
};
use crate::game::game_logic_plugin::action::action_systems::gather_entity::gather_entity;
use crate::game::game_logic_plugin::action::action_systems::move_entity::move_entity;
use crate::game::game_logic_plugin::action::action_systems::spawn_unit::spawn_unit;
use crate::game::game_logic_plugin::action::action_systems::upgrade_entity::upgrade_entity;
use crate::game::game_logic_plugin::action::rules::move_rules::MovementCostGrid;
use crate::game::game_logic_plugin::events_resources_structs::action_messages::{Action, Clear};
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::{
    RenderBuildEntity, RenderGatherEntity, RenderMoveEntity,
};
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType, Levels};
use crate::world::game_resources::data::unit::{
    FieldOfView, HelperBasicStats, LevelCost, MovementPoints, UnitLevel,
};
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;
use bevy::ecs::system::SystemParam;
use bevy::math::IVec2;
use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, Res, ResMut};

/// Everything action handlers may touch, bundled into one `SystemParam`.
/// Handlers are plain functions that take `&mut ActionCtx`.
///
/// Queries are merged per component set so they never conflict
/// (e.g. `HelperBasicStats` is `&mut` in build but `&` in gather,
/// `BuildingLevel` is `&mut` in upgrade but `&` in spawn).
#[derive(SystemParam)]
pub struct ActionCtx<'w, 's> {
    // read-only resources
    pub turn_manager: Res<'w, TurnManager>,
    pub config: Res<'w, GameLogicConfig>,
    pub terrain: Res<'w, TerrainMap>,
    pub cost_grid: Res<'w, MovementCostGrid>,
    // mutable resources
    pub players: ResMut<'w, Players>,
    pub units_map: ResMut<'w, UnitsMap>,
    pub buildings_map: ResMut<'w, BuildingMap>,
    pub resource_map: ResMut<'w, ResourceMap>,
    pub fog_map: ResMut<'w, FogMap>,
    // queries
    pub helpers: Query<
        'w,
        's,
        (
            &'static Id,
            &'static PlayerRace,
            &'static mut HelperBasicStats,
        ),
    >,
    pub movers: Query<'w, 's, (&'static Id, &'static mut MovementPoints)>,
    pub buildings: Query<
        'w,
        's,
        (
            &'static Id,
            &'static PlayerRace,
            &'static BuildingType,
            &'static mut BuildingLevel,
            &'static Levels,
        ),
    >,
    pub leveled_units: Query<'w, 's, (&'static Id, &'static mut UnitLevel, &'static LevelCost)>,
    pub sight: Query<'w, 's, (&'static Id, &'static FieldOfView)>,
    // output
    pub commands: Commands<'w, 's>,
    pub render_build: MessageWriter<'w, RenderBuildEntity>,
    pub render_gather: MessageWriter<'w, RenderGatherEntity>,
    pub render_move: MessageWriter<'w, RenderMoveEntity>,
    pub clear: MessageWriter<'w, Clear>,
}

/// Converts a position to map coordinates, `None` if it is outside the map.
pub fn tile(pos: IVec2, size: usize) -> Option<(usize, usize)> {
    let inside = pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < size && (pos.y as usize) < size;
    inside.then_some((pos.x as usize, pos.y as usize))
}

/// Reads every `Action` once, in the order they were sent, and dispatches it.
pub fn action_router(mut actions: MessageReader<Action>, mut ctx: ActionCtx) {
    for action in actions.read().cloned() {
        match action {
            Action::Move {
                unit_pos,
                target_pos,
            } => move_entity(&mut ctx, unit_pos, target_pos),
            Action::Gather { resource_pos } => gather_entity(&mut ctx, resource_pos),
            Action::Build {
                entity_pos,
                building_type,
            } => build_entity(&mut ctx, entity_pos, building_type),
            Action::Upgrade { entity_pos, layer } => upgrade_entity(&mut ctx, entity_pos, layer),
            Action::SpawnUnit {
                city_pos,
                unit_type,
            } => spawn_unit(&mut ctx, city_pos, unit_type),
            Action::RevealMap {
                center,
                reveal_source,
                full,
            } => fog_update(&mut ctx, center, reveal_source, full),
            // Not handled by the router (yet).
            Action::Attack { .. } | Action::Delete { .. } | Action::Info { .. } => {}
        }
    }

    // Debug override goes last, after all reveals of this frame.
    apply_debug_visibility(&mut ctx);
}
