use crate::ui::{
    //in_game_asset_loading::InGameAssetLoadingPlugin, in_game_ui::InGameMenuPlugin,
    initial_loading::InitialLoadingPlugin,
    local_game_setup_menu::GameSetupMenuPlugin,
    main_menu::MainMenuPlugin,
    menus::AppState,
};

use crate::ui::in_game_asset_loading::InGameAssetLoadingPlugin;
use crate::ui::settings_menu::SettingsMenuPlugin;
use bevy::app::App;
use bevy::prelude::{AppExtStates, Plugin};

//mod in_game_asset_loading;
mod colors;
mod in_game_asset_loading;
mod in_game_ui;
mod initial_loading;
mod local_game_setup_menu;
mod main_menu;
pub mod menus;
mod settings_menu;
mod update_ui_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(InitialLoadingPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(GameSetupMenuPlugin)
            .add_plugins(InGameAssetLoadingPlugin)
            .add_plugins(SettingsMenuPlugin);
        //.add_plugins(InGameMenuPlugin);
    }
}
