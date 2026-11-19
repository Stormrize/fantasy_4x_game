use crate::game::events::AppMessageExt;
use crate::game::game_logic_plugin::action::ActionPlugin;
use crate::game::game_logic_plugin::ai::AiPlugin;
use crate::game::game_logic_plugin::combat::CombatPlugin;
use crate::game::game_logic_plugin::selection::SelectionPlugin;
use bevy::app::{App, Plugin};

pub(crate) mod action;
pub(crate) mod ai;
pub(crate) mod combat;
pub(crate) mod components;
pub mod selection;

pub struct GameLogic;

impl Plugin for GameLogic {
    fn build(&self, app: &mut App) {
        app.register_all_messages()
            .add_plugins(SelectionPlugin)
            .add_plugins(ActionPlugin)
            .add_plugins(AiPlugin)
            .add_plugins(CombatPlugin);
    }
}
