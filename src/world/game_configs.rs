use crate::world::game_config::{GameMode, MapShape, PlayerRace};
use crate::world::ui_and_graphic_resources::player_registry::Id;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CameraConfig {
    pub min_orthographic_scale: f32, //в зависимости от настроек
    pub max_orthographic_scale: f32, //в зависимости от настроек
    pub camera_viewport_height: f32, //в зависимости от настроек
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            min_orthographic_scale: 0.1,
            max_orthographic_scale: 100.0,
            camera_viewport_height: 32.0 * 1.5,
        }
    }
}
#[derive(Debug)]
pub struct GenerationConfig {
    pub seed: u32,
    pub terrain_size: usize,
    pub players: Vec<(Id, PlayerRace)>,
    pub water_threshold: u8,
    pub global_octaves: u8,
    pub global_frequency: f64,        //8
    pub global_persistence: f64,      //8
    pub global_lacunarity: f64,       //8
    pub global_detail_frequency: f64, //8
    pub global_detail_weight: f64,    //8
    pub local_octaves: u8,            //1
    pub local_frequency: f64,         //8
    pub local_persistence: f64,       //8
    pub local_lacunarity: f64,        //8
    pub local_small_threshold: u8,    //8
    pub local_middle_threshold: u8,   //8
    pub local_seed_offset: f64,       //8
    pub noise_scale: f32,
    pub noise_strength: f32,
    pub terrain_shape: MapShape,
}

impl GenerationConfig {
    pub fn from_player_config(config: &LocalPlayerConfig) -> Self {
        let game_config = &config.game_config;

        Self {
            seed: config.seed,
            terrain_size: game_config.map_size as usize,
            players: config.races.clone(),

            //todo менять в зависимости от карты
            water_threshold: 100,

            global_octaves: 6,
            global_frequency: 8.0,
            global_persistence: 0.5,
            global_lacunarity: 2.0,
            global_detail_frequency: 0.075,
            global_detail_weight: 0.15,

            local_octaves: 4,
            local_frequency: 9.0,
            local_persistence: 0.55,
            local_lacunarity: 2.0,
            local_small_threshold: 115,
            local_middle_threshold: 158,
            local_seed_offset: 19.19,

            //borders
            noise_scale: 0.1,
            noise_strength: 8.0,
            terrain_shape: game_config.map_shape,
        }
    }
}

#[derive(Resource)]
pub struct RenderConfig {
    pub preload_margin_chunks: u8, //1
    pub unload_margin_chunks: u8,  //1
    pub terrain_size: usize,       //8
    pub chunk_size: usize,         //8
    pub tile_size: f32,            //4
}

impl RenderConfig {
    pub fn from_player_config(config: &LocalPlayerConfig) -> Self {
        Self {
            // todo добавить загрузку с настроек
            preload_margin_chunks: 2,
            unload_margin_chunks: 2,
            terrain_size: config.game_config.map_size as usize,
            chunk_size: 4,
            tile_size: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct SessionConfig {
    pub seed: u32,                              //4
    pub game_mode: GameMode,                    //1
    pub player: PlayerRace,                     //24
    pub current_players: Vec<(u8, PlayerRace)>, //24
}

#[derive(Debug, Resource)]
pub struct GameConfig {
    pub map_size: MapSize,
    pub map_shape: MapShape,
    pub player_race: PlayerRace,
}

impl GameConfig {
    pub fn new(map_size: MapSize, map_shape: MapShape, player_race: PlayerRace) -> Self {
        Self {
            map_size,
            map_shape,
            player_race,
        }
    }
}

#[derive(Debug, Resource)]
pub struct LocalPlayerConfig {
    pub game_config: GameConfig,
    pub seed: u32,
    pub races: Vec<(Id, PlayerRace)>,
    pub ai_difficulty: AiDifficulty,
}

pub struct OnlinePlayerConfig {
    pub game_config: GameConfig,
    pub player_id: Id,
    pub allowed_races: Vec<PlayerRace>,
}

#[derive(Resource)]
pub enum ConfigType {
    Local(LocalPlayerConfig),
    Multiplayer(OnlinePlayerConfig),
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MapSize {
    Tiny = 8,
    Small = 16,
    Medium = 32,
    Big = 64,
    Large = 128,
}

impl MapSize {
    pub fn len() -> usize {
        5
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AiDifficulty {
    Easy,
    Normal,
    Hard,
    Crazy,
}
impl AiDifficulty {
    pub fn len() -> usize {
        4
    }
}
