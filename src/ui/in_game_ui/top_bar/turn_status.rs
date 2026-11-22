//! Turn indicator chip: "YOUR TURN", "TURN: <RACE>" or "COMBAT".

use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::TurnStatusText;
use crate::ui::in_game_ui::theme;
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::turn_manager::{CombatPhase, TurnManager};

// Spawning

pub(super) fn spawn_turn_chip(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    let chip = commands
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme::SLOT_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::IGNORE,
            ChildOf(parent),
        ))
        .id();

    commands.spawn((
        theme::label(ui, language, "", 10.0, theme::MUTED),
        TurnStatusText,
        ChildOf(chip),
    ));
}

// Live update

pub(crate) fn update_turn_status(
    ui: UiContext,
    settings: Res<Persistent<SettingsConfig>>,
    game_logic_config: Res<GameLogicConfig>,
    players: Res<Players>,
    turn_manager: Res<TurnManager>,
    mut status: Query<(&mut Text, &mut TextColor), With<TurnStatusText>>,
) {
    let language = settings.language;
    let local_id = game_logic_config.player.id;
    let current = turn_manager.current_player;

    let (message, color) = if !matches!(turn_manager.combat_phase, CombatPhase::Idle) {
        (
            theme::resolve(&ui, language, "turn_combat").to_uppercase(),
            theme::ENEMY,
        )
    } else if current.id == local_id {
        (
            theme::resolve(&ui, language, "turn_your").to_uppercase(),
            theme::GOLD_TEXT,
        )
    } else {
        let race = players
            .0
            .iter()
            .find(|player| player.id == current.id)
            .map(|player| theme::race_display_name(&player.player_race.0))
            .unwrap_or_else(|| theme::resolve(&ui, language, "enemy"));
        (
            format!(
                "{}: {}",
                theme::resolve(&ui, language, "turn").to_uppercase(),
                race.to_uppercase()
            ),
            theme::MUTED,
        )
    };

    for (mut text, mut text_color) in &mut status {
        if text.0 != message {
            text.0 = message.clone();
        }
        if text_color.0 != color {
            text_color.0 = color;
        }
    }
}
