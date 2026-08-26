use crate::game::game_logic_plugin::selection::SelectionPlugin;
use bevy::app::{App, Plugin};

mod action;
mod events_resources_structs;
pub mod selection;

pub struct GameLogic;

impl Plugin for GameLogic {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectionPlugin);
    }
}
