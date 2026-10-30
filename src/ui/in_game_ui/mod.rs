mod mini_interactive_ui;
mod spawn_in_game_ui;

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, OnEnter, in_state};

use crate::ui::in_game_ui::spawn_in_game_ui::{spawn_in_game_ui, update_resources_ui};
use crate::ui::menus::AppState;

pub(super) struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGameRenderer), spawn_in_game_ui)
            .add_systems(
                Update,
                update_resources_ui.run_if(in_state(AppState::InGameRenderer)),
            );
    }
}
