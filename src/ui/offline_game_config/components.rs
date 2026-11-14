//! Ресурсы состояния меню настройки игры.

use bevy::prelude::Resource;

use crate::world::components::{AiDifficulty, MapShape, MapSize};

/// "Живые" значения из сайдбара — обновляются сразу при выборе в дропдауне.
/// Читаются при нажатии "Начать игру" в попапе расы.
#[derive(Resource, Debug, Clone, Default)]
pub struct PendingGameSetup {
    pub map_size: MapSize,
    pub map_shape: MapShape,
    pub ai_difficulty: AiDifficulty,
    pub bots_amount: usize,
}
