use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, NextState, OnEnter, Res, ResMut, State, in_state};

use crate::app_state::AppState;
use crate::io::startup_loading::fonts_loader_plugin::FontsLoadingState;
use crate::io::startup_loading::localization_loader_plugin::LocalizationState;
use crate::io::startup_loading::ui_assets_loader_plugin::RaceLoadingState;
use crate::ui::fist_loading_screen::spawn_loading_screen::spawn_loading_screen;

mod spawn_loading_screen;
pub(super) struct InitialLoader;

impl Plugin for InitialLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InitialAssetLoading), spawn_loading_screen)
            .add_systems(
                Update,
                try_enter_main_menu.run_if(in_state(AppState::InitialAssetLoading)),
            );
    }
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
