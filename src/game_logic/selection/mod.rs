use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoScheduleConfigs};
use crate::game_logic::selection::click_router::route_selection;
use crate::game_logic::selection::events_resources_structs::components::SelectedEntity;
use crate::game_logic::selection::events_resources_structs::messages::AppMessageExt;
use crate::game_logic::selection::manage_selected_entity::manage_selected_entity;
use crate::game_logic::selection::selection_systems::clear::clear_selection_system;
use crate::game_logic::selection::selection_systems::cycle::cycle_selection_system;
use crate::game_logic::selection::selection_systems::new_tile::new_tile_selection_system;
use crate::game_logic::selection::tile_clicked::click_to_world;
use crate::ui::menus::AppState;

mod click_router;
pub mod tile_clicked;
mod selection_systems;
mod manage_selected_entity;
pub mod events_resources_structs;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.register_interactive_messages()
            .insert_resource(SelectedEntity::default())
            .add_systems(
                Update,
                (
                    click_to_world,
                    route_selection,
                    clear_selection_system,
                    new_tile_selection_system,
                    cycle_selection_system,
                    manage_selected_entity,
                ).chain().run_if(in_state(AppState::InGameMenu)),
            );
    }
}