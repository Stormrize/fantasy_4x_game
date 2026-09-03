use crate::world::game_config::{MapShape, PlayerRace};
use crate::world::game_configs::{AiDifficulty, MapSize};
use bevy::prelude::{Component, Resource};

// note не забывать менять константы
pub(super) const MAP_SIZE_VARIANTS: [MapSize; 5] = [
    MapSize::Tiny,
    MapSize::Small,
    MapSize::Medium,
    MapSize::Big,
    MapSize::Large,
];
pub(super) const MAP_SHAPE_VARIANTS: [MapShape; 2] = [MapShape::Jagged, MapShape::Round];
pub(super) const AI_DIFFICULTY_VARIANTS: [AiDifficulty; 4] = [
    AiDifficulty::Easy,
    AiDifficulty::Normal,
    AiDifficulty::Hard,
    AiDifficulty::Crazy,
];
pub(super) const BOT_COUNT_MIN: usize = 1;
pub(super) const BOT_COUNT_MAX: usize = 7;

#[derive(Resource)]
pub(super) struct GameSetupSelection {
    pub race: Option<PlayerRace>,
    pub map_size: MapSize,
    pub map_shape: MapShape,
    pub ai_difficulty: AiDifficulty,
    pub bot_count: usize,
}
#[derive(Component)]
pub(super) struct RaceButton(pub(super) PlayerRace);

#[derive(Component)]
pub(super) struct DifficultyButton(pub(super) AiDifficulty);

#[derive(Component)]
pub(super) struct StartButton;

// note
#[derive(Component)]
pub(super) struct ChangeLanguageTry;

// slider
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SliderTarget {
    MapSize,
    MapShape,
    AiDifficulty,
    BotCount,
}

#[derive(Component)]
pub(super) struct Slider {
    pub target: SliderTarget,
    pub steps: usize,   // сколько дискретных позиций
    pub current: usize, // текущий индекс
    pub dragging: bool,
}

#[derive(Component)]
pub(super) struct SliderHandle;

#[derive(Component)]
pub(super) struct SliderValueLabel(pub SliderTarget);

impl Default for GameSetupSelection {
    fn default() -> Self {
        Self {
            race: None,
            map_size: MapSize::Medium,   // индекс 1 в MAP_SIZE_VARIANTS
            map_shape: MapShape::Jagged, // индекс 0 в MAP_SHAPE_VARIANTS
            ai_difficulty: AiDifficulty::Hard, // индекс 2 в AI_DIFFICULTY_VARIANTS
            bot_count: 2,
        }
    }
}
