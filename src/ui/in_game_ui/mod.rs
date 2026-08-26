mod mini_interactive_ui;
mod spawn_in_game_ui;

use crate::ui::in_game_ui::spawn_in_game_ui::spawn_in_game_ui;
use crate::ui::menus::AppState;
use bevy::app::App;
use bevy::prelude::{OnEnter, Plugin};

pub(super) struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGameRenderer), spawn_in_game_ui);
    }
}
