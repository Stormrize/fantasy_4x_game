use crate::ui::local_game_setup_menu::build_local_game_config::handle_start_button;
use crate::ui::local_game_setup_menu::drag_systems::{
    start_slider_drag, stop_slider_drag, update_dragging_sliders, update_slider_handle_visuals,
    update_slider_labels,
};
use crate::ui::local_game_setup_menu::selection_pick_systems::{
    handle_race_selection, update_race_button_visuals, update_start_button_visuals,
};
use crate::ui::local_game_setup_menu::spawn_game_setup_menu::spawn_game_setup_menu;
use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, OnEnter, in_state};
mod build_local_game_config;
mod components_resources;
mod drag_systems;
mod selection_pick_systems;
mod slider_systems;
mod spawn_game_setup_menu;

pub(super) struct GameSetupMenuPlugin;

impl Plugin for GameSetupMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameSetupMenu), spawn_game_setup_menu)
            .add_systems(
                Update,
                (
                    handle_race_selection,
                    update_race_button_visuals,
                    start_slider_drag,
                    stop_slider_drag,
                    update_dragging_sliders,
                    update_slider_handle_visuals,
                    update_slider_labels,
                    handle_start_button,
                    update_start_button_visuals,
                )
                    .chain()
                    .run_if(in_state(AppState::GameSetupMenu)),
            );
    }
}
