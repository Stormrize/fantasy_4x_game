use crate::ui::initial_loading::fonts_loader_plugin::fonts::{
    fonts_loading_ready, start_loading_fonts,
};
use crate::ui::initial_loading::localization_loader_plugin::LocalizationState;
use crate::ui::menus::AppState;
use bevy::app::{App, Update};
use bevy::prelude::{AppExtStates, IntoScheduleConfigs, OnEnter, Plugin, States, in_state};

pub mod fonts;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(super) enum FontsLoadingState {
    #[default]
    Loading,
    Ready,
}

pub(super) struct FontsLoaderPlugin;

impl Plugin for FontsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<FontsLoadingState>()
            .add_systems(OnEnter(AppState::InitialAssetLoading), start_loading_fonts)
            .add_systems(
                Update,
                fonts_loading_ready
                    .run_if(in_state(LocalizationState::Loading))
                    .run_if(in_state(AppState::InitialAssetLoading)),
            );
    }
}
