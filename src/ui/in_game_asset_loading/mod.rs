mod loading_screen;

use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    Commands, IntoScheduleConfigs, NextState, OnEnter, Res, ResMut, State, in_state,
};
use bevy_persistent::Persistent;

// use crate::io::in_game_loading::map_generation_plugin::{
//     GameMapGenerationPlugin, MapGenerationLoadingState,
// };
use crate::io::in_game_loading::buildings_data_loader_plugin::BuildingsDataLoadingState;
use crate::io::in_game_loading::game_assets_loader_plugin::{
    GameAssetsLoadingPlugin, GameAssetsLoadingState, RacesAssetsRegistry,
};
use crate::io::in_game_loading::units_data_loader_plugin::{
    RawRacesUnitsData, UnitsData, UnitsDataLoadingPlugin, UnitsDataLoadingState,
};
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::menus::AppState;
use crate::world::game_configs::{
    CameraConfig, ConfigType, GenerationConfig, LocalPlayerConfig, PlayerConfig, RenderConfig,
};

pub(super) struct InGameAssetLoadingPlugin;

impl Plugin for InGameAssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameAssetsLoadingPlugin)
            .add_plugins(UnitsDataLoadingPlugin)
            //.add_plugins(GameMapGenerationPlugin)
            .add_systems(OnEnter(AppState::InGameAssetLoading), start_loading)
            .add_systems(
                Update,
                (try_enter_game, build_game_configs).run_if(in_state(AppState::InGameAssetLoading)),
            );
    }
}

fn start_loading(
    mut units_loading_state: ResMut<NextState<UnitsDataLoadingState>>,
    mut buildings_loading_state: ResMut<NextState<BuildingsDataLoadingState>>,
    mut assets_loading_state: ResMut<NextState<GameAssetsLoadingState>>,
) {
    units_loading_state.set(UnitsDataLoadingState::Loading);
    buildings_loading_state.set(BuildingsDataLoadingState::Loading);
    assets_loading_state.set(GameAssetsLoadingState::Loading);
}

fn try_enter_game(
    //game_map_loading_state: Res<State<MapGenerationLoadingState>>,
    assets_loading_state: Res<State<GameAssetsLoadingState>>,
    units_loading_state: Res<State<UnitsDataLoadingState>>,
    buildings_loading_state: Res<State<BuildingsDataLoadingState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    //let map_ready = *game_map_loading_state.get() == MapGenerationLoadingState::Ready;
    let assets_ready = *assets_loading_state == GameAssetsLoadingState::Ready;
    let units_ready = *units_loading_state == UnitsDataLoadingState::Ready;
    let buildings_ready = *buildings_loading_state == BuildingsDataLoadingState::Ready;
    if
    /*map_ready &&*/
    assets_ready && units_ready && buildings_ready {
        next_state.set(AppState::InGameRenderer);
    }
}

//---------------------------------------------------------------------------------------------------------------------------//

fn build_game_configs(
    settings: Res<Persistent<SettingsConfig>>,
    config: Res<ConfigType>,
    mut commands: Commands,
    raw_races_units_data: Res<RawRacesUnitsData>,
    mut raw_units_data: ResMut<Assets<UnitsData>>,
    //mut next_map_generation_state: ResMut<NextState<MapGenerationLoadingState>>,
    units_generation_state: Res<State<UnitsDataLoadingState>>,
    mut races_assets_data: ResMut<RacesAssetsRegistry>,
) {
    if *units_generation_state.get() == UnitsDataLoadingState::Loading {
        return;
    }

    match &*config {
        ConfigType::Local(local_player_config) => {
            build_camera_config(&local_player_config.player_config, &settings, &mut commands);
            build_generation_config(
                local_player_config,
                &raw_races_units_data,
                &mut raw_units_data,
                &mut commands,
                //&mut next_map_generation_state,
            );
            build_render_config(
                &local_player_config.player_config,
                &settings,
                &mut races_assets_data,
                &mut commands,
            )
        }
        ConfigType::Multiplayer(online_player_config) => {
            build_camera_config(
                &online_player_config.player_config,
                &settings,
                &mut commands,
            );
            // get_map_from_server
            build_render_config(
                &online_player_config.player_config,
                &settings,
                &mut races_assets_data,
                &mut commands,
            )
        }
    }
}

fn build_camera_config(
    player_config: &PlayerConfig,
    settings: &Persistent<SettingsConfig>,
    commands: &mut Commands,
) {
    let camera_config = CameraConfig::new(settings, player_config);
    commands.insert_resource(camera_config);
}

fn build_generation_config(
    local_player_config: &LocalPlayerConfig,
    raw_races_units_data: &RawRacesUnitsData,
    raw_units_data: &mut Assets<UnitsData>,
    commands: &mut Commands,
    //map_generation_state: &mut NextState<MapGenerationLoadingState>,
) {
    let mut races_units_data = HashMap::with_capacity(raw_units_data.len());
    for (player_race, units_data) in &raw_races_units_data.0 {
        if let Some(units_data) = raw_units_data.remove(units_data) {
            races_units_data.insert(player_race.clone(), units_data);
        }
    }
    let generation_config = GenerationConfig::new(local_player_config, races_units_data);
    commands.insert_resource(generation_config);
    //map_generation_state.set(MapGenerationLoadingState::Loading);
}

fn build_render_config(
    player_config: &PlayerConfig,
    settings_config: &SettingsConfig,
    races_assets_data: &mut RacesAssetsRegistry,
    commands: &mut Commands,
) {
    let races_assets_registry = std::mem::take(races_assets_data);

    let render_config = RenderConfig::new(player_config, settings_config, races_assets_registry.0);
    commands.insert_resource(render_config);
    commands.remove_resource::<RacesAssetsRegistry>();
}
