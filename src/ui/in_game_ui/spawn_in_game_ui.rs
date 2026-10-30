use bevy::prelude::{
    BackgroundColor, Color, Commands, Component, Node, PositionType, Query, Res, Text, default,
};
use bevy::ui::{AlignItems, JustifyContent, Val};

use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::turn_manager::TurnManager;

#[derive(Component, Clone, Copy)]
pub enum ResourceTextType {
    Turn,
    Gold,
    Food,
    Science,
}

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
            parent.spawn((Text::new("Turn: "), ResourceTextType::Turn));
            parent.spawn((Text::new("Gold: "), ResourceTextType::Gold));
            parent.spawn((Text::new("Food: "), ResourceTextType::Food));
            parent.spawn((Text::new("Science: "), ResourceTextType::Science));
        });
}

/// Обновляет верхнюю панель ресурсами текущего игрока.
pub(crate) fn update_resources_ui(
    players: Res<Players>,
    turn_manager: Res<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    mut text_q: Query<(&mut Text, &ResourceTextType)>,
) {
    let current = turn_manager.current_player;
    let is_local = current.id == game_logic_config.player.id;

    let (gold, food, science) = players
        .0
        .iter()
        .find(|p| p.id == current.id)
        .map(|p| (p.gold, p.food, p.science))
        .unwrap_or((0, 0, 0));

    // Итерируемся по элементам интерфейса и обновляем текст по типу
    for (mut text, text_type) in &mut text_q {
        match text_type {
            ResourceTextType::Turn => {
                text.0 = if is_local {
                    "Turn: You  ".to_string()
                } else {
                    format!("Turn: Bot {}   ", current.id.0)
                };
            }
            ResourceTextType::Gold => text.0 = format!("Gold: {gold}  "),
            ResourceTextType::Food => text.0 = format!("Food: {food}  "),
            ResourceTextType::Science => text.0 = format!("Science: {science}    "),
        }
    }
}
