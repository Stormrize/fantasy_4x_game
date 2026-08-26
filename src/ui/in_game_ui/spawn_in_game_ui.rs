use bevy::prelude::{BackgroundColor, Color, Commands, Node, PositionType, Text, default};
use bevy::ui::{AlignItems, JustifyContent, Val};

struct ManaBarUi;

pub(crate) fn spawn_in_game_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                // Отступ сверху
                top: Val::Px(10.0),
                // Центрирование по горизонтали с учетом ширины 20%
                left: Val::Percent(40.0),
                width: Val::Percent(20.0),
                height: Val::Percent(5.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.56, 0.56, 0.56)),
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("Gold"),));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(10.0),
                width: Val::Percent(20.0),
                height: Val::Percent(5.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.56, 0.56, 0.56)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(50.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(70.0),
                width: Val::Percent(20.0),
                height: Val::Percent(5.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.56, 0.56, 0.56)),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("Weather: Shiny"));
        });
}
