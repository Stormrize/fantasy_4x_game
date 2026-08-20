use bevy::prelude::{MessageReader, MessageWriter, Res};
use crate::game::game_logic_plugin::selection::events_resources_structs::components::{SelectedEntity, TileContent};
use crate::game::game_logic_plugin::selection::events_resources_structs::messages::{ActionSelectionEvent, ClearSelectionEvent, CycleSelectionEvent, NewTileSelectionEvent, TileClickedEvent};

use crate::world::{
    game_config::GameConfig,
    game_resources::{
        buildings_map::BuildingMap,
        fog_map::FogMap,
        resource_map::ResourceMap,
        terrain_map::TerrainMap,
        unit_map::UnitsMap,
        data::{
            buildings::BuildingType,
            biome::BiomeType,
            resources::ResourceType,
            units::Unit
        }
    }
};

pub fn route_selection(
    config: Res<GameConfig>,
    terrain: Res<TerrainMap>,
    resources: Res<ResourceMap>,
    buildings: Res<BuildingMap>,
    units: Res<UnitsMap>,
    fog: Res<FogMap>,

    selected: Res<SelectedEntity>,

    mut tile_clicked: MessageReader<TileClickedEvent>,

    mut new_tile: MessageWriter<NewTileSelectionEvent>,
    mut cycle: MessageWriter<CycleSelectionEvent>,
    mut action: MessageWriter<ActionSelectionEvent>,
    mut clear: MessageWriter<ClearSelectionEvent>,
) {
    for click in tile_clicked.read() {
        let pos = click.grid_pos;

        if pos.x < 0
            || pos.y < 0
            || pos.x >= config.terrain_size as i32
            || pos.y >= config.terrain_size as i32
        {
            clear.write(ClearSelectionEvent);
            continue;
        }

        let tile = TileContent::from_world(
            pos,
            match terrain.get(pos.x as usize, pos.y as usize).clone() { 
                BiomeType::Water(_) => true,
                _ => false,
            },
            units.get(pos.x as usize, pos.y as usize).clone() != Unit::None,
            buildings.get(pos.x as usize, pos.y as usize).clone() != BuildingType::None,
            resources.get(pos.x as usize, pos.y as usize).clone() != ResourceType::None,
            fog.get(pos.x as usize, pos.y as usize) == true,
        );

        match selected.position {
            None => {
                new_tile.write(NewTileSelectionEvent { tile });
            }

            Some(selected_pos) if selected_pos == pos => {
                cycle.write(CycleSelectionEvent { tile });
            }

            Some(selected_pos) => {
                action.write(ActionSelectionEvent {
                    selected_pos,
                    target_tile: tile,
                    selected_layer: selected.layer.unwrap(),
                });
            }
        }
    }
}
