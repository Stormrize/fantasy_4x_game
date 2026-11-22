//! Compact counters at the top of the sidebar.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;

use super::super::model::SidebarModel;
use super::super::widgets::spawn_count_chip;

pub(crate) fn spawn_summary(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    model: &SidebarModel,
) {
    let block = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG_SOFT),
            Pickable::IGNORE,
            ChildOf(body),
        ))
        .id();

    let mut counts_node = theme::row(10.0);
    counts_node.flex_wrap = FlexWrap::Wrap;
    let counts = commands
        .spawn((counts_node, Pickable::IGNORE, ChildOf(block)))
        .id();

    let chips = [
        ("units", model.own_units().count()),
        ("buildings", model.own_buildings().count()),
        ("discovered_biomes", model.discovery.biomes.len()),
        ("discovered_resources", model.discovery.resource_tiles),
        ("inventory", model.scroll_count()),
    ];
    for (key, value) in chips {
        spawn_count_chip(commands, counts, ui, language, key, value);
    }
}
