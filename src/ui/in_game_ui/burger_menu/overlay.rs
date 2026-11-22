//! Layout of the pause menu: fullscreen dim, centered panel, title, buttons.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::HudPart;
use crate::ui::in_game_ui::theme;

use super::actions::spawn_menu_buttons;

pub(crate) fn spawn_burger_overlay(commands: &mut Commands, ui: &UiContext, language: Language) {
    let overlay = spawn_overlay_root(commands);
    let panel = spawn_panel(commands, overlay);

    theme::spawn_translation(
        commands,
        panel,
        ui,
        language,
        "game_menu_title",
        24.0,
        theme::GOLD_TEXT,
    );
    theme::spawn_wrapped_translation(
        commands,
        panel,
        ui,
        language,
        "game_menu_subtitle",
        10.0,
        theme::MUTED,
    );

    spawn_divider(commands, panel);
    spawn_menu_buttons(commands, panel, ui, language);

    theme::spawn_wrapped_translation(
        commands,
        panel,
        ui,
        language,
        "esc_closes_menu",
        9.0,
        theme::MUTED,
    );
}

/// Fullscreen dim that centers the panel. Hidden until the menu is opened.
fn spawn_overlay_root(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            HudPart::Burger,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                display: Display::None,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme::OVERLAY_BG),
            Pickable::default(),
            GlobalZIndex(30),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id()
}

fn spawn_panel(commands: &mut Commands, overlay: Entity) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Px(360.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(22.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            ChildOf(overlay),
        ))
        .id()
}

fn spawn_divider(commands: &mut Commands, panel: Entity) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: UiRect::vertical(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(theme::BORDER_SOFT),
        Pickable::IGNORE,
        ChildOf(panel),
    ));
}
