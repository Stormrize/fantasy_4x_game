use crate::world::game_resources::data::biome::BiomeType;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TerrainMap {
    pub map: Vec<BiomeType>,
    pub size: usize,
}
