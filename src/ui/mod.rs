use crate::ui::in_game_asset_loading::InGameAssetLoadingPlugin;
use crate::ui::in_game_ui::InGameMenuPlugin;
use crate::ui::initial_loading::InitialLoadingPlugin;
use crate::ui::local_game_setup_menu::GameSetupMenuPlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::menus::AppState;
use bevy::app::App;
use bevy::prelude::{AppExtStates, Plugin};

mod in_game_asset_loading;
mod in_game_ui;
mod initial_loading;
mod local_game_setup_menu;
mod main_menu;
pub mod menus;
mod settings_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(InitialLoadingPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(GameSetupMenuPlugin)
            .add_plugins(InGameAssetLoadingPlugin)
            .add_plugins(InGameMenuPlugin);
    }
}
