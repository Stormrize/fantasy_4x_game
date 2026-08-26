use crate::ui::local_game_setup_menu::button_system::{build_game_config, button_system};
use crate::ui::local_game_setup_menu::spawn_game_setup_menu::spawn_game_setup_menu;
use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::OnEnter;

mod button_system;
mod spawn_game_setup_menu;

pub(super) struct GameSetupMenuPlugin;

impl Plugin for GameSetupMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::GameSetupMenu),
            (spawn_game_setup_menu, build_game_config),
        )
        .add_systems(Update, button_system);
    }
}
