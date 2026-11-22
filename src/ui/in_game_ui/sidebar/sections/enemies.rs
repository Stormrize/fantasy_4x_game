//! "Known enemies" section, only shown during combat.

use bevy::prelude::*;

use crate::game::game_logic_plugin::components::TileLayer;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;

use super::super::interactions::{attach_queue_order, attach_select};
use super::super::model::SidebarModel;
use super::units::spawn_unit_row;

/// Visible enemy units. While planning, clicking a row queues an attack order
/// instead of just selecting the tile.
pub(crate) fn spawn_enemies_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    model: &SidebarModel,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    theme::spawn_section_header(commands, section, ui, language, "known_enemies");

    let enemies: Vec<_> = model.known_enemies().collect();

    if enemies.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_visible_enemies",
            10.0,
            theme::MUTED,
        );
        return;
    }

    for unit in enemies {
        let is_selected = model.is_selected(unit.pos, TileLayer::Unit);
        let row = spawn_unit_row(commands, section, ui, language, unit, is_selected);

        if model.planning {
            attach_queue_order(commands, row, unit.pos, unit.tile_content.clone());
        } else {
            attach_select(commands, row, unit.pos, TileLayer::Unit);
        }
    }
}
