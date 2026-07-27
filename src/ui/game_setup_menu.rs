use bevy::prelude::{Commands, Node};
use bevy::ui::{FlexDirection, Val};
use bevy::utils::default;

pub fn spawn_game_setup_menu(commands: &mut Commands) {

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,

            ..default()
        }
    ));
}