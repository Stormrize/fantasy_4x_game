//! `SystemParam` bundles and query aliases used by the rebuild system.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::game::game_logic_plugin::combat::integration::CombatOrders;
use crate::game::game_logic_plugin::components::AlreadySelectedTile;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{CombatUiState, SidebarBody, SidebarView};
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType, Levels};
use crate::world::game_resources::data::unit::{
    CombatBasicStats, CurrentHp, HelperBasicStats, LevelAbilities, MovementPoints,
    MovementPointsMax, UnitLevel, UnitType,
};
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;

// Query aliases

pub(crate) type CombatUnitsQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Id,
        &'static PlayerRace,
        &'static UnitType,
        &'static UnitLevel,
        &'static CombatBasicStats,
        &'static CurrentHp,
        &'static MovementPoints,
        &'static MovementPointsMax,
        &'static LevelAbilities,
    ),
>;

pub(crate) type HelpersQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Id,
        &'static PlayerRace,
        &'static HelperBasicStats,
        &'static MovementPoints,
        &'static MovementPointsMax,
    ),
>;

pub(crate) type BuildingsQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Id,
        &'static BuildingType,
        &'static BuildingLevel,
        &'static Levels,
    ),
>;

// System parameters

/// All resources read by the sidebar. Grouped to stay below Bevy's system
/// parameter limit and to make the rebuild system signature readable.
#[derive(SystemParam)]
pub(crate) struct SidebarResources<'w> {
    pub ui: UiContext<'w>,
    pub settings: Res<'w, Persistent<SettingsConfig>>,
    pub config: Res<'w, GameLogicConfig>,
    pub players: Res<'w, Players>,
    pub selected: Res<'w, AlreadySelectedTile>,
    pub units_map: Res<'w, UnitsMap>,
    pub buildings_map: Res<'w, BuildingMap>,
    pub resources_map: Res<'w, ResourceMap>,
    pub terrain: Res<'w, TerrainMap>,
    pub fog: Res<'w, FogMap>,
    pub turn_manager: Res<'w, TurnManager>,
    pub orders: Res<'w, CombatOrders>,
    pub combat_ui: Res<'w, CombatUiState>,
    pub view: ResMut<'w, SidebarView>,
}

impl SidebarResources<'_> {
    /// `true` when anything the sidebar displays has changed since the last run.
    pub(crate) fn any_changed(&self) -> bool {
        self.selected.is_changed()
            || self.settings.is_changed()
            || self.players.is_changed()
            || self.units_map.is_changed()
            || self.buildings_map.is_changed()
            || self.resources_map.is_changed()
            || self.terrain.is_changed()
            || self.fog.is_changed()
            || self.turn_manager.is_changed()
            || self.orders.is_changed()
            || self.combat_ui.is_changed()
            || self.view.is_changed()
    }
}

/// Entity queries used to build the sidebar lists.
#[derive(SystemParam)]
pub(crate) struct SidebarQueries<'w, 's> {
    pub body: Query<
        'w,
        's,
        (Entity, &'static ScrollPosition, Option<&'static Children>),
        With<SidebarBody>,
    >,
    pub combat_units: CombatUnitsQuery<'w, 's>,
    pub helpers: HelpersQuery<'w, 's>,
    pub buildings: BuildingsQuery<'w, 's>,
}
