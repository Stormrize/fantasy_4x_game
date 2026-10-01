use crate::game::game_logic_plugin::events_resources_structs::AppMessageExt;
use crate::game::game_logic_plugin::events_resources_structs::components::AlreadySelectedTile;
use crate::game::game_logic_plugin::selection::action_systems::move_entity::move_entity;
use crate::game::game_logic_plugin::selection::manage_selected_entity::manage_selected_entity;
use crate::game::game_logic_plugin::selection::{
    click_router::route_selection, tile_clicked::click_to_world,
};
use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, in_state, resource_changed};

mod action_systems;
mod click_router;
mod manage_selected_entity;
pub mod selection_systems;
pub mod tile_clicked;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.register_all_messages()
            .insert_resource(AlreadySelectedTile::default())
            .add_systems(
                Update,
                (
                    click_to_world,
                    route_selection,
                    move_entity,
                    manage_selected_entity.run_if(resource_changed::<AlreadySelectedTile>),
                )
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}
