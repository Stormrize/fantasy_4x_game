use crate::world::game_config::MapShape::Jagged;
use bevy::prelude::Resource;
use smol_str::SmolStr;
use std::collections::HashMap;

#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    //seed + players
    pub seed: u32, //4
    pub game_mode: GameMode,
    pub player: PlayerRace,
    pub current_players: Vec<(u8, PlayerRace)>, //?

    //terrain
    pub terrain_size: usize, //8
    pub chunk_size: usize,   //8
    pub tile_size: f32,      //4

    //camera_plugin
    pub min_orthographic_scale: f32, //4
    pub max_orthographic_scale: f32, //4
    pub camera_viewport_height: f32, //4

    //renderer
    pub preload_margin_chunks: u8, //1
    pub unload_margin_chunks: u8,  //1

    pub terrain_shape: MapShape, //1

    //global procedural thresholds
    pub water_threshold: u8, //1

    //global procedural generation (water/ground)
    pub global_octaves: u8,           //1
    pub global_frequency: f64,        //8
    pub global_persistence: f64,      //8
    pub global_lacunarity: f64,       //8
    pub global_detail_frequency: f64, //8
    pub global_detail_weight: f64,    //8

    //local procedural generation (for each fantasy race)
    pub local_octaves: u8,          //1
    pub local_frequency: f64,       //8
    pub local_persistence: f64,     //8
    pub local_lacunarity: f64,      //8
    pub local_small_threshold: u8,  //8
    pub local_middle_threshold: u8, //8
    pub local_seed_offset: f64,     //8

    //borders
    pub noise_scale: f32,
    pub noise_strength: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            //seed + players
            seed: 33,
            game_mode: GameMode::Local,
            player: PlayerRace::new(SmolStr::from("WaterMages")),
            current_players: vec![
                (0, PlayerRace::new(SmolStr::from("WaterMages"))),
                (1, PlayerRace::new(SmolStr::from("FireMages"))),
                (2, PlayerRace::new(SmolStr::from("StoneMages"))),
            ],

            //terrain
            terrain_size: 32,
            chunk_size: 4,
            tile_size: 1.0,
            terrain_shape: Jagged,

            //camera_plugin
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
            local_small_threshold: 115,  //0.45
            local_middle_threshold: 158, //0.62
            local_seed_offset: 19.19,

            //borders
            noise_scale: 0.1,
            noise_strength: 8.0,
        }
    }
}

impl GameConfig {
    pub fn new(
        seed: u32,
        game_mode: GameMode,
        player: PlayerRace,
        current_players: Vec<(u8, PlayerRace)>,
        terrain_size: usize,
        chunk_size: usize,
        tile_size: f32,
        min_orthographic_scale: f32,
        max_orthographic_scale: f32,
        camera_viewport_height: f32,
        preload_margin_chunks: u8,
        unload_margin_chunks: u8,
        water_threshold: u8,
    ) -> Self {
        Self {
            seed,
            game_mode,
            player,
            current_players,
            terrain_size,
            chunk_size,
            tile_size,
            min_orthographic_scale,
            max_orthographic_scale,
            camera_viewport_height,
            preload_margin_chunks,
            unload_margin_chunks,
            water_threshold,
            ..Default::default()
        }
    }

    pub fn with_seed(seed: u32) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }
}

//---------------------------------------------------------------------------------------------------------------------------//

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayerRace(pub SmolStr);

impl PlayerRace {
    pub fn new(name: SmolStr) -> Self {
        Self(name)
    }
}

impl Default for PlayerRace {
    fn default() -> Self {
        Self(SmolStr::from(""))
    }
}
//---------------------------------------------------------------------------------------------------------------------------//

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum GameMode {
    Multiplayer,
    Local,
}

#[derive(Clone, Copy, Debug, Default)]
#[allow(dead_code)]
#[repr(u8)]
pub enum MapShape {
    #[default]
    Round,
    Jagged,
}

impl MapShape {
    pub fn len() -> usize {
        2
    }
}
