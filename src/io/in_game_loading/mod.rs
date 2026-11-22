use bevy::{
    app::{App, Plugin},
    ecs::system::ResMut,
    state::state::{NextState, OnEnter},
};

use crate::{
    app_state::AppState,
    core_rust::generation::map_generation_plugin::{GameMapGeneration, MapGenerationLoadingState},
    io::in_game_loading::{
        buildings_data_loader_plugin::{BuildingsDataLoader, BuildingsDataLoadingState},
        game_assets_loader_plugin::{GameAssetsLoader, GameAssetsLoadingState},
        units_data_loader_plugin::{UnitsDataLoader, UnitsDataLoadingState},
    },
};

pub mod buildings_data_loader_plugin;
pub mod game_assets_loader_plugin;
pub mod units_data_loader_plugin;

pub(super) struct InGameAssetLoader;

impl Plugin for InGameAssetLoader {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameAssetsLoader)
            .add_plugins(UnitsDataLoader)
            .add_plugins(BuildingsDataLoader)
            .add_plugins(GameMapGeneration)
            .add_systems(OnEnter(AppState::InGameAssetLoading), load_generate);
    }
}

fn load_generate(
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
