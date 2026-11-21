use crate::game::events::action_messages::EndTurn;
use crate::game::game_logic_plugin::action::rules::move_rules::{is_combat, is_players_turn};
use crate::game::game_logic_plugin::ai::{ActiveBotTurn, ActiveBotTurnState, AiPhase};
use crate::game::game_logic_plugin::combat::integration::{CombatCycle, CombatPhase};
use crate::world::game_configs::GameLogicConfig;
use crate::world::turn_manager::{TurnManager, TurnTransition};
use bevy::prelude::{MessageReader, Res, ResMut};

pub fn end_turn(
    mut turn_manager: ResMut<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    mut turn_ended: MessageReader<EndTurn>,
    mut active_bot_turn: ResMut<ActiveBotTurnState>,
    mut combat_cycle: ResMut<CombatCycle>,
) {
    for action in turn_ended.read() {
        if !is_players_turn(action.0, turn_manager.current_player) {
            return;
        }

        if is_combat(&turn_manager.combat_phase) {
            return;
        }

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
                // Combat phase: give the local player time to queue orders in the
                // combat UI, then simulate. The turn returns to the human after
                // the simulation finishes (poll_combat).
                active_bot_turn.0 = None;
                combat_cycle.phase = CombatPhase::PlayerPlanning;
                println!("Simulation time");
            }
        }
    }
}
