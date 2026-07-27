use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::menus::AppState;
use bevy::app::App;
use bevy::prelude::{AppExtStates, Plugin};

mod in_game_menu;
mod main_menu;
pub mod menus;
mod settings_menu;
mod game_setup_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_plugins(MainMenuPlugin);
    }
}
