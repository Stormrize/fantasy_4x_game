use crate::ui::in_game_asset_loading::game_assets_loader_plugin::{
    GameAssetsLoadingPlugin, GameAssetsLoadingState,
};

use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, NextState, Res, ResMut, State, in_state};

mod game_assets_loader_plugin;
mod map_generation_plugin;

pub(super) struct InGameAssetLoadingPlugin;

impl Plugin for InGameAssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameAssetsLoadingPlugin)
            //.add_plugins(GameMapGenerationPlugin)
            .add_systems(
                Update,
                try_enter_game.run_if(in_state(AppState::InGameAssetLoading)),
            );
    }
}

fn try_enter_game(
    //game_map_loading_state: Res<State<MapGenerationLoadingState>>,
    game_assets_loading_state: Res<State<GameAssetsLoadingState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    //let map_ready = *game_map_loading_state.get() == MapGenerationLoadingState::Ready;
    let assets_ready = *game_assets_loading_state == GameAssetsLoadingState::Ready;

    if
    /*map_ready &&*/
    assets_ready {
        next_state.set(AppState::InGameRenderer);
    }
}
