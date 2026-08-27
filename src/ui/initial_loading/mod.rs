use crate::ui::initial_loading::load_ui_assets::{
    InitialRawRaceDataHandles, check_raw_handles, cleanup_resources, load_races,
    set_new_state_if_all_loaded,
};
use crate::ui::initial_loading::spawn_loading_screen::spawn_loading_screen;
use crate::ui::menus::AppState;
use crate::world::ui_and_graphic_resources::data::ui_race_data::RawRaceData;
use crate::world::ui_and_graphic_resources::player_registry::RacesUiRegistry;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, OnEnter, OnExit, in_state};
use bevy_common_assets::ron::RonAssetPlugin;

pub mod load_ui_assets;
mod spawn_loading_screen;

pub(super) struct InitialLoadingPlugin;

impl Plugin for InitialLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InitialRawRaceDataHandles>()
            .init_resource::<RacesUiRegistry>()
            .add_plugins(RonAssetPlugin::<RawRaceData>::new(&["ron"]))
            .add_systems(
                OnEnter(AppState::InitialAssetLoading),
                (load_races, spawn_loading_screen),
            )
            .add_systems(OnExit(AppState::InitialAssetLoading), cleanup_resources)
            .add_systems(
                Update,
                (check_raw_handles, set_new_state_if_all_loaded)
                    .chain()
                    .run_if(in_state(AppState::InitialAssetLoading)),
            );
    }
}
