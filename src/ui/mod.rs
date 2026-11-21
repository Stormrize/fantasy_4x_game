use bevy::app::App;
use bevy::prelude::{AppExtStates, Component, Plugin};

use crate::ui::in_game_asset_loading::InGameAssetLoader;
use crate::ui::in_game_ui::InGameMenu;
use crate::ui::offline_game_config::OfflineGameConfig;
use crate::ui::online_game_config::OnlineGameConfig;
use crate::ui::settings_menu::Settings;
use crate::ui::{fist_loading_screen::InitialLoader, main_menu::MainMenu, menus::AppState};

mod about_me;
mod colors;
mod fist_loading_screen;
mod in_game_asset_loading;
mod in_game_ui;
mod main_menu;
pub mod menus;
mod offline_game_config;
mod online_game_config;
mod settings_menu;

pub struct UiPlugin;

#[derive(Component)]
pub struct TranslationKey(pub &'static str);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(InitialLoader)
            .add_plugins(MainMenu)
            .add_plugins(Settings)
            .add_plugins(OfflineGameConfig)
            .add_plugins(OnlineGameConfig)
            .add_plugins(InGameAssetLoader)
            .add_plugins(InGameMenu);
    }
}
