use crate::world::components::{GameMode, Id, PlayerType};
use crate::world::game_configs::{GameLogicConfig, Player};
use bevy::prelude::Resource;

#[derive(Debug, Clone, Copy)]
pub enum TurnTransition {
    LocalPlayerTurn(Player),
    TriggerBotAI(Player),
    WaitForNetwork(Player),
    SimulationTime,
}

#[derive(Resource, Debug)]
pub struct TurnManager {
    pub current_player: Player,
    pub game_mode: GameMode,
    pub starting_index: u16,
    pub length: u16,
}

impl TurnManager {
    pub fn new(id: Id, player_type: PlayerType, length: u16, game_mode: GameMode) -> Self {
        Self {
            starting_index: id.0,
            length,
            current_player: Player::new(id, player_type),
            game_mode,
        }
    }

    pub fn next_turn(&self, game_logic_config: &GameLogicConfig) -> TurnTransition {
        let new_player_index = (self.current_player.id.0 + 1) % self.length;

        if new_player_index == self.starting_index {
            return TurnTransition::SimulationTime;
        }

        let player = game_logic_config.players.0[new_player_index as usize];
        match self.game_mode {
            GameMode::Offline => {
                if player.id.0 == new_player_index {
                    TurnTransition::LocalPlayerTurn(player)
                } else {
                    TurnTransition::TriggerBotAI(player)
                }
            }
            GameMode::Online => {
                if player.id.0 == new_player_index {
                    TurnTransition::LocalPlayerTurn(player)
                } else {
                    TurnTransition::WaitForNetwork(player)
                }
            }
        }
    }
}
