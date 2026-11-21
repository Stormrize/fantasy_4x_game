//! Burger (pause) menu: resume, relations, return to the main menu or exit the
//! game. Opened from the top bar or with `Esc`.

use bevy::app::AppExit;
use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{HudPart, HudUiState};
use crate::ui::in_game_ui::theme;

pub(crate) fn spawn_burger_overlay(commands: &mut Commands, ui: &UiContext, language: Language) {
    let overlay = commands
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
        .id();

    let panel = commands
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
        .id();

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

    let resume = menu_button(commands, panel, ui, language, "resume_game");
    commands
        .entity(resume)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.burger_open = false;
        });

    let relations = menu_button(commands, panel, ui, language, "relations");
    commands.entity(relations).observe(
        |mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.burger_open = false;
            hud.relations_open = true;
        },
    );

    let main_menu = menu_button(commands, panel, ui, language, "exit_to_main_menu");
    commands.entity(main_menu).observe(
        |mut click: On<PointerClick>,
         mut hud: ResMut<HudUiState>,
         mut next_state: ResMut<NextState<AppState>>| {
            click.propagate(false);
            hud.burger_open = false;
            next_state.set(AppState::MainMenu);
        },
    );

    let exit = menu_button(commands, panel, ui, language, "exit_game");
    commands.entity(exit).observe(
        |mut click: On<PointerClick>, mut exit: MessageWriter<AppExit>| {
            click.propagate(false);
            exit.write(AppExit::Success);
        },
    );

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

fn menu_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
) -> Entity {
    let button = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(38.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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

    theme::spawn_translation(commands, button, ui, language, key, 13.0, theme::TEXT);
    theme::add_hover(commands, button, theme::BUTTON_BG, theme::BUTTON_BG_HOVER);

    button
}
