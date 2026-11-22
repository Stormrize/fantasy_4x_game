//! "Terrain" section: discovered biome types and their movement cost.

use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::SidebarSection;
use crate::ui::in_game_ui::theme;

use super::super::format::{biome_color, biome_key, move_cost_label};
use super::super::model::SidebarModel;
use super::super::widgets::{
    ListRow, spawn_collapsible_header, spawn_list_row, spawn_row_subtitle,
};

pub(crate) fn spawn_terrain_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    model: &SidebarModel,
) {
    let view = model.view;
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();

    spawn_collapsible_header(
        commands,
        section,
        ui,
        language,
        "terrain",
        view.terrain_open,
        SidebarSection::Terrain,
    );

    if !view.terrain_open {
        return;
    }

    if model.discovery.biomes.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_terrain",
            10.0,
            theme::MUTED,
        );
        return;
    }

    let selected_biome = model.selected_biome();

    for entry in &model.discovery.biomes {
        let is_selected = selected_biome == Some(entry.kind);

        // Rows are informational only: no hover, no click.
        let ListRow { row, text } =
            spawn_list_row(commands, section, biome_color(entry.kind), is_selected, false);
        theme::spawn_translation(
            commands,
            text,
            ui,
            language,
            biome_key(entry.kind),
            12.0,
            theme::TEXT,
        );
        spawn_row_subtitle(
            commands,
            text,
            ui,
            language,
            format!("{} {}", entry.tiles, ui.get(language, "tiles")),
        );

        // Movement cost on the right side of the row.
        theme::spawn_label(
            commands,
            row,
            ui,
            language,
            move_cost_label(ui, language, entry.kind),
            10.0,
            theme::MOVE_POINT,
        );
    }
}
