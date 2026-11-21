use crate::game::events::action_messages::EndTurn;
use crate::game::game_logic_plugin::action::rules::move_rules::is_players_turn;
use crate::game::game_logic_plugin::ai::{ActiveBotTurn, ActiveBotTurnState, AiPhase};
use crate::game::game_logic_plugin::combat::integration::{CombatCycle, CombatPhase};
use crate::world::game_configs::GameLogicConfig;
use crate::world::turn_manager::{TurnManager, TurnTransition};
use bevy::prelude::{MessageReader, Res, ResMut};

/// end turn action, gives a turn to next player
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
        // Combat is resolved through the combat cycle; ignore end-turn input
        // while it is being planned or simulated.
        if !matches!(combat_cycle.phase, CombatPhase::Idle) {
            return;
        }

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
                // Combat phase: kick off background combat-AI planning and
                // simulation. The turn returns to the human once it finishes
                // (poll_combat).
                active_bot_turn.0 = None;
                combat_cycle.phase = CombatPhase::Planning;
                println!("Simulation time");
            }
        }
    }
}
