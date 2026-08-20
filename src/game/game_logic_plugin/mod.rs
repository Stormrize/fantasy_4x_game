use bevy::app::{App, Plugin};
use crate::game::game_logic_plugin::selection::SelectionPlugin;

mod action;
pub mod selection;

pub struct GameLogic;

impl Plugin for GameLogic {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectionPlugin);
    }
}
