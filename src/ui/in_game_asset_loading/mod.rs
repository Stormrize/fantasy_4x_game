use crate::ui::{
    in_game_asset_loading::{
        check_all_loaded::set_new_state_if_all_loaded,
        load_game_assets::{
            RawVisualRaceHandles, check_raw_handles, cleanup_resources, load_assets,
        },
        load_game_map::start_map_generation,
    },
    menus::AppState,
};
use crate::world::ui_and_graphic_resources::{
    data::visual_race_data::RawRaceAssets, player_registry::RacesAssetsRegistry,
};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, OnEnter, OnExit, in_state};
use bevy_common_assets::ron::RonAssetPlugin;

mod check_all_loaded;
mod load_game_assets;
mod load_game_map;

pub(super) struct InGameAssetLoadingPlugin;

impl Plugin for InGameAssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RawVisualRaceHandles>()
            .init_resource::<RacesAssetsRegistry>()
            .add_plugins(RonAssetPlugin::<RawRaceAssets>::new(&["ron"]))
            .add_systems(
                OnEnter(AppState::InGameAssetLoading),
                (load_assets, start_map_generation),
            )
            .add_systems(OnExit(AppState::InGameAssetLoading), cleanup_resources)
            .add_systems(
                Update,
                (check_raw_handles, set_new_state_if_all_loaded)
                    .chain()
                    .run_if(in_state(AppState::InGameAssetLoading)),
            );
    }
}
