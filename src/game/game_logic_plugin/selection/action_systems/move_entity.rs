use crate::game::game_logic_plugin::events_resources_structs::action_messages::MoveEntityCommand;
use crate::game::game_logic_plugin::events_resources_structs::components::AlreadySelectedTile;
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderMoveEntity;
use crate::game::game_logic_plugin::selection::action_systems::rules::{
    check_move_validity, is_players_unit,
};
use crate::game::game_logic_plugin::selection::selection_systems::clear::clear_selection_system;
use crate::world::components::Id;
use crate::world::game_configs::GameLogicConfig;
use crate::world::game_resources::data::unit::MovementPoints;
use crate::world::game_resources::unit_map::UnitsMap;
use bevy::prelude::{MessageReader, MessageWriter, Query, Res, ResMut};

pub fn move_entity(
    mut move_command: MessageReader<MoveEntityCommand>,
    config: Res<GameLogicConfig>,
    mut units_map: ResMut<UnitsMap>,
    mut already_selected_tile: ResMut<AlreadySelectedTile>,
    movement_stats: Query<(&Id, &MovementPoints)>,
    mut render_move: MessageWriter<RenderMoveEntity>,
) {
    for command in move_command.read() {
        let current_pos = command.entity_pos;
        let target_pos = command.target_pos;
        let selected_entity = units_map
            .get(current_pos.x as usize, current_pos.y as usize)
            .unwrap();

        let (id, movement_points ) = movement_stats.get(selected_entity).unwrap();

        if !is_players_unit(*id, config.player) {
            println!("Not you race");
            return;
        }

        if !check_move_validity(movement_points.0, current_pos, target_pos) {
            println!("Move invalidated");
            clear_selection_system(&mut already_selected_tile);
            break;
        }

        units_map.set(current_pos.x as usize, current_pos.y as usize, None);
        units_map.set(
            target_pos.x as usize,
            target_pos.y as usize,
            Some(selected_entity),
        );

        println!("Moved to position: {}{}", target_pos.x, target_pos.y);
        render_move.write(RenderMoveEntity {
            entity: selected_entity,
            current_pos,
            new_pos: target_pos,
        });
        clear_selection_system(&mut already_selected_tile);
    }
}
