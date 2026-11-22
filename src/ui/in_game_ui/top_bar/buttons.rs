//! Buttons on the right side of the top bar.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::HudUiState;
use crate::ui::in_game_ui::theme;

/// Opens the relations overlay (and closes the pause menu).
pub(super) fn spawn_relations_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    let button = spawn_text_button(commands, parent, ui, language, "relations");

    commands
        .entity(button)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.relations_open = !hud.relations_open;
            hud.burger_open = false;
        });
}

/// Toggles the left information sidebar so the map can be inspected without it.
pub(super) fn spawn_sidebar_toggle(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    let button = spawn_text_button(commands, parent, ui, language, "sidebar_toggle");

    commands
        .entity(button)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.sidebar_open = !hud.sidebar_open;
        });
}

/// Opens the pause menu (and closes the relations overlay).
pub(super) fn spawn_burger_button(commands: &mut Commands, parent: Entity) {
    let button = commands
        .spawn((
            Node {
                width: Val::Px(40.0),
                height: Val::Px(34.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(theme::BUTTON_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    // Three drawn bars instead of a glyph so the menu works with any font.
    for _ in 0..3 {
        commands.spawn((
            Node {
                width: Val::Px(18.0),
                height: Val::Px(2.0),
                border_radius: BorderRadius::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(theme::TEXT),
            Pickable::IGNORE,
            ChildOf(button),
        ));
    }

    theme::add_hover(commands, button, theme::BUTTON_BG, theme::BUTTON_BG_HOVER);

    commands
        .entity(button)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.burger_open = !hud.burger_open;
            hud.relations_open = false;
        });
}

/// Shared look of the labelled top-bar buttons. The caller attaches the click.
fn spawn_text_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
) -> Entity {
    let button = commands
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(7.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme::BUTTON_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_translation(commands, button, ui, language, key, 11.0, theme::TEXT);
    theme::add_hover(commands, button, theme::BUTTON_BG, theme::BUTTON_BG_HOVER);

    button
}
