use bevy::prelude::{MessageReader, MessageWriter, Res};

use crate::game::game_logic_plugin::events_resources_structs::action_messages::{
    Clear, Cycle, Move, NewTile,
};
use crate::game::game_logic_plugin::events_resources_structs::{
    action_messages::TileClickedEvent, components::*,
};
use crate::world::game_configs::GameLogicConfig;
use crate::world::game_resources::WorldMaps;

pub fn route_selection(
    world_maps: WorldMaps,
    config: Res<GameLogicConfig>,
    already_selected_tile: Res<AlreadySelectedTile>,
    mut tile_clicked: MessageReader<TileClickedEvent>,
    mut move_cmd: MessageWriter<Move>,
    mut cycle: MessageWriter<Cycle>,
    mut new_tile: MessageWriter<NewTile>,
    mut clear: MessageWriter<Clear>,
) {
    for click in tile_clicked.read() {
        let clicked_pos = click.pos;

        if clicked_pos.x < 0
            || clicked_pos.y < 0
            || clicked_pos.x >= config.terrain_size as i32
            || clicked_pos.y >= config.terrain_size as i32
        {
            clear.write(Clear);
            continue;
        }

        let tile_content = TileContent::from_world(
            true,
            world_maps
                .units
                .is_some(clicked_pos.x as usize, clicked_pos.y as usize),
            world_maps
                .buildings
                .is_some(clicked_pos.x as usize, clicked_pos.y as usize),
            world_maps
                .resources
                .is_some(clicked_pos.x as usize, clicked_pos.y as usize),
            world_maps
                .fog
                .get(clicked_pos.x as usize, clicked_pos.y as usize),
        );

        let just_selected_tile = JustSelectedTile {
            pos: clicked_pos,
            tile_content,
        };

        match already_selected_tile.pos {
            None => {
                new_tile.write(NewTile(just_selected_tile));
            }

            Some(selected_pos) if selected_pos == clicked_pos => {
                cycle.write(Cycle(just_selected_tile));
            }

            Some(selected_pos) => {
                move_cmd.write(Move {
                    entity_pos: selected_pos,
                    target_pos: clicked_pos,
                });
            }
        }
    }
}
