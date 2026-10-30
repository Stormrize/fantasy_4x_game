use crate::game::game_logic_plugin::action::rules::move_rules::{
    MovementCostGrid, check_move_validity, is_players_turn, is_players_unit,
};
use crate::game::game_logic_plugin::events_resources_structs::action_messages::{Clear, Move};
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderMoveEntity;
use crate::world::components::{GameMode, Id};
use crate::world::game_configs::GameLogicConfig;
use crate::world::game_resources::data::unit::MovementPoints;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;
use bevy::prelude::{MessageReader, MessageWriter, Query, Res, ResMut};

#[allow(clippy::too_many_arguments)]
pub fn move_entity(
    mut move_command: MessageReader<Move>,
    turn_manager: Res<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    cost_grid: Res<MovementCostGrid>,
    mut units_map: ResMut<UnitsMap>,
    mut movement_stats: Query<(&Id, &mut MovementPoints)>,
    mut render_move: MessageWriter<RenderMoveEntity>,
    mut clear: MessageWriter<Clear>,
) {
    for command in move_command.read() {
        let current_pos = command.entity_pos;
        let target_pos = command.target_pos;
        let Some(selected_entity) = units_map.get(current_pos.x as usize, current_pos.y as usize)
        else {
            clear.write(Clear);
            continue;
        };

        let (id, mut movement_points) = movement_stats.get_mut(selected_entity).unwrap();

        if !is_players_unit(*id, turn_manager.current_player) {
            println!("Not your race");
            clear.write(Clear);
            continue;
        }

        // В онлайне нельзя ходить вне своего хода (сервер проверит ещё раз).
        if matches!(turn_manager.game_mode, GameMode::Online)
            && !is_players_turn(game_logic_config.player, turn_manager.current_player)
        {
            println!("Not your turn");
            clear.write(Clear);
            continue;
        }

        let Some(cost) = check_move_validity(
            &cost_grid,
            &units_map,
            movement_points.0,
            current_pos,
            target_pos,
        ) else {
            println!("Not valid move");
            clear.write(Clear);
            continue;
        };

        if matches!(turn_manager.game_mode, GameMode::Online) {
            // send http request
        }

        units_map.set(current_pos.x as usize, current_pos.y as usize, None);
        units_map.set(
            target_pos.x as usize,
            target_pos.y as usize,
            Some(selected_entity),
        );

        render_move.write(RenderMoveEntity {
            entity: selected_entity,
            current_pos,
            new_pos: target_pos,
        });
        movement_points.0 -= cost;
        clear.write(Clear);
    }
}
