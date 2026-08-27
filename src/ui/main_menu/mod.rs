use crate::ui::{
    main_menu::{button_system::button_system, spawn_start_menu::spawn_main_menu},
    menus::AppState,
};
use bevy::app::{App, Update};
use bevy::prelude::{IntoScheduleConfigs, OnEnter, Plugin, in_state};

mod button_system;
mod spawn_start_menu;

pub(super) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)));
    }
}
