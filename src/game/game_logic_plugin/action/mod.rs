use crate::game::game_logic_plugin::action::action_systems::end_turn::end_turn;
use crate::game::game_logic_plugin::action::action_systems::move_entity::move_entity;
use crate::game::game_logic_plugin::action::keyboard_router::route_keyboard_actions;
use crate::game::game_logic_plugin::action::selection_systems::clear::clear_selection_system;
use crate::game::game_logic_plugin::action::selection_systems::cycle::cycle_selection_system;
use crate::game::game_logic_plugin::action::selection_systems::new_tile::new_tile_selection_system;
use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, in_state};

mod action_systems;
mod keyboard_router;
mod rules;
mod selection_systems;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // selection systems
                clear_selection_system,
                new_tile_selection_system,
                cycle_selection_system,
                // action systems
                route_keyboard_actions,
                move_entity,
                end_turn,
            )
                .run_if(in_state(AppState::InGameRenderer)),
        );
    }
}
