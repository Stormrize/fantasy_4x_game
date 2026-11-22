use bevy::{
    app::{App, Plugin},
    ecs::system::ResMut,
    state::state::{NextState, OnEnter},
};

use crate::{
    app_state::AppState,
    io::startup_loading::{
        fonts_loader_plugin::{FontsLoader, FontsLoadingState},
        localization_loader_plugin::{LocalizationLoader, LocalizationState},
        setting_loader_saver_plugin::PlayerSettings,
        ui_assets_loader_plugin::{RaceLoadingState, RaceManifestsLoader},
    },
};

pub mod fonts_loader_plugin;
pub mod localization_loader_plugin;
pub mod setting_loader_saver_plugin;
pub mod ui_assets_loader_plugin;

pub(super) struct InitialLoader;

impl Plugin for InitialLoader {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerSettings)
            .add_plugins(LocalizationLoader)
            .add_plugins(RaceManifestsLoader)
            .add_plugins(FontsLoader)
            .add_systems(OnEnter(AppState::InitialAssetLoading), start_loading);
    }
}

fn start_loading(
    mut localization_state: ResMut<NextState<LocalizationState>>,
    mut fonts_loading_state: ResMut<NextState<FontsLoadingState>>,
    mut race_loading_state: ResMut<NextState<RaceLoadingState>>,
) {
    localization_state.set(LocalizationState::Loading);
    fonts_loading_state.set(FontsLoadingState::Loading);
    race_loading_state.set(RaceLoadingState::Loading);
}
