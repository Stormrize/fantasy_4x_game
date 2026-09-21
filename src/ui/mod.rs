use bevy::app::App;
use bevy::prelude::{AppExtStates, Component, Plugin};

use crate::ui::game_setup_menu::GameSetupMenuPlugin;
use crate::ui::in_game_asset_loading::InGameAssetLoadingPlugin;
use crate::ui::settings_menu::SettingsMenuPlugin;
use crate::ui::{
    fist_loading_screen::InitialLoadingPlugin, main_menu::MainMenuPlugin, menus::AppState,
};

mod colors;
mod fist_loading_screen;
mod game_setup_menu;
mod in_game_asset_loading;
mod in_game_ui;
mod main_menu;
pub mod menus;
mod settings_menu;

pub struct UiPlugin;

#[derive(Component)]
pub struct TranslationKey(pub &'static str);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(InitialLoadingPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(SettingsMenuPlugin)
            .add_plugins(GameSetupMenuPlugin)
            .add_plugins(InGameAssetLoadingPlugin);
        //.add_plugins(InGameMenuPlugin);
    }
}
