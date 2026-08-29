use crate::ui::local_game_setup_menu::components_resources::{
    AI_DIFFICULTY_VARIANTS, BOT_COUNT_MAX, BOT_COUNT_MIN, GameSetupSelection, MAP_SHAPE_VARIANTS,
    MAP_SIZE_VARIANTS, RaceButton, Slider, SliderHandle, SliderTarget, SliderValueLabel,
    StartButton,
};
use crate::ui::local_game_setup_menu::slider_systems::{slider_handle_left_percent, slider_label};
use crate::ui::menus::AppState;
use crate::world::game_config::PlayerRace;
use crate::world::ui_and_graphic_resources::data::ui_race_data::UiRaceData;
use crate::world::ui_and_graphic_resources::player_registry::RacesUiRegistry;
use bevy::color::{Color, Srgba};
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    AlignItems, BackgroundColor, Button, Commands, DespawnOnExit, FlexDirection, FlexWrap,
    ImageNode, JustifyContent, Node, PositionType, Res, Text, TextColor, TextFont, Val, default,
};
use bevy::ui::{BorderColor, RelativeCursorPosition, UiRect};

pub(super) fn spawn_game_setup_menu(
    mut commands: Commands,
    races_ui_registry: Res<RacesUiRegistry>,
) {
    // сбрасываем выбор при каждом входе в меню
    commands.insert_resource(GameSetupSelection::default());

    let races_data: &HashMap<PlayerRace, UiRaceData> = &races_ui_registry.0;
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
            // ---- Левая колонка: расы ----
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
                    for (race_id, race_data) in races_data {
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
                                border: UiRect::all(Val::Px(3.0)),
                                ..default()
                            },
                            ImageNode::new(race_data.ui_icon.clone()),
                            BorderColor::all(Color::NONE),
                            Button,
                            RaceButton(race_id.clone()),
                        ));
                    }
                });

            // ---- Правая колонка: сложность + старт ----
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
                    // 1. Заголовок "Levels"
                    parent
                        .spawn((Node {
                            width: Val::Percent(80.0),
                            height: Val::Percent(16.0),
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

                    // хелпер, чтобы не копипастить 3 одинаковых блока
                    for (target, title, steps, initial_index) in [
                        (
                            SliderTarget::MapSize,
                            "Map Size",
                            MAP_SIZE_VARIANTS.len(),
                            1,
                        ),
                        (
                            SliderTarget::MapShape,
                            "Map Shape",
                            MAP_SHAPE_VARIANTS.len(),
                            0,
                        ),
                        (
                            SliderTarget::AiDifficulty,
                            "AI Difficulty",
                            AI_DIFFICULTY_VARIANTS.len(),
                            2,
                        ),
                        (
                            SliderTarget::BotCount,
                            "Bots",
                            BOT_COUNT_MAX - BOT_COUNT_MIN + 1,
                            0,
                        ),
                    ] {
                        parent
                            .spawn(Node {
                                width: Val::Percent(80.0),
                                height: Val::Percent(16.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(title),
                                    TextColor(Color::WHITE),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                ));

                                parent
                                    .spawn((
                                        Node {
                                            width: Val::Percent(80.0),
                                            height: Val::Px(10.0),
                                            margin: UiRect::vertical(Val::Px(6.0)),
                                            position_type: PositionType::Relative,
                                            ..default()
                                        },
                                        BackgroundColor(Color::Srgba(Srgba::new(
                                            1.0, 1.0, 1.0, 0.25,
                                        ))),
                                        Button,
                                        RelativeCursorPosition::default(),
                                        Slider {
                                            target,
                                            steps,
                                            current: initial_index,
                                            dragging: false,
                                        },
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Node {
                                                width: Val::Percent(4.0),
                                                height: Val::Px(16.0),
                                                position_type: PositionType::Absolute,
                                                top: Val::Px(-3.0),
                                                left: Val::Percent(slider_handle_left_percent(
                                                    initial_index,
                                                    steps,
                                                )),
                                                ..default()
                                            },
                                            BackgroundColor(Color::WHITE),
                                            SliderHandle,
                                        ));
                                    });

                                parent.spawn((
                                    Text::new(slider_label(target, initial_index)),
                                    TextColor(Color::WHITE),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    SliderValueLabel(target),
                                ));
                            });
                    }

                    // 5. Кнопка "Start"
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(80.0),
                                height: Val::Percent(16.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.1))),
                            BorderColor::all(Color::NONE),
                            Button,
                            StartButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Start"),
                                TextColor(Color::WHITE),
                                TextFont {
                                    font_size: 22.0,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}
