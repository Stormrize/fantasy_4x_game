use bevy::prelude::{MessageReader, MessageWriter, Res};
use crate::game_logic::selection::events_resources_structs::components::{SelectedEntity, TileContent};
use crate::game_logic::selection::events_resources_structs::messages::{ActionSelectionEvent, ClearSelectionEvent, CycleSelectionEvent, NewTileSelectionEvent, TileClickedEvent};
use crate::world::buildings_map::BuildingMap;
use crate::world::enums::biome::BiomeType;
use crate::world::enums::buildings::BuildingType;
use crate::world::enums::resources::ResourceType;
use crate::world::enums::units::Unit;
use crate::world::fog_map::FogMap;
use crate::world::resource_map::ResourceMap;
use crate::world::game_config::TerrainConfig;
use crate::world::terrain_map::TerrainMap;
use crate::world::unit_map::UnitsMap;

pub fn route_selection(
    config: Res<TerrainConfig>,
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


        if pos.x < 0 || pos.y < 0
            || pos.x >= config.terrain_size as i32
            || pos.y >= config.terrain_size as i32
        {
            clear.write(ClearSelectionEvent);
            continue;
        }

        let tile = TileContent::from_world(
            pos,
            terrain.get(pos.x as usize, pos.y as usize) == BiomeType::Water,
            units.get(pos.x as usize, pos.y as usize) != Unit::None,
            buildings.get(pos.x as usize, pos.y as usize).0 != BuildingType::None,
            resources.get(pos.x as usize, pos.y as usize) != ResourceType::None,
            fog.get(pos.x as usize, pos.y as usize) == true,
        );


        match selected.position {
            None => {
                new_tile.write(NewTileSelectionEvent {
                    tile,
                });
            }

            Some(selected_pos) if selected_pos == pos => {
                cycle.write(CycleSelectionEvent {
                    tile,
                });
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