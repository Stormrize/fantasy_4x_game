use bevy::app::App;
use bevy::camera::{Camera, Camera2d, Camera3d};
use bevy::color::Color;
use bevy::prelude::{Button, Commands, Component, DespawnOnExit, KeyCode, MessageReader, Query, Res, ResMut, Text, TextColor, TextFont, Time, Transform, With};
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val};
use bevy::utils::default;
use crate::ui::menus::AppState;

#[derive(Component)]
pub enum MainMenuButtons {
    StartButton,
    ResumeButton,
    ExitButton,
}

#[derive(Component)]
pub struct WierdCamera;

pub fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        DespawnOnExit(AppState::MainMenu)
    ));
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
        DespawnOnExit(AppState::MainMenu),
    )).with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(10.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            MainMenuButtons::StartButton,
        )).with_children(|button_parent| {
            button_parent.spawn((
                Text::new("Start Game"),
                TextFont {font_size: 20.0, ..default()},
                TextColor(Color::WHITE),
            ));
        });
        parent.spawn((
            Button,
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(10.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            MainMenuButtons::ResumeButton,
        )).with_children(|button_parent| {
            button_parent.spawn((
                Text::new("Resume Game"),
                TextFont {font_size: 20.0, ..default()},
                TextColor(Color::WHITE),
            ));
        });;
        parent.spawn((
            Button,
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(10.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            MainMenuButtons::ExitButton,
        )).with_children(|button_parent| {
            button_parent.spawn((
                Text::new("Exit Game"),
                TextFont {font_size: 20.0, ..default()},
                TextColor(Color::WHITE),
            ));
        });
    });
}