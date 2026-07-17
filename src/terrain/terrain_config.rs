use bevy::prelude::Resource;
use crate::terrain::terrain_config::TerrainShape::Jagged;
// pub const TERRAIN_SIZE: usize = 32;
// pub const CHUNK_SIZE: usize = 4;
// pub const TILE_SIZE: f32 = 1.0;
// 
// pub const MIN_ORTHOGRAPHIC_SCALE: f32 = 0.1;
// pub const MAX_ORTHOGRAPHIC_SCALE: f32 = 100.0;

#[derive(Resource)]
pub struct TerrainConfig {
    pub seed: u32,
    pub terrain_size: usize,
    pub chunk_size: usize,
    pub tile_size: f32,
    pub min_orthographic_scale: f32,
    pub max_orthographic_scale: f32,
    //pub camera_viewport_height: f32,
    pub preload_margin_chunks: u8,
    pub unload_margin_chunks: u8,
    pub terrain_shape: TerrainShape,
    pub water_threshold: u8,
    pub threshold_grass: u8,
    pub threshold_forest: u8,
    pub octaves: u8,
    pub frequency: f64,
    pub persistence: f64,
    pub lacunarity: f64,
    pub detail_frequency: f64,
    pub detail_weight: f64,

}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            seed: 32,
            terrain_size: 32,
            chunk_size: 4,
            tile_size: 1.0,
            min_orthographic_scale: 0.1,
            max_orthographic_scale: 100.0,
            //camera_viewport_height: CHUNK_SIZE as f32 * 1.5,
            preload_margin_chunks: 2,
            unload_margin_chunks: 2,
            terrain_shape: Jagged,
            water_threshold: 90,
            threshold_grass: 140,
            threshold_forest: 170,
            octaves: 6,
            frequency: 8.0,
            persistence: 0.5,
            lacunarity: 2.0,

            detail_frequency: 0.075,
            detail_weight: 0.15,
        }
    }
}

impl TerrainConfig {
    pub fn with_seed(seed: u32) -> Self {
        Self {
            seed, 
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum TerrainShape {
    #[default]
    Round,
    Jagged,
}
