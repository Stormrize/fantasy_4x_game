//! "Inventory" section: the scrolls the local player owns.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;
use crate::world::game_configs::PlayerData;

use super::super::format::{rare_description_key, rare_kind_key};

pub(crate) fn spawn_inventory_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    player: &PlayerData,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    theme::spawn_section_header(commands, section, ui, language, "inventory");

    if player.inventory.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "empty_inventory",
            10.0,
            theme::MUTED,
        );
        return;
    }

    for scroll in &player.inventory {
        let mut row_node = theme::row(7.0);
        row_node.width = Val::Percent(100.0);
        row_node.padding = UiRect::axes(Val::Px(7.0), Val::Px(3.0));
        row_node.border_radius = BorderRadius::all(Val::Px(4.0));
        let row = commands
            .spawn((
                row_node,
                BackgroundColor(theme::SLOT_BG),
                Pickable::IGNORE,
                ChildOf(section),
            ))
            .id();

        theme::spawn_dot(commands, row, theme::GOLD_TEXT, 7.0);

        let text = commands.spawn((theme::grow_column(1.0), ChildOf(row))).id();
        theme::spawn_translation(
            commands,
            text,
            ui,
            language,
            rare_kind_key(scroll.kind),
            11.0,
            theme::TEXT,
        );
        theme::spawn_wrapped_translation(
            commands,
            text,
            ui,
            language,
            rare_description_key(scroll.kind),
            9.0,
            theme::MUTED,
        );
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            format!("{} {}", ui.get(language, "power"), scroll.power),
            9.0,
            theme::MUTED,
        );
    }
}
