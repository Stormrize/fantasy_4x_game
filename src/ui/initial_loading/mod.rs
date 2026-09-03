use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, NextState, OnEnter, Res, ResMut, State, in_state};

use crate::ui::initial_loading::player_settings_loader_plugin::GameConfigPlugin;
use crate::ui::initial_loading::ui_assets::{RaceLoadingPlugin, RaceLoadingState};
use crate::ui::initial_loading::{
    fonts_loader_plugin::{FontsLoaderPlugin, FontsLoadingState},
    localization_loader_plugin::{LocalizationPlugin, LocalizationState},
    spawn_loading_screen::spawn_loading_screen,
};

pub(super) mod fonts_loader_plugin;
pub(super) mod localization_loader_plugin;
pub(super) mod player_settings_loader_plugin;
mod spawn_loading_screen;
pub(super) mod ui_assets;

pub(super) struct InitialLoadingPlugin;

impl Plugin for InitialLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameConfigPlugin)
            .add_plugins(LocalizationPlugin)
            .add_plugins(RaceLoadingPlugin)
            .add_plugins(FontsLoaderPlugin)
            .add_systems(OnEnter(AppState::InitialAssetLoading), spawn_loading_screen)
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
        println!("Loaded all handles initial_loading");
        next_state.set(AppState::MainMenu);
    }
}
