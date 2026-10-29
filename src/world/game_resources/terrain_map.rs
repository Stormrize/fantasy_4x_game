use crate::world::game_resources::data::biome::{Biome, BiomeType};
use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Resource, Deserialize, Debug)]
pub struct TerrainMap {
    pub map: Vec<Biome>,
    pub size: usize,
}

impl TerrainMap {
    #[inline]
    pub fn get_biome(&self, row: usize, col: usize) -> &Biome {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        &self.map[row * self.size + col]
    }

    #[inline]
    pub fn get_biome_type(&self, row: usize, col: usize) -> BiomeType {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        self.map[row * self.size + col].biome_type
    }
}
