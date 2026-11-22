//! Small square button used in the combat bar header.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;

#[derive(Clone, Copy)]
pub(super) enum ButtonTone {
    Normal,
    /// Accent color for the main action.
    Primary,
}

impl ButtonTone {
    fn colors(self) -> (Color, Color) {
        match self {
            Self::Normal => (theme::BUTTON_BG, theme::BUTTON_BG_HOVER),
            Self::Primary => (theme::PRIMARY_BG, theme::PRIMARY_BG_HOVER),
        }
    }
}

/// The caller attaches the click behavior.
pub(super) fn spawn_square_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    label: &str,
    tone: ButtonTone,
) -> Entity {
    let (bg, hover) = tone.colors();

    let button = commands
        .spawn((
            Node {
                width: Val::Px(32.0),
                height: Val::Px(26.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(bg),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_label(commands, button, ui, language, label, 12.0, theme::TEXT);
    theme::add_hover(commands, button, bg, hover);

    button
}
