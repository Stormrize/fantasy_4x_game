use crate::world::components::{AiDifficulty, Gold, PlayerType};
use crate::world::game_configs::{GameLogicConfig, Player};

/// Небольшой бонус к доходу (золото/еда/наука) для ботов на сложности Crazy.
pub const CRAZY_INCOME_MULTIPLIER: f32 = 1.2;

/// Параметры поведения бота в зависимости от сложности.
///
/// Бонусов к ресурсам нет ни на одной сложности, кроме Crazy.
/// Боты не видят сквозь туман войны: чем выше сложность, тем лучше память
/// и агрессивнее разведка/экономика, а не «читерские» знания.
#[derive(Debug, Clone)]
pub struct AiProfile {
    /// Максимум строителей, которых бот готов нанять.
    pub max_helpers: usize,
    /// Сколько строителей можно нанять за один ход.
    pub max_hires_per_turn: usize,
    /// Сколько улучшений зданий можно сделать за один ход.
    pub max_upgrades_per_turn: usize,
    /// Максимум боевых юнитов, которых бот хочет держать.
    pub max_combat_units: usize,
    /// Сколько боевых юнитов можно нанять за один ход.
    pub max_combat_hires_per_turn: usize,
    /// Неприкосновенный запас золота: бот не тратит, опускаясь ниже него.
    pub gold_reserve: Gold,
    /// Сколько ходов бот помнит позиции чужих юнитов после того, как их видел.
    pub unit_memory_turns: u64,
    /// Вес разведки при выборе действий строителей.
    pub exploration_weight: f32,
    /// Предпочтение науки при выборе зданий и целей.
    pub science_bias: f32,
}

impl AiProfile {
    pub fn from_difficulty(difficulty: AiDifficulty) -> Self {
        match difficulty {
            AiDifficulty::Easy => Self {
                max_helpers: 2,
                max_hires_per_turn: 1,
                max_upgrades_per_turn: 1,
                max_combat_units: 1,
                max_combat_hires_per_turn: 1,
                gold_reserve: 25,
                unit_memory_turns: 0, // «золотая рыбка»: юнитов забывает сразу
                exploration_weight: 0.8,
                science_bias: 0.8,
            },
            AiDifficulty::Normal => Self {
                max_helpers: 3,
                max_hires_per_turn: 2,
                max_upgrades_per_turn: 1,
                max_combat_units: 2,
                max_combat_hires_per_turn: 1,
                gold_reserve: 15,
                unit_memory_turns: 2,
                exploration_weight: 1.0,
                science_bias: 1.0,
            },
            AiDifficulty::Hard => Self {
                max_helpers: 4,
                max_hires_per_turn: 2,
                max_upgrades_per_turn: 2,
                max_combat_units: 3,
                max_combat_hires_per_turn: 1,
                gold_reserve: 10,
                unit_memory_turns: 4,
                exploration_weight: 1.2,
                science_bias: 1.2,
            },
            AiDifficulty::Crazy => Self {
                max_helpers: 5,
                max_hires_per_turn: 3,
                max_upgrades_per_turn: 2,
                max_combat_units: 4,
                max_combat_hires_per_turn: 2,
                gold_reserve: 5,
                unit_memory_turns: 6,
                exploration_weight: 1.4,
                science_bias: 1.4,
            },
        }
    }

    /// Множитель дохода для игрока `id`: 1.0 для человека и для ботов не-Crazy.
    pub fn income_multiplier_for(config: &GameLogicConfig, player: Player) -> f32 {
        let is_bot = player.player_type == PlayerType::Bot;
        if is_bot && config.ai_difficulty == Some(AiDifficulty::Crazy) {
            CRAZY_INCOME_MULTIPLIER
        } else {
            1.0
        }
    }
}
