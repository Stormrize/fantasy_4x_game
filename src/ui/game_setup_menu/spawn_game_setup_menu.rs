use bevy::color::Color;
use bevy::prelude::{default, AlignItems, BackgroundColor, Commands, DespawnOnExit, FlexDirection, JustifyContent, Node, Res, Val};
use crate::ui::menus::AppState;
use crate::world::ui_resources::ui_race_data::UiRacesData;

pub fn spawn_game_setup_menu(mut commands: Commands, ui_race_data: &UiRacesData) {

    let races_data = &ui_race_data.races_data;
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        DespawnOnExit(AppState::GameSetupMenu),
    )).with_children(|parent|{
        for i in 0..races_data.len() {
            parent.spawn((

            ));
        }
    });
}