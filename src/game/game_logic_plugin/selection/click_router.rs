use bevy::prelude::{MessageReader, Res, ResMut};

use crate::game::game_logic_plugin::events_resources_structs::action_messages::MoveEntityCommand;
use crate::game::game_logic_plugin::{
    events_resources_structs::{
        action_messages::{ActionCommandWriters, TileClickedEvent},
        components::*,
    },
    selection::selection_systems::{
        clear::clear_selection_system, cycle::cycle_selection_system,
        new_tile::new_tile_selection_system,
    },
};
use crate::world::game_configs::GameLogicConfig;
use crate::world::game_resources::WorldMaps;

pub fn route_selection(
    world_maps: WorldMaps,
    config: Res<GameLogicConfig>,
    mut already_selected_tile: ResMut<AlreadySelectedTile>,
    mut tile_clicked: MessageReader<TileClickedEvent>,
    mut commands: ActionCommandWriters,
) {
    for click in tile_clicked.read() {
        let clicked_pos = click.pos;

        if clicked_pos.x < 0
            || clicked_pos.y < 0
            || clicked_pos.x >= config.terrain_size as i32
            || clicked_pos.y >= config.terrain_size as i32
        {
            clear_selection_system(&mut already_selected_tile);
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
                new_tile_selection_system(just_selected_tile, &mut already_selected_tile);
            }

            Some(selected_pos) if selected_pos == clicked_pos => {
                cycle_selection_system(just_selected_tile, &mut already_selected_tile);
            }

            Some(selected_pos) => {
                commands.move_cmd.write(MoveEntityCommand {
                    entity_pos: selected_pos,
                    target_pos: clicked_pos,
                });
            }
        }
    }
}
