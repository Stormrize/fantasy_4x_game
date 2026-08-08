use bevy::app::{App, Plugin};
use crate::game_logic::selection::SelectionPlugin;

pub mod selection;
mod action;

pub struct GameLogic;

impl Plugin for GameLogic {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectionPlugin);
    }
}