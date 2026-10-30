use bevy::ecs::query::Without;
use bevy::prelude::{
    BackgroundColor, Color, Commands, Component, Node, PositionType, Query, Res, Text, With,
    default,
};
use bevy::ui::{AlignItems, JustifyContent, Val};

use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::turn_manager::TurnManager;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct FoodText;

#[derive(Component)]
pub struct ScienceText;

#[derive(Component)]
pub struct TurnText;

pub(crate) fn spawn_in_game_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(10.0),
                width: Val::Percent(80.0),
                height: Val::Percent(5.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("Turn: "), TurnText));
            parent.spawn((Text::new("Gold: "), GoldText));
            parent.spawn((Text::new("Food: "), FoodText));
            parent.spawn((Text::new("Science: "), ScienceText));
        });
}

/// Обновляет верхнюю панель ресурсами текущего игрока.
pub(crate) fn update_resources_ui(
    players: Res<Players>,
    turn_manager: Res<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    mut turn_q: Query<
        &mut Text,
        (
            With<TurnText>,
            Without<GoldText>,
            Without<FoodText>,
            Without<ScienceText>,
        ),
    >,
    mut gold_q: Query<
        &mut Text,
        (
            With<GoldText>,
            Without<TurnText>,
            Without<FoodText>,
            Without<ScienceText>,
        ),
    >,
    mut food_q: Query<
        &mut Text,
        (
            With<FoodText>,
            Without<TurnText>,
            Without<GoldText>,
            Without<ScienceText>,
        ),
    >,
    mut science_q: Query<
        &mut Text,
        (
            With<ScienceText>,
            Without<TurnText>,
            Without<GoldText>,
            Without<FoodText>,
        ),
    >,
) {
    let current = turn_manager.current_player;
    let is_local = current.id == game_logic_config.player.id;

    if let Ok(mut text) = turn_q.single_mut() {
        text.0 = if is_local {
            "Turn: You  ".to_string()
        } else {
            format!("Turn: Bot {}   ", current.id.0)
        };
    }

    let (gold, food, science) = players
        .0
        .iter()
        .find(|p| p.id == current.id)
        .map(|p| (p.gold, p.food, p.science))
        .unwrap_or((0, 0, 0));

    if let Ok(mut text) = gold_q.single_mut() {
        text.0 = format!("Gold: {gold}  ");
    }
    if let Ok(mut text) = food_q.single_mut() {
        text.0 = format!("Food: {food}  ");
    }
    if let Ok(mut text) = science_q.single_mut() {
        text.0 = format!("Science: {science}    ");
    }
}
