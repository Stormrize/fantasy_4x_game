use crate::game::game_logic_plugin::action::action_router::route_action;
use crate::game::game_logic_plugin::events_resources_structs::action_messages::ActionCommandWriters;
use crate::game::game_logic_plugin::events_resources_structs::components::{AlreadySelectedTile, JustSelectedTile, TileContent, TileLayer, WorldMaps};
use crate::game::game_logic_plugin::events_resources_structs::render_messages::TileClickedEvent;
use crate::game::game_logic_plugin::selection::selection_systems::clear::clear_selection_system;
use crate::game::game_logic_plugin::selection::selection_systems::cycle::cycle_selection_system;
use crate::game::game_logic_plugin::selection::selection_systems::new_tile::new_tile_selection_system;
use bevy::prelude::{MessageReader, ResMut};

pub fn route_selection(
    world_maps: WorldMaps,

    mut already_selected_tile: ResMut<AlreadySelectedTile>,
    mut tile_clicked: MessageReader<TileClickedEvent>,

    mut commands: ActionCommandWriters,
) {
    for click in tile_clicked.read() {
        let clicked_pos = click.pos;

        if clicked_pos.x < 0
            || clicked_pos.y < 0
            || clicked_pos.x >= world_maps.config.terrain_size as i32
            || clicked_pos.y >= world_maps.config.terrain_size as i32
        {
            clear_selection_system(&mut already_selected_tile);
            continue;
        }
        
        let tile_content = TileContent::from_world(
            world_maps
                .terrain
                .get(clicked_pos.x as usize, clicked_pos.y as usize)
                .clone(),
            world_maps
                .units
                .get(clicked_pos.x as usize, clicked_pos.y as usize)
                .clone(),
            world_maps
                .buildings
                .get(clicked_pos.x as usize, clicked_pos.y as usize)
                .clone(),
            world_maps
                .resources
                .get(clicked_pos.x as usize, clicked_pos.y as usize)
                .clone(),
            world_maps
                .fog
                .get(clicked_pos.x as usize, clicked_pos.y as usize)
        );

        let just_selected_tile = JustSelectedTile { 
            pos: clicked_pos, 
            tile_content
        };

        match already_selected_tile.pos {
            None => {
                new_tile_selection_system(just_selected_tile, &mut already_selected_tile);
            }

            Some(selected_pos) if selected_pos == clicked_pos => {
                cycle_selection_system(just_selected_tile, &mut already_selected_tile);
            }

            Some(_) => {
                route_action(
                    &*world_maps.config,
                    just_selected_tile,
                    &mut already_selected_tile,
                    &mut commands,
                );
            }
        }
    }
}
