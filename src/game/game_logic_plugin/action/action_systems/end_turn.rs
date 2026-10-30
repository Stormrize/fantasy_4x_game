use crate::game::game_logic_plugin::ai::{ActiveBotTurn, ActiveBotTurnState, AiPhase};
use crate::game::game_logic_plugin::events_resources_structs::action_messages::EndTurn;
use crate::world::components::{Id, PlayerType};
use crate::world::game_configs::{GameLogicConfig, Player};
use crate::world::turn_manager::{TurnManager, TurnTransition};
use bevy::prelude::{MessageReader, Res, ResMut};

pub fn end_turn(
    mut turn_manager: ResMut<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    mut turn_ended: MessageReader<EndTurn>,
    mut active_bot_turn: ResMut<ActiveBotTurnState>,
) {
    for _ in turn_ended.read() {
        let transition = turn_manager.next_turn(&game_logic_config);

        match transition {
            TurnTransition::LocalPlayerTurn(player) => {
                turn_manager.current_player = player;
                active_bot_turn.0 = None;
                println!("Current player is {:#?} from local", player);
            }
            TurnTransition::TriggerBotAI(player) => {
                turn_manager.current_player = player;
                // Ход бота: планирование начнётся в ai::start_bot_turn.
                active_bot_turn.0 = Some(ActiveBotTurn {
                    player,
                    phase: AiPhase::Planning,
                });
                println!("Current player is {:#?} from ai", player);
            }
            TurnTransition::WaitForNetwork(player) => {
                turn_manager.current_player = player;
                // await for a http request
            }
            TurnTransition::SimulationTime => {
                // Боевая фаза: здесь будет планирование боевого ИИ.
                // После неё ход возвращается стартовому игроку (человеку).
                turn_manager.current_player =
                    Player::new(Id(turn_manager.starting_index), PlayerType::Human);
                active_bot_turn.0 = None;
                println!("Simulation time");
            }
        }
    }
}
