use crate::game::game_logic_plugin::selection::{
    click_router::route_selection,
    events_resources_structs::{
        components::SelectedEntity,
        messages::AppMessageExt
    },
    manage_selected_entity::manage_selected_entity,
    selection_systems::{
        clear::clear_selection_system,
        cycle::cycle_selection_system,
        new_tile::new_tile_selection_system,
    },
    tile_clicked::click_to_world,
};
use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, in_state};

mod click_router;
pub mod events_resources_structs;
mod manage_selected_entity;
mod selection_systems;
pub mod tile_clicked;

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
                )
                    .chain()
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}
