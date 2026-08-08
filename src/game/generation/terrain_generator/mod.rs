use ::noise::Perlin;
use bevy::prelude::Resource;
use crate::world::game_config::GameConfig;

mod local_terrain_generation;
mod global_terrain_generation;
mod noise;

pub struct PerlinPlaceholder {
    pub perlin_main: Perlin,
    pub perlin_detail: Perlin,
    pub perlin_local: Perlin,
}

impl Default for PerlinPlaceholder {
    fn default() -> Self {
        Self {
            perlin_main: Perlin::new(42),
            perlin_detail: Perlin::new(42_u32.wrapping_add(1337)),
            perlin_local: Perlin::new(42_u32.wrapping_add(7331)),
        }
    }
}

impl PerlinPlaceholder {
    pub fn new(config: &GameConfig) -> Self {
        Self {
            perlin_main: Perlin::new(config.seed),
            perlin_detail: Perlin::new(config.seed.wrapping_sub(1337)),
            perlin_local: Perlin::new(config.seed.wrapping_add(7331)),
        }
    }
}