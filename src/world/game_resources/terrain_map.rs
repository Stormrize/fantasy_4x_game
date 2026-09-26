use bevy::prelude::Resource;

use crate::world::game_resources::data::biome::BiomeType;

#[derive(Resource)]
pub struct TerrainMap {
    pub map: Vec<BiomeType>,
    pub size: usize,
}

impl TerrainMap {
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> &BiomeType {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        &self.map[row * self.size + col]
    }
}
