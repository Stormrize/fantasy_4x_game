//! Bottom-right cluster with the round "end turn" button.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::game::events::action_messages::EndTurn;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::HudPart;
use crate::ui::in_game_ui::theme;
use crate::world::game_configs::GameLogicConfig;
use crate::world::turn_manager::{CombatPhase, TurnManager};

const END_TURN_BUTTON_SIZE: f32 = 84.0;

pub(crate) fn spawn_end_turn_cluster(
    commands: &mut Commands,
    ui: &UiContext,
    language: Language,
    game_logic_config: &GameLogicConfig,
) {
    let cluster = spawn_cluster_root(commands);

    spawn_decorations(commands, cluster);

    let label_column = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(2.0),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(cluster),
        ))
        .id();

    spawn_end_turn_button(commands, label_column, ui, language, game_logic_config);
}

fn spawn_cluster_root(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(24.0),
                bottom: Val::Px(20.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                column_gap: Val::Px(10.0),
                ..default()
            },
            Pickable::IGNORE,
            HudPart::EndTurn,
            GlobalZIndex(10),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id()
}

/// Decorative placeholders for the future tech / units / buildings wheels.
/// Intentionally non-interactive for now.
fn spawn_decorations(commands: &mut Commands, cluster: Entity) {
    let decorations = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                padding: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(cluster),
        ))
        .id();

    theme::spawn_dot(commands, decorations, Color::srgb(0.16, 0.29, 0.58), 24.0);
    theme::spawn_dot(commands, decorations, Color::srgb(0.36, 0.20, 0.38), 24.0);
}

fn spawn_end_turn_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    game_logic_config: &GameLogicConfig,
) {
    let next_turn = commands
        .spawn((
            Node {
                width: Val::Px(END_TURN_BUTTON_SIZE),
                height: Val::Px(END_TURN_BUTTON_SIZE),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(theme::PRIMARY_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_label(commands, next_turn, ui, language, ">", 26.0, theme::TEXT);
    theme::spawn_translation(
        commands,
        next_turn,
        ui,
        language,
        "end_turn",
        8.0,
        theme::MUTED,
    );

    theme::add_hover(
        commands,
        next_turn,
        theme::PRIMARY_BG,
        theme::PRIMARY_BG_HOVER,
    );

    // Ending the turn is only possible outside of combat.
    let local_player = game_logic_config.player;
    commands.entity(next_turn).observe(
        move |mut click: On<PointerClick>,
              mut end_turn: MessageWriter<EndTurn>,
              turn_manager: Res<TurnManager>| {
            click.propagate(false);
            if matches!(turn_manager.combat_phase, CombatPhase::Idle) {
                end_turn.write(EndTurn(local_player));
            }
        },
    );
}
