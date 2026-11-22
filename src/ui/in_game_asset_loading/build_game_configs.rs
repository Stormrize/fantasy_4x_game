use bevy::ecs::{
    observer::On,
    system::{Commands, Res, ResMut},
};
use bevy_persistent::Persistent;

use crate::{
    io::{
        in_game_loading::{
            buildings_data_loader_plugin::RacesBuildingsData,
            game_assets_loader_plugin::RacesAssetsRegistry,
            units_data_loader_plugin::RacesUnitsData,
        },
        startup_loading::setting_loader_saver_plugin::SettingsConfig,
    },
    ui::in_game_asset_loading::BuildConfigs,
    world::game_configs::{
        CameraConfig, ConfigType, GameLogicConfig, GameModeConfig, PlayerConfig, PlayerData,
        Players, RenderConfig,
    },
};

pub(super) fn build_game_configs(
    _event: On<BuildConfigs>,
    settings: Res<Persistent<SettingsConfig>>,
    config: Res<ConfigType>,
    mut races_assets_data: ResMut<RacesAssetsRegistry>,
    mut units_data: ResMut<RacesUnitsData>,
    mut buildings_data: ResMut<RacesBuildingsData>,
    mut commands: Commands,
) {
    let player_config = config.player_config;
    build_camera_config(player_config, &settings, &mut commands);
    build_render_config(
        player_config,
        &settings,
        &mut races_assets_data,
        &mut commands,
    );
    build_game_logic_config(
        &mut units_data,
        &mut buildings_data,
        player_config,
        &config.game_mode,
        &mut commands,
    );
}

fn build_camera_config(
    player_config: PlayerConfig,
    settings: &Persistent<SettingsConfig>,
    commands: &mut Commands,
) {
    let camera_config = CameraConfig::new(settings, player_config);
    commands.insert_resource(camera_config);
}

fn build_game_logic_config(
    units_data: &mut RacesUnitsData,
    buildings_data: &mut RacesBuildingsData,
    player_config: PlayerConfig,
    game_mode: &GameModeConfig,
    commands: &mut Commands,
) {
    let units_data = std::mem::take(&mut units_data.0);
    let buildings_data = std::mem::take(&mut buildings_data.0);

    match game_mode {
        GameModeConfig::Local(config) => {
            let game_logic_config =
                GameLogicConfig::new(units_data, buildings_data, player_config, config);
            commands.insert_resource(game_logic_config);

            let players = config
                .races
                .iter()
                .map(|(id, race)| PlayerData::new(*id, race.clone()))
                .collect();

            commands.insert_resource(Players(players));
        }
        GameModeConfig::Multiplayer(_) => {
            // await config
        }
    }

    commands.remove_resource::<RacesBuildingsData>();
    commands.remove_resource::<RacesUnitsData>();
}

fn build_render_config(
    player_config: PlayerConfig,
    settings_config: &SettingsConfig,
    races_assets_data: &mut RacesAssetsRegistry,
    commands: &mut Commands,
) {
    let races_assets_registry = std::mem::take(races_assets_data);
    let render_config = RenderConfig::new(player_config, settings_config, races_assets_registry.0);
    commands.insert_resource(render_config);
    commands.remove_resource::<RacesAssetsRegistry>();
}
