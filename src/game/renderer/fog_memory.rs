use bevy::prelude::*;

use crate::world::components::PlayerRace;
use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::WorldMaps;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::building::BuildingType;
use crate::world::game_resources::data::unit::UnitType;
use crate::world::game_resources::fog_map::FogMap;

/// Render-ready description of one map layer: the owning race and the asset
/// index inside `RaceAssets`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerVisual {
    pub race: PlayerRace,
    pub index: usize,
}

impl LayerVisual {
    pub fn new(race: &PlayerRace, index: usize) -> Self {
        Self {
            race: race.clone(),
            index,
        }
    }
}

/// Everything the local player last saw on a single tile while it was fully
/// visible. Tiles that were only explored (general visibility) are rendered
/// from this snapshot instead of the live maps.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TileMemory {
    pub terrain: Option<LayerVisual>,
    pub resource: Option<LayerVisual>,
    pub building: Option<LayerVisual>,
    pub unit: Option<LayerVisual>,
}

/// Local player's explored memory.
///
/// For explored tiles it stores the static terrain and the contents that were
/// visible the last time the tile was in full visibility. A tile that fades
/// back into the fog therefore keeps exactly the state it was last observed in.
#[derive(Resource, Default)]
pub struct FogMemory {
    size: usize,
    tiles: Vec<TileMemory>,
}

impl FogMemory {
    pub fn clear(&mut self) {
        self.size = 0;
        self.tiles.clear();
    }

    /// Resizes the memory when a new world is generated.
    pub fn ensure_size(&mut self, size: usize) {
        if self.size == size {
            return;
        }
        self.size = size;
        self.tiles = vec![TileMemory::default(); size * size];
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<&TileMemory> {
        if row >= self.size || col >= self.size {
            return None;
        }
        self.tiles.get(row * self.size + col)
    }

    #[inline]
    fn set(&mut self, row: usize, col: usize, tile: TileMemory) {
        if row >= self.size || col >= self.size {
            return;
        }
        self.tiles[row * self.size + col] = tile;
    }
}

/// A snapshot of what a loaded chunk currently renders. It is compared with
/// the freshly captured snapshot to rebuild only the layers that actually
/// changed in that chunk.
#[derive(Debug, Clone, Default)]
pub struct ChunkSnapshot {
    pub tiles: Vec<TileMemory>,
    pub explored: Vec<bool>,
}

impl ChunkSnapshot {
    pub fn capture(config: &RenderConfig, memory: &FogMemory, fog: &FogMap, coord: IVec2) -> Self {
        let chunk_size = config.chunk_size;
        let min_x = coord.x * chunk_size as i32;
        let min_y = coord.y * chunk_size as i32;

        let mut tiles = Vec::with_capacity(chunk_size * chunk_size);
        let mut explored = Vec::with_capacity(chunk_size * chunk_size);

        for row in min_x..min_x + chunk_size as i32 {
            for col in min_y..min_y + chunk_size as i32 {
                let (row, col) = (row as usize, col as usize);
                tiles.push(memory.get(row, col).cloned().unwrap_or_default());
                explored.push(fog.is_general_visible_player(row, col));
            }
        }

        Self { tiles, explored }
    }

    pub fn terrain_differs(&self, other: &Self) -> bool {
        self.tiles
            .iter()
            .zip(&other.tiles)
            .any(|(a, b)| a.terrain != b.terrain)
    }

    pub fn resources_differ(&self, other: &Self) -> bool {
        self.tiles
            .iter()
            .zip(&other.tiles)
            .any(|(a, b)| a.resource != b.resource)
    }

    pub fn buildings_differ(&self, other: &Self) -> bool {
        self.tiles
            .iter()
            .zip(&other.tiles)
            .any(|(a, b)| a.building != b.building)
    }

    pub fn fog_differs(&self, other: &Self) -> bool {
        self.explored != other.explored
    }
}

/// Refreshes the local player's explored memory with everything currently
/// visible. Static layers are remembered indefinitely; entities that disappear
/// while still observed are removed from the memory as well.
pub fn update_fog_memory(
    mut memory: ResMut<FogMemory>,
    world: WorldMaps,
    buildings_query: Query<(&PlayerRace, &BuildingType)>,
    units_query: Query<(&PlayerRace, &UnitType)>,
) {
    let changed = world.fog.is_changed()
        || world.terrain.is_changed()
        || world.resources.is_changed()
        || world.buildings.is_changed()
        || world.units.is_changed();
    if !changed {
        return;
    }

    let size = world.fog.size;
    memory.ensure_size(size);

    for row in 0..size {
        for col in 0..size {
            if !world.fog.is_general_visible_player(row, col) {
                continue;
            }

            let full_visible = world.fog.is_full_visible_player(row, col);
            let mut tile = memory.get(row, col).cloned().unwrap_or_default();

            // Terrain is static, so it is known for every explored tile even
            // if the tile is not currently in the line of sight. `Ground` has
            // no render assets and is treated as empty terrain.
            let biome = world.terrain.get_biome(row, col);
            tile.terrain = match biome.biome_type {
                BiomeType::Ground => None,
                _ => Some(LayerVisual::new(
                    biome.owner_race(),
                    biome.biome_type.index(),
                )),
            };

            if full_visible {
                tile.resource = world
                    .resources
                    .get(row, col)
                    .map(|resource| LayerVisual::new(resource.owner_race(), resource.index()));

                tile.building = world
                    .buildings
                    .get(row, col)
                    .and_then(|entity| buildings_query.get(entity).ok())
                    .map(|(race, building_type)| LayerVisual::new(race, building_type.index()));

                tile.unit = world
                    .units
                    .get(row, col)
                    .and_then(|entity| units_query.get(entity).ok())
                    .map(|(race, unit_type)| LayerVisual::new(race, unit_type.index()));
            }

            memory.set(row, col, tile);
        }
    }
}
