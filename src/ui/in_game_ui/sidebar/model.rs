//! [`SidebarModel`]: everything the sections need to draw, built once per rebuild.

use bevy::prelude::*;

use crate::game::game_logic_plugin::components::{AlreadySelectedTile, TileLayer};
use crate::ui::in_game_ui::state::SidebarView;
use crate::world::components::Id;
use crate::world::game_configs::PlayerData;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::resource::ResourceType;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::turn_manager::CombatPhase;

use super::data::{BuildingIndex, BuildingView, Discovery, UnitIndex, UnitView};
use super::grid::pos_in_bounds;
use super::params::SidebarResources;

pub(crate) struct SidebarModel<'a> {
    pub local_id: Id,
    pub local_player: Option<&'a PlayerData>,
    pub selection: AlreadySelectedTile,
    /// Any combat phase except `Idle`.
    pub in_combat: bool,
    /// Combat phase where orders can be queued.
    pub planning: bool,
    pub view: &'a SidebarView,
    pub units: UnitIndex,
    pub buildings: BuildingIndex,
    pub discovery: Discovery,
    resources_map: &'a ResourceMap,
    terrain: &'a TerrainMap,
}

impl<'a> SidebarModel<'a> {
    pub(crate) fn new(
        res: &'a SidebarResources<'_>,
        units: UnitIndex,
        buildings: BuildingIndex,
        discovery: Discovery,
    ) -> Self {
        let local_id = res.config.player.id;
        let phase = &res.turn_manager.combat_phase;

        Self {
            local_id,
            local_player: res.players.0.iter().find(|player| player.id == local_id),
            selection: *res.selected,
            in_combat: !matches!(phase, CombatPhase::Idle),
            planning: matches!(phase, CombatPhase::Planning),
            view: &*res.view,
            units,
            buildings,
            discovery,
            resources_map: &*res.resources_map,
            terrain: &*res.terrain,
        }
    }

    // Collections

    pub(crate) fn own_units(&self) -> impl Iterator<Item = &UnitView> + '_ {
        self.units
            .iter()
            .filter(move |unit| unit.owner == self.local_id)
    }

    /// Enemy units the local player can currently see.
    pub(crate) fn known_enemies(&self) -> impl Iterator<Item = &UnitView> + '_ {
        self.units
            .iter()
            .filter(move |unit| unit.owner != self.local_id && unit.visible)
    }

    pub(crate) fn own_buildings(&self) -> impl Iterator<Item = &BuildingView> + '_ {
        self.buildings
            .iter()
            .filter(move |city| city.owner == self.local_id)
    }

    pub(crate) fn scroll_count(&self) -> usize {
        self.local_player
            .map_or(0, |player| player.inventory.len())
    }

    // Selection

    /// Is `pos` selected on exactly this `layer`?
    pub(crate) fn is_selected(&self, pos: IVec2, layer: TileLayer) -> bool {
        self.selection.pos == Some(pos) && self.selection.layer == Some(layer)
    }

    fn selected_pos(&self, layer: TileLayer) -> Option<IVec2> {
        self.selection.pos.filter(|_| self.selection.layer == Some(layer))
    }

    /// Resource on the selected tile, if the resource layer is selected.
    pub(crate) fn selected_resource(&self) -> Option<&ResourceType> {
        let pos = self.selected_pos(TileLayer::Resource)?;
        if !pos_in_bounds(pos, self.resources_map.size) {
            return None;
        }
        self.resources_map.get(pos.x as usize, pos.y as usize)
    }

    /// Biome of the selected tile, if the terrain layer is selected.
    pub(crate) fn selected_biome(&self) -> Option<BiomeType> {
        let pos = self.selected_pos(TileLayer::Terrain)?;
        if !pos_in_bounds(pos, self.terrain.size) {
            return None;
        }
        Some(self.terrain.get_biome(pos.x as usize, pos.y as usize).biome_type)
    }
}
