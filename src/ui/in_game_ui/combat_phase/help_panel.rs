//! The "how combat works" panel above the combat bar.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::HudPart;
use crate::ui::in_game_ui::theme;

/// One localization key per help paragraph. Add a key here to add a paragraph.
const HELP_KEYS: [&str; 6] = [
    "combat_help_1",
    "combat_help_2",
    "combat_help_3",
    "combat_help_4",
    "combat_help_5",
    "combat_help_6",
];

pub(super) fn spawn_help_panel(commands: &mut Commands, ui: &UiContext, language: Language) {
    let panel = commands
        .spawn((
            HudPart::CombatHelp,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(340.0),
                right: Val::Px(150.0),
                bottom: Val::Px(160.0),
                display: Display::None,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            GlobalZIndex(14),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id();

    theme::spawn_section_header(commands, panel, ui, language, "how_combat_works");

    for key in HELP_KEYS {
        theme::spawn_wrapped_translation(commands, panel, ui, language, key, 10.0, theme::TEXT);
    }
}
