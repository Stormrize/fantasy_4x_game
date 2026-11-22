//! The rebuild system: gathers the state, builds a [`SidebarModel`] and
//! respawns the sidebar body.

use bevy::prelude::*;

use crate::game::game_logic_plugin::components::{AlreadySelectedTile, TileLayer};
use crate::ui::in_game_ui::state::{SidebarView, UnitTab};
use crate::world::game_resources::data::unit::UnitType;

use super::data::UnitIndex;
use super::gather::{gather_buildings, gather_discovery, gather_units};
use super::model::SidebarModel;
use super::params::{SidebarQueries, SidebarResources};
use super::sections::{
    buildings::spawn_buildings_section, enemies::spawn_enemies_section,
    inventory::spawn_inventory_section, orders::spawn_orders_section,
    resources::spawn_resources_section, summary::spawn_summary, terrain::spawn_terrain_section,
    units::spawn_units_section,
};

/// Rebuilds the sidebar body when the game state changed (or on first run).
pub(crate) fn rebuild_sidebar(
    mut commands: Commands,
    mut resources: SidebarResources,
    queries: SidebarQueries,
) {
    let state_changed = resources.any_changed();

    let Ok((body, scroll, children)) = queries.body.single() else {
        return;
    };
    let is_initial = children.is_none_or(|children| children.is_empty());
    if !state_changed && !is_initial {
        return;
    }

    // Clear the body but keep the scroll offset.
    let scroll = scroll.0;
    commands.entity(body).despawn_children();
    commands.entity(body).insert(ScrollPosition(scroll));

    // Gather.
    let units = gather_units(
        &resources.units_map,
        &resources.fog,
        &resources.buildings_map,
        &resources.resources_map,
        &queries.combat_units,
        &queries.helpers,
    );
    let buildings = gather_buildings(&resources.buildings_map, &queries.buildings);
    let discovery = gather_discovery(&resources.terrain, &resources.fog, &resources.resources_map);

    let selection = *resources.selected;
    sync_view(&mut resources.view, selection, &units);

    // Draw.
    let model = SidebarModel::new(&resources, units, buildings, discovery);
    let ui = &resources.ui;
    let language = resources.settings.language;

    spawn_summary(&mut commands, body, ui, language, &model);
    spawn_units_section(&mut commands, body, ui, language, &model);

    if model.in_combat {
        spawn_enemies_section(&mut commands, body, ui, language, &model);
        spawn_orders_section(
            &mut commands,
            body,
            ui,
            language,
            &resources.orders,
            resources.combat_ui.armed_skill,
        );
    }

    spawn_buildings_section(&mut commands, body, ui, language, &model);
    spawn_resources_section(&mut commands, body, ui, language, &model);
    spawn_terrain_section(&mut commands, body, ui, language, &model);

    if let Some(player) = model.local_player {
        spawn_inventory_section(&mut commands, body, ui, language, player);
    }
}

/// Opens the section that matches the newly selected tile and switches the unit
/// tab to the group the selected unit belongs to. Does nothing while the
/// selection is unchanged.
fn sync_view(view: &mut SidebarView, selected: AlreadySelectedTile, units: &UnitIndex) {
    let key = selected.pos.zip(selected.layer);
    if view.last_selection == key {
        return;
    }
    view.last_selection = key;

    let Some((pos, layer)) = key else {
        return;
    };

    match layer {
        TileLayer::Unit => {
            view.units_open = true;
            if let Some(unit) = units.at(pos) {
                view.unit_tab = if unit.kind == UnitType::Helper {
                    UnitTab::Builder
                } else {
                    UnitTab::Combat
                };
            }
        }
        TileLayer::Building => view.buildings_open = true,
        TileLayer::Resource => view.resources_open = true,
        TileLayer::Terrain | TileLayer::Fog => view.terrain_open = true,
    }
}
