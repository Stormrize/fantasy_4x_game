use crate::ui::initial_loading::localization_loader_plugin::public_api::{
    CurrentLanguage, Language,
};
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::SettingsConfig;
use bevy::app::{App, Plugin};
use bevy_persistent::{Persistent, StorageFormat};
use std::path::PathBuf;

pub mod settings_api;
pub mod settings_config;

const CONFIG_FILE_NAME: &str = "settings.ron";

pub(super) struct GameConfigPlugin;

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        let settings_config = Persistent::<SettingsConfig>::builder()
            .name("settings")
            .format(StorageFormat::RonPrettyWithStructNames)
            .path(resolve_config_path())
            .default(SettingsConfig::default())
            .revertible(true)
            .build()
            .expect("Failed to initialize persistent game config");

        let language = settings_config.language;

        app.insert_resource(settings_config)
            .insert_resource(CurrentLanguage(language));
    }
}

fn resolve_config_path() -> PathBuf {
    let config_dir = if cfg!(target_arch = "wasm32") {
        PathBuf::from("/local/settings")
    } else {
        sysdirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("fantasy_4x_game") // note имя поменяется
    };

    config_dir.join(CONFIG_FILE_NAME)
}
