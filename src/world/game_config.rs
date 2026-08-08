use crate::world::game_config::TerrainShape::Jagged;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameConfig {
    pub seed: u32,                      //4
    //pub current_players: Vec<(u8, FantasyRace)>,//?
    pub terrain_size: usize,            //8
    pub chunk_size: usize,              //8
    pub tile_size: f32,                 //4
    pub min_orthographic_scale: f32,    //4
    pub max_orthographic_scale: f32,    //4
    pub camera_viewport_height: f32,    //4
    pub preload_margin_chunks: u8,      //1
    pub unload_margin_chunks: u8,       //1
    pub terrain_shape: TerrainShape,    //1
    pub water_threshold: u8,            //1
    pub global_octaves: u8,             //1
    pub global_frequency: f64,          //8
    pub global_persistence: f64,        //8
    pub global_lacunarity: f64,         //8
    pub global_detail_frequency: f64,   //8
    pub global_detail_weight: f64,      //8
    pub local_octaves: u8,              //1
    pub local_frequency: f64,           //8
    pub local_persistence: f64,         //8
    pub local_lacunarity: f64,          //8
    pub local_small_threshold: f64,     //8
    pub local_middle_threshold: f64,    //8
    pub local_seed_offset: f64,         //8
    pub noise_scale: f32,
    pub noise_strength: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            //seed + players
            seed: 33,
            //players: vec![(0, FantasyRace::WaterMages), (1, FantasyRace::FireMages), (2, FantasyRace::StoneMages)],

            //terrain
            terrain_size: 32,
            chunk_size: 4,
            tile_size: 1.0,
            terrain_shape: Jagged,

            //camera
            min_orthographic_scale: 0.1,
            max_orthographic_scale: 100.0,
            camera_viewport_height: 32.0 * 1.5,

            //renderer
            preload_margin_chunks: 2,
            unload_margin_chunks: 2,

            //global procedural thresholds
            water_threshold: 100,

            //global procedural generation (water/ground)
            global_octaves: 6,
            global_frequency: 8.0,
            global_persistence: 0.5,
            global_lacunarity: 2.0,
            global_detail_frequency: 0.075,
            global_detail_weight: 0.15,

            //local procedural generation (for each fantasy race)
            local_octaves: 4,
            local_frequency: 9.0,
            local_persistence: 0.55,
            local_lacunarity: 2.0,
            local_small_threshold: 0.45,
            local_middle_threshold: 0.62,
            local_seed_offset: 19.19,

            //borders
            noise_scale: 0.1,
            noise_strength: 8.0,
        }
    }
}

impl GameConfig {
    pub fn with_seed(seed: u32) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
#[repr(u8)]
pub enum TerrainShape {
    #[default]
    Round,
    Jagged,
}
