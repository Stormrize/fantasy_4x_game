//! Ресурсы состояния меню настройки игры.

use crate::world::game_config::MapShape;
use crate::world::game_configs::{AiDifficulty, MapSize};
use bevy::prelude::Resource;

/// "Живые" значения из сайдбара — обновляются сразу при выборе в дропдауне.
/// Читаются при нажатии "Начать игру" в попапе расы.
#[derive(Resource, Debug, Clone)]
pub struct PendingGameSetup {
    pub map_size: MapSize,
    pub map_shape: MapShape,
    pub ai_difficulty: AiDifficulty,
}

impl Default for PendingGameSetup {
    fn default() -> Self {
        Self {
            map_size: MapSize::default(),
            map_shape: MapShape::default(),
            ai_difficulty: AiDifficulty::default(),
        }
    }
}
