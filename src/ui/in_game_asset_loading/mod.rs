mod loading_screen;

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    AppExtStates, Commands, IntoScheduleConfigs, NextState, OnEnter, Res, ResMut, State, States,
    in_state,
};
use bevy_persistent::Persistent;

use crate::core_rust::generation::map_generation_plugin::{
    GameMapGenerationPlugin, MapGenerationLoadingState,
};
use crate::io::in_game_loading::buildings_data_loader_plugin::{BuildingsDataLoadingPlugin, BuildingsDataLoadingState, RacesBuildingsData};
use crate::io::in_game_loading::game_assets_loader_plugin::{
    GameAssetsLoadingPlugin, GameAssetsLoadingState, RacesAssetsRegistry,
};
use crate::io::in_game_loading::units_data_loader_plugin::{RacesUnitsData, UnitsDataLoadingPlugin, UnitsDataLoadingState};
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::menus::AppState;
use crate::world::game_configs::{CameraConfig, ConfigType, GameLogicConfig, PlayerConfig, RenderConfig};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum InGameLoading {
    #[default]
    Io,
    Configs,
}

pub(super) struct InGameAssetLoadingPlugin;

impl Plugin for InGameAssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InGameLoading>()
            .add_plugins(GameAssetsLoadingPlugin)
            .add_plugins(UnitsDataLoadingPlugin)
            .add_plugins(BuildingsDataLoadingPlugin)
            .add_plugins(GameMapGenerationPlugin)
            .add_systems(
                OnEnter(AppState::InGameAssetLoading),
                (reset_loading_state, start_loading_generating),
            )
            .add_systems(OnEnter(InGameLoading::Configs), build_game_configs)
            .add_systems(
                Update,
                (
                    advance_to_configs.run_if(in_state(InGameLoading::Io)),
                    try_enter_game.run_if(in_state(AppState::InGameAssetLoading)),
                ),
            );
    }
}

fn reset_loading_state(mut next_state: ResMut<NextState<InGameLoading>>) {
    next_state.set(InGameLoading::Io);
}

fn start_loading_generating(
    mut units_loading_state: ResMut<NextState<UnitsDataLoadingState>>,
    mut buildings_loading_state: ResMut<NextState<BuildingsDataLoadingState>>,
    mut assets_loading_state: ResMut<NextState<GameAssetsLoadingState>>,
    mut map_generation_loading_state: ResMut<NextState<MapGenerationLoadingState>>,
) {
    units_loading_state.set(UnitsDataLoadingState::Loading);
    buildings_loading_state.set(BuildingsDataLoadingState::Loading);
    assets_loading_state.set(GameAssetsLoadingState::Loading);
    map_generation_loading_state.set(MapGenerationLoadingState::Loading);
}

fn advance_to_configs(
    units_loading_state: Res<State<UnitsDataLoadingState>>,
    assets_loading_state: Res<State<GameAssetsLoadingState>>,
    buildings_loading_state: Res<State<BuildingsDataLoadingState>>,
    mut next_state: ResMut<NextState<InGameLoading>>,
) {
    let io_done = *units_loading_state == UnitsDataLoadingState::Ready
        && *assets_loading_state == GameAssetsLoadingState::Ready
        && *buildings_loading_state == BuildingsDataLoadingState::Ready;

    if io_done {
        next_state.set(InGameLoading::Configs);
    }
}

fn try_enter_game(
    map_loading_state: Res<State<MapGenerationLoadingState>>,
    assets_loading_state: Res<State<GameAssetsLoadingState>>,
    units_loading_state: Res<State<UnitsDataLoadingState>>,
    buildings_loading_state: Res<State<BuildingsDataLoadingState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let map_ready = *map_loading_state.get() == MapGenerationLoadingState::Ready;
    let assets_ready = *assets_loading_state.get() == GameAssetsLoadingState::Ready;
    let units_ready = *units_loading_state.get() == UnitsDataLoadingState::Ready;
    let buildings_ready = *buildings_loading_state.get() == BuildingsDataLoadingState::Ready;

    if map_ready && assets_ready && units_ready && buildings_ready {
        next_state.set(AppState::InGameRenderer);
    }
}

fn build_game_configs(
    settings: Res<Persistent<SettingsConfig>>,
    config: Res<ConfigType>,
    mut races_assets_data: ResMut<RacesAssetsRegistry>,
    mut units_data: ResMut<RacesUnitsData>,
    mut buildings_data: ResMut<RacesBuildingsData>,
    mut commands: Commands,
) {
    let player_config = &config.player_config;
    build_camera_config(player_config, &settings, &mut commands);
    build_render_config(
        player_config,
        &settings,
        &mut races_assets_data,
        &mut commands,
    );
    build_game_logic_config(&mut units_data, &mut buildings_data, player_config, &mut commands);
}

fn build_camera_config(
    player_config: &PlayerConfig,
    settings: &Persistent<SettingsConfig>,
    commands: &mut Commands,
) {
    let camera_config = CameraConfig::new(settings, player_config);
    commands.insert_resource(camera_config);
}

fn build_game_logic_config(
    units_data: &mut RacesUnitsData,
    buildings_data: &mut RacesBuildingsData,
    player_config: &PlayerConfig,
    commands: &mut Commands,
) {
    let units_data = std::mem::take(&mut units_data.0);
    let buildings_data = std::mem::take(&mut buildings_data.0);
    let game_logic_config = GameLogicConfig::new(units_data, buildings_data, player_config);
    commands.insert_resource(game_logic_config);
    commands.remove_resource::<RacesBuildingsData>();
    commands.remove_resource::<RacesUnitsData>();
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
