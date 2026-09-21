use bevy::prelude::Resource;

use crate::world::game_resources::data::biome::BiomeType;

#[derive(Resource)]
pub struct TerrainMap {
    pub map: Vec<BiomeType>,
    pub size: usize,
}
