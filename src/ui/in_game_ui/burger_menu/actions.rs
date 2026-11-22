//! The pause menu buttons and what each of them does.

use bevy::app::AppExit;
use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::HudUiState;
use crate::ui::in_game_ui::theme;

/// Spawns all menu buttons, top to bottom.
pub(super) fn spawn_menu_buttons(
    commands: &mut Commands,
    panel: Entity,
    ui: &UiContext,
    language: Language,
) {
    spawn_resume_button(commands, panel, ui, language);
    spawn_relations_button(commands, panel, ui, language);
    spawn_main_menu_button(commands, panel, ui, language);
    spawn_exit_button(commands, panel, ui, language);
}

fn spawn_resume_button(commands: &mut Commands, panel: Entity, ui: &UiContext, language: Language) {
    let button = menu_button(commands, panel, ui, language, "resume_game");
    commands
        .entity(button)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.burger_open = false;
        });
}

fn spawn_relations_button(
    commands: &mut Commands,
    panel: Entity,
    ui: &UiContext,
    language: Language,
) {
    let button = menu_button(commands, panel, ui, language, "relations");
    commands
        .entity(button)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.burger_open = false;
            hud.relations_open = true;
        });
}

fn spawn_main_menu_button(
    commands: &mut Commands,
    panel: Entity,
    ui: &UiContext,
    language: Language,
) {
    let button = menu_button(commands, panel, ui, language, "exit_to_main_menu");
    commands.entity(button).observe(
        |mut click: On<PointerClick>,
         mut hud: ResMut<HudUiState>,
         mut next_state: ResMut<NextState<AppState>>| {
            click.propagate(false);
            hud.burger_open = false;
            next_state.set(AppState::MainMenu);
        },
    );
}

fn spawn_exit_button(commands: &mut Commands, panel: Entity, ui: &UiContext, language: Language) {
    let button = menu_button(commands, panel, ui, language, "exit_game");
    commands.entity(button).observe(
        |mut click: On<PointerClick>, mut exit: MessageWriter<AppExit>| {
            click.propagate(false);
            exit.write(AppExit::Success);
        },
    );
}

/// Shared look of a menu button. The caller attaches the click behavior.
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
