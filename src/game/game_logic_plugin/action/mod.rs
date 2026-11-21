use crate::game::game_logic_plugin::action::action_systems::action_router;
use crate::game::game_logic_plugin::action::action_systems::collect_income::collect_income;
use crate::game::game_logic_plugin::action::action_systems::end_turn::end_turn;
use crate::game::game_logic_plugin::action::action_systems::reset_movement_points::reset_movement_points;
use crate::game::game_logic_plugin::action::keyboard_router::route_keyboard_actions;
use crate::game::game_logic_plugin::action::rules::move_rules::{
    insert_movement_cost_grid, maintain_movement_cost_grid,
};
use crate::game::game_logic_plugin::action::selection_systems::clear::clear_selection_system;
use crate::game::game_logic_plugin::action::selection_systems::cycle::cycle_selection_system;
use crate::game::game_logic_plugin::action::selection_systems::new_tile::new_tile_selection_system;
use crate::ui::menus::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, OnEnter, in_state};

pub(crate) mod action_systems;
mod keyboard_router;
pub(crate) mod rules;
mod selection_systems;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGameRenderer), insert_movement_cost_grid)
            .add_systems(
                Update,
                (
                    // selection systems
                    clear_selection_system,
                    new_tile_selection_system,
                    cycle_selection_system,
                    // keyboard router
                    route_keyboard_actions,
                    // take action from keyboard or ui and do something
                    action_router,
                    // end turn
                    collect_income,
                    end_turn,
                    reset_movement_points,
                )
                    .chain()
                    .run_if(in_state(AppState::InGameRenderer)),
            )
            .add_systems(
                Update,
                maintain_movement_cost_grid
                    .before(action_router)
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}
