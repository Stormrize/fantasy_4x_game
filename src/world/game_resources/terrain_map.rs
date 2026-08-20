use crate::world::game_resources::data::biome::BiomeType;
use bevy::prelude::Resource;

pub struct GlobalTerrainMap {
    pub map: Vec<BiomeType>,
    pub size: usize,
}

#[derive(Resource)]
pub struct TerrainMap {
    pub map: Vec<BiomeType>,
    pub size: usize,
}
