use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, NextState, OnEnter, Res, ResMut, State, in_state};

use crate::io::startup_loading::fonts_loader_plugin::{FontsLoaderPlugin, FontsLoadingState};
use crate::io::startup_loading::localization_loader_plugin::{
    LocalizationPlugin, LocalizationState,
};
use crate::io::startup_loading::setting_loader_saver_plugin::PlayerSettingsPlugin;
use crate::io::startup_loading::ui_assets_loader_plugin::{RaceLoadingPlugin, RaceLoadingState};
use crate::ui::fist_loading_screen::spawn_loading_screen::spawn_loading_screen;
use crate::ui::menus::AppState;

mod spawn_loading_screen;
pub(super) struct InitialLoadingPlugin;

impl Plugin for InitialLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerSettingsPlugin)
            .add_plugins(LocalizationPlugin)
            .add_plugins(RaceLoadingPlugin)
            .add_plugins(FontsLoaderPlugin)
            .add_systems(
                OnEnter(AppState::InitialAssetLoading),
                (spawn_loading_screen, start_loading),
            )
            .add_systems(
                Update,
                try_enter_main_menu.run_if(in_state(AppState::InitialAssetLoading)),
            );
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

fn try_enter_main_menu(
    race_loading_state: Res<State<RaceLoadingState>>,
    localization_state: Res<State<LocalizationState>>,
    fonts_loading_state: Res<State<FontsLoadingState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let races_ready = *race_loading_state.get() == RaceLoadingState::Ready;
    let localization_ready = *localization_state.get() == LocalizationState::Ready;
    let fonts_ready = *fonts_loading_state.get() == FontsLoadingState::Ready;

    if races_ready && localization_ready && fonts_ready {
        println!("Loaded all handles fist_loading_screen");
        next_state.set(AppState::MainMenu);
    }
}