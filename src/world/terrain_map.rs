use bevy::prelude::Resource;
use crate::world::enums::biome::BiomeType;

pub struct GlobalTerrainMap {
    pub map: Vec<BiomeType>,
    pub size: usize,
}

#[derive(Resource)]
pub struct TerrainMap {
    pub map: Vec<BiomeType>,
    pub size: usize,
}
