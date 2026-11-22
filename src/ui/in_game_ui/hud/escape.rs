//! `Esc` closes the topmost overlay or opens the pause menu.

use bevy::prelude::*;

use crate::ui::in_game_ui::state::HudUiState;

pub(crate) fn toggle_burger_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hud: ResMut<HudUiState>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    if hud.relations_open {
        hud.relations_open = false;
    } else if hud.combat_help_open {
        hud.combat_help_open = false;
    } else {
        hud.burger_open = !hud.burger_open;
    }
}
