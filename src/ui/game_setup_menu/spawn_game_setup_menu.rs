use crate::ui::menus::AppState;
use crate::world::ui_and_graphic_resources::ui_race_dataset::InitialRacesData;
use bevy::color::{Color, Srgba};
use bevy::prelude::{
    AlignItems, BackgroundColor, Button, Commands, Component, DespawnOnExit, FlexDirection,
    FlexWrap, ImageNode, JustifyContent, Node, Res, Text, TextColor, TextFont, Val, default,
};
use bevy::ui::UiRect;

#[derive(Component)]
pub(super) enum GameSetupMenuButtons {
    StartGame,
}

pub(super) fn spawn_game_setup_menu(mut commands: Commands, races_data: Res<InitialRacesData>) {
    let races_data = &races_data.0;
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            DespawnOnExit(AppState::GameSetupMenu),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(60.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::SpaceAround,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(Srgba::new(0.0, 0.0, 1.0, 1.0))),
                    DespawnOnExit(AppState::GameSetupMenu),
                ))
                .with_children(|parent| {
                    for race in races_data {
                        parent.spawn((
                            Node {
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                },
                                width: Val::Percent(15.0),
                                height: Val::Percent(15.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ImageNode::new(race.1.ui_icon.clone()),
                            Button,
                            GameSetupMenuButtons::StartGame,
                        ));
                    }
                });
            parent
                .spawn((
                    Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::SpaceAround,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(Srgba::new(0.0, 0.5, 0.5, 1.0))),
                    DespawnOnExit(AppState::GameSetupMenu),
                ))
                .with_children(|parent| {
                    // 1. Блок "Levels"
                    parent
                        .spawn((Node {
                            width: Val::Percent(80.0),
                            height: Val::Percent(20.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Levels"),
                                TextColor(Color::WHITE),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));
                        });

                    // 2. Блок "Hard"
                    parent
                        .spawn((Node {
                            width: Val::Percent(80.0),
                            height: Val::Percent(20.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Hard"),
                                TextColor(Color::Srgba(Srgba::new(1.0, 0.0, 0.0, 0.7))),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));
                        });

                    // 3. Блок "Medium"
                    parent
                        .spawn(
                            (Node {
                                width: Val::Percent(80.0),
                                height: Val::Percent(20.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            }),
                        )
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Medium"),
                                TextColor(Color::Srgba(Srgba::new(1.0, 1.0, 0.0, 0.7))),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));
                        });

                    // 4. Блок "Easy"
                    parent
                        .spawn(
                            (Node {
                                width: Val::Percent(80.0),
                                height: Val::Percent(20.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            }),
                        )
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Easy"),
                                TextColor(Color::Srgba(Srgba::new(0.0, 1.0, 0.0, 0.7))),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}
