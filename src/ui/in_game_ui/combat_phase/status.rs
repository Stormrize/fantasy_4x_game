//! Live phase status text in the combat bar header.

use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::CombatStatusText;
use crate::ui::in_game_ui::theme;
use crate::world::turn_manager::{CombatPhase, TurnManager};

pub(super) fn spawn_status_text(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    commands.spawn((
        theme::label(ui, language, "", 11.0, theme::MUTED),
        CombatStatusText,
        ChildOf(parent),
    ));
}

pub(crate) fn update_combat_status(
    ui: UiContext,
    settings: Res<Persistent<SettingsConfig>>,
    turn_manager: Res<TurnManager>,
    mut status: Query<(&mut Text, &mut TextColor), With<CombatStatusText>>,
) {
    let language = settings.language;
    let (message, color) = match &turn_manager.combat_phase {
        CombatPhase::Planning => (
            theme::resolve(&ui, language, "combat_planning"),
            theme::GOLD_TEXT,
        ),
        CombatPhase::Resolving(_) => (
            theme::resolve(&ui, language, "combat_resolving"),
            theme::ENEMY,
        ),
        CombatPhase::Idle => (theme::resolve(&ui, language, "combat_idle"), theme::MUTED),
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
