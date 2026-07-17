use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoScheduleConfigs, OnEnter, Plugin};
use crate::ui::main_menu::button_system::button_system;
use crate::ui::main_menu::spawn_start_menu::{spawn_main_menu};
use crate::ui::menus::AppState;

mod spawn_start_menu;
mod button_system;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)));
    }}
