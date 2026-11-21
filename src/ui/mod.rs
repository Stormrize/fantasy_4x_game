use bevy::app::App;
use bevy::ecs::system::{Res, SystemParam};
use bevy::prelude::{AppExtStates, Component, Font, Handle, Plugin};

use crate::io::startup_loading::fonts_loader_plugin::Fonts;
use crate::io::startup_loading::localization_loader_plugin::{Language, Localization};
use crate::ui::about_me::AboutMe;
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

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(InitialLoader)
            .add_plugins(MainMenu)
            .add_plugins(Settings)
            .add_plugins(OfflineGameConfig)
            .add_plugins(OnlineGameConfig)
            .add_plugins(InGameAssetLoader)
            .add_plugins(InGameMenu)
            .add_plugins(AboutMe);
    }
}

/// Marks a text entity so its content can be re-resolved from `Localization`
/// when the language changes (`settings_menu::change_tracking`).
#[derive(Component, Clone, Copy)]
pub struct TranslationKey(pub &'static str);

/// Single system parameter that bundles everything text-building UI systems
/// need: the persisted settings (for the current language and, in the settings
/// menu, for writing new values), the localization tables and the per-language
/// fonts.
#[derive(SystemParam)]
pub struct UiContext<'w> {
    pub localization: Res<'w, Localization>,
    pub fonts: Res<'w, Fonts>,
}

impl UiContext<'_> {
    /// Font handle for the currently selected language.
    pub fn font(&self, language: Language) -> Handle<Font> {
        self.fonts.get(language).clone()
    }

    /// Translated string for `key` in the currently selected language.
    pub fn get<'a>(&'a self, lanugage: Language, key: &'a str) -> &'a str {
        self.localization.get(key, lanugage)
    }
}
