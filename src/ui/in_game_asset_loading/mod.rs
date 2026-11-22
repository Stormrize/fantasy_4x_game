use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    Commands, Event, IntoScheduleConfigs, NextState, Res, ResMut, State, in_state,
};

use crate::app_state::AppState;
use crate::core_rust::generation::map_generation_plugin::MapGenerationLoadingState;
use crate::io::in_game_loading::buildings_data_loader_plugin::BuildingsDataLoadingState;
use crate::io::in_game_loading::game_assets_loader_plugin::GameAssetsLoadingState;
use crate::io::in_game_loading::units_data_loader_plugin::UnitsDataLoadingState;

use crate::ui::in_game_asset_loading::build_game_configs::build_game_configs;

mod build_game_configs;

pub(super) struct InGameAssetLoader;

#[derive(Event)]
pub struct BuildConfigs;

impl Plugin for InGameAssetLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            try_enter_game.run_if(in_state(AppState::InGameAssetLoading)),
        )
        .add_observer(build_game_configs);
    }
}

fn try_enter_game(
    map_loading_state: Res<State<MapGenerationLoadingState>>,
    assets_loading_state: Res<State<GameAssetsLoadingState>>,
    units_loading_state: Res<State<UnitsDataLoadingState>>,
    buildings_loading_state: Res<State<BuildingsDataLoadingState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    let map_ready = *map_loading_state == MapGenerationLoadingState::Ready;
    let assets_ready = *assets_loading_state == GameAssetsLoadingState::Ready;
    let units_ready = *units_loading_state == UnitsDataLoadingState::Ready;
    let buildings_ready = *buildings_loading_state == BuildingsDataLoadingState::Ready;

    if map_ready && assets_ready && units_ready && buildings_ready {
        commands.trigger(BuildConfigs);
        next_state.set(AppState::InGameRenderer);
        println!("Set In game renderer");
    }
}
