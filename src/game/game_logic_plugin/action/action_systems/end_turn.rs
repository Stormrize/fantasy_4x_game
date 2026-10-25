use crate::game::game_logic_plugin::events_resources_structs::action_messages::EndTurn;
use crate::world::game_configs::GameLogicConfig;
use crate::world::turn_manager::{TurnManager, TurnTransition};
use bevy::prelude::{MessageReader, Res, ResMut};

pub fn end_turn(
    mut turn_manager: ResMut<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    mut turn_ended: MessageReader<EndTurn>,
) {
    for _ in turn_ended.read() {
        let transition = turn_manager.next_turn(&game_logic_config);

        match transition {
            TurnTransition::LocalPlayerTurn(player) => {
                turn_manager.current_player = player;
                println!("Current player is {:#?} from local", player);
            }
            TurnTransition::TriggerBotAI(player) => {
                turn_manager.current_player = player;
                // start move, economy AI logic
                println!("Current player is {:#?} from ai", player);
            }
            TurnTransition::WaitForNetwork(player) => {
                turn_manager.current_player = player;
                // await for a http request
            }
            TurnTransition::SimulationTime => {
                // start AI fight logic
            }
        }
    }
}
