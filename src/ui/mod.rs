use bevy::app::App;
use bevy::prelude::{AppExtStates, Plugin};
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::menus::AppState;

pub mod menus;
mod in_game_menu;
mod main_menu;
mod settings_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(MainMenuPlugin);
    }
}