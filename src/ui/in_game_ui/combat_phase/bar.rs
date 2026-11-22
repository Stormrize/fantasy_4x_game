//! Layout of the bottom combat bar.
//!
//! ```text
//! COMBAT PHASE  status ........  |<  <  >  >|  OK
//! SKILL  ATK 1 2 3 ...........  [Explain] [Simulate]
//! short instructions
//! ```

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::game::game_logic_plugin::combat::integration::ResolveCombat;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{HudPart, HudUiState};
use crate::ui::in_game_ui::theme;

use super::army_nav::spawn_army_nav_buttons;
use super::buttons::{ButtonTone, spawn_square_button};
use super::help_panel::spawn_help_panel;
use super::skills::spawn_skill_buttons;
use super::status::spawn_status_text;

pub(crate) fn spawn_combat_bar(commands: &mut Commands, ui: &UiContext, language: Language) {
    let bar = spawn_bar_root(commands);

    spawn_header_row(commands, bar, ui, language);
    spawn_controls_row(commands, bar, ui, language);

    theme::spawn_wrapped_translation(
        commands,
        bar,
        ui,
        language,
        "combat_instructions",
        9.0,
        theme::MUTED,
    );

    spawn_help_panel(commands, ui, language);
}

fn spawn_bar_root(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            HudPart::CombatBar,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(340.0),
                right: Val::Px(150.0),
                bottom: Val::Px(18.0),
                display: Display::None,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(7.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            GlobalZIndex(12),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id()
}

/// Row 1: phase title, live status and the timeline-style navigation.
fn spawn_header_row(commands: &mut Commands, bar: Entity, ui: &UiContext, language: Language) {
    let header = spawn_row(commands, bar, 10.0);

    theme::spawn_translation(
        commands,
        header,
        ui,
        language,
        "combat_phase",
        17.0,
        theme::GOLD_TEXT,
    );
    spawn_status_text(commands, header, ui, language);
    commands.spawn((theme::spacer(), Pickable::IGNORE, ChildOf(header)));

    spawn_army_nav_buttons(commands, header, ui, language);

    let simulate_short =
        spawn_square_button(commands, header, ui, language, "OK", ButtonTone::Primary);
    attach_resolve(commands, simulate_short);
}

/// Row 2: skill selection and the main actions.
fn spawn_controls_row(commands: &mut Commands, bar: Entity, ui: &UiContext, language: Language) {
    let controls = spawn_row(commands, bar, 6.0);

    theme::spawn_translation(
        commands,
        controls,
        ui,
        language,
        "skill",
        10.0,
        theme::MUTED,
    );
    spawn_skill_buttons(commands, controls, ui, language);

    commands.spawn((theme::spacer(), Pickable::IGNORE, ChildOf(controls)));

    let explain = theme::spawn_button(
        commands,
        controls,
        ui,
        language,
        "explain",
        theme::BUTTON_BG,
        theme::BUTTON_BG_HOVER,
    );
    commands
        .entity(explain)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.combat_help_open = !hud.combat_help_open;
        });

    let simulate = theme::spawn_button(
        commands,
        controls,
        ui,
        language,
        "simulate_step",
        theme::PRIMARY_BG,
        theme::PRIMARY_BG_HOVER,
    );
    attach_resolve(commands, simulate);
}

fn spawn_row(commands: &mut Commands, bar: Entity, gap: f32) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(gap),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(bar),
        ))
        .id()
}

/// Clicking simulates the combat round.
fn attach_resolve(commands: &mut Commands, button: Entity) {
    commands.entity(button).observe(
        |mut click: On<PointerClick>, mut resolve: MessageWriter<ResolveCombat>| {
            click.propagate(false);
            resolve.write(ResolveCombat);
        },
    );
}
