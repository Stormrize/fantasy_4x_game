use crate::ui::initial_loading::localization_loader_plugin::localization::{
    RawLocalization, localization_loading_ready, start_loading_localization,
};
use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{AppExtStates, IntoScheduleConfigs, OnEnter, States, in_state};
use bevy_common_assets::ron::RonAssetPlugin;

mod localization;
pub mod public_api;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(super) enum LocalizationState {
    #[default]
    Loading,
    Ready,
}

pub(super) struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<RawLocalization>::new(&["ron"]))
            .init_state::<LocalizationState>()
            .add_systems(
                OnEnter(AppState::InitialAssetLoading),
                start_loading_localization,
            )
            .add_systems(
                Update,
                localization_loading_ready
                    .run_if(in_state(LocalizationState::Loading))
                    .run_if(in_state(AppState::InitialAssetLoading)),
            );
    }
}
