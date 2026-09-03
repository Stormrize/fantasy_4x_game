use crate::ui::{
    main_menu::spawn_start_menu::spawn_main_menu,
    menus::AppState,
};
use bevy::app::App;
use bevy::prelude::{OnEnter, Plugin};

mod spawn_start_menu;

pub(super) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
    }
}
