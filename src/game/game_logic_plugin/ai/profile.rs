use crate::world::components::{AiDifficulty, Gold, PlayerType};
use crate::world::game_configs::{GameLogicConfig, Player};

/// Small income bonus (gold/food/science) for bots on Crazy difficulty.
pub const CRAZY_INCOME_MULTIPLIER: f32 = 1.2;

/// Bot behavior parameters, depending on difficulty.
///
/// There are no resource bonuses on any difficulty except Crazy. Bots do not
/// see through the fog of war: higher difficulties get better memory and more
/// aggressive exploration/economy, not "cheating" knowledge.
#[derive(Debug, Clone)]
pub struct AiProfile {
    /// Maximum number of builders the bot is willing to hire.
    pub max_helpers: usize,
    /// How many builders can be hired per turn.
    pub max_hires_per_turn: usize,
    /// How many building upgrades can be done per turn.
    pub max_upgrades_per_turn: usize,
    /// Maximum number of combat units the bot wants to keep.
    pub max_combat_units: usize,
    /// How many combat units can be hired per turn.
    pub max_combat_hires_per_turn: usize,
    /// Untouchable gold reserve: the bot never spends below it.
    pub gold_reserve: Gold,
    /// How many turns the bot remembers enemy unit positions after seeing them.
    pub forget_memory_turns: u64,
    /// Exploration weight when choosing builder actions.
    pub exploration_weight: f32,
    /// Science preference when choosing buildings and targets.
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
                forget_memory_turns: 0, // "goldfish": forgets units immediately
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
                forget_memory_turns: 2,
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
                forget_memory_turns: 4,
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
                forget_memory_turns: 6,
                exploration_weight: 1.4,
                science_bias: 1.4,
            },
        }
    }

    /// Income multiplier for player `id`: 1.0 for the human and for non-Crazy
    /// bots.
    pub fn income_multiplier_for(config: &GameLogicConfig, player: Player) -> f32 {
        let is_bot = player.player_type == PlayerType::Bot;
        if is_bot && config.ai_difficulty == Some(AiDifficulty::Crazy) {
            CRAZY_INCOME_MULTIPLIER
        } else {
            1.0
        }
    }
}
