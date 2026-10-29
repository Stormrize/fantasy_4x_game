use crate::io::in_game_loading::buildings_data_loader_plugin::BuildingsData;
use crate::io::in_game_loading::units_data_loader_plugin::UnitsData;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::world::components::{
    AiDifficulty, Food, Gold, Id, Mana, MapShape, MapSize, PlayerRace, PlayerType, Science,
};
use crate::world::ui_and_graphic_resources::data::visual_race_data::RaceAssets;
use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub id: Id,
    pub player_type: PlayerType,
}

impl Player {
    pub fn new(id: Id, player_type: PlayerType) -> Self {
        Self { id, player_type }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerData {
    pub id: Id,
    pub gold: Gold,
    pub food: Food,
    pub science: Science,
    pub current_mana: Mana,
    pub maximum_mana: Mana,
    pub player_race: PlayerRace,
}

impl PlayerData {
    pub fn new(id: Id, player_race: PlayerRace) -> Self {
        Self {
            id,
            gold: 10,
            food: 20,
            science: 10,
            current_mana: 20,
            maximum_mana: 40,
            player_race,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Players(pub Vec<PlayerData>);

#[derive(Resource, Debug)]
pub struct CameraConfig {
    pub min_orthographic_scale: f32,
    pub max_orthographic_scale: f32,
    pub camera_viewport_height: f32,
    pub edge_scroll_enabled: bool,
    pub edge_scroll_speed: f32,
    pub camera_pan_speed: f32,
    pub camera_zoom_speed: f32,
    pub invert_zoom: bool,
    pub shadow_maps_enabled: bool,
}

impl CameraConfig {
    pub fn new(settings: &SettingsConfig, game_config: PlayerConfig) -> Self {
        Self {
            min_orthographic_scale: settings.tile_size,
            max_orthographic_scale: game_config.map_size as u8 as f32 * 1.5,
            camera_viewport_height: settings.tile_size * 2.0,
            edge_scroll_enabled: settings.edge_scroll_enabled,
            edge_scroll_speed: settings.edge_scroll_speed,
            camera_pan_speed: settings.camera_pan_speed,
            camera_zoom_speed: settings.camera_zoom_speed,
            invert_zoom: settings.invert_zoom,
            // note
            shadow_maps_enabled: true,
        }
    }
}
#[derive(Debug)]
pub struct GenerationConfig {
    pub seed: u32,
    pub terrain_size: usize,
    pub players: Vec<(Id, PlayerRace)>,
    // note add struct to group all that
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
    pub fn new(player_config: PlayerConfig, local_player_config: &LocalPlayerConfig) -> Self {
        Self {
            seed: local_player_config.seed,
            terrain_size: player_config.map_size as usize,
            players: local_player_config.races.clone(),

            // todo change based on map shape
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
            terrain_shape: player_config.map_shape,
        }
    }
}

#[derive(Resource, Debug)]
pub struct RenderConfig {
    pub assets_registry: HashMap<PlayerRace, RaceAssets>,
    pub preload_margin_chunks: u8, //1
    pub unload_margin_chunks: u8,  //1
    pub terrain_size: usize,       //8
    pub chunk_size: usize,         //8
    pub tile_size: f32,            //4
}

impl RenderConfig {
    pub fn new(
        player_config: PlayerConfig,
        settings_config: &SettingsConfig,
        assets_registry: HashMap<PlayerRace, RaceAssets>,
    ) -> Self {
        Self {
            assets_registry,
            preload_margin_chunks: settings_config.preload_margin_chunks,
            unload_margin_chunks: settings_config.unload_margin_chunks,
            terrain_size: player_config.map_size as usize,
            chunk_size: settings_config.chunk_size,
            tile_size: settings_config.tile_size,
        }
    }
}

#[derive(Resource, Debug)]
pub struct GameLogicConfig {
    pub units_data: HashMap<PlayerRace, UnitsData>,
    pub buildings_data: HashMap<PlayerRace, BuildingsData>,
    pub players: Vec<Player>,
    pub terrain_size: usize,
    pub ai_difficulty: Option<AiDifficulty>,
    pub map_shape: MapShape,
    pub player: Player,
}

impl GameLogicConfig {
    pub fn new(
        units_data: HashMap<PlayerRace, UnitsData>,
        buildings_data: HashMap<PlayerRace, BuildingsData>,
        config: PlayerConfig,
        local_config: &LocalPlayerConfig,
    ) -> Self {
        let ai_difficulty = Some(local_config.ai_difficulty);

        let players: Vec<Player> = local_config
            .races
            .iter()
            .skip(1)
            .map(|(id, _)| Player::new(*id, PlayerType::Bot))
            .collect();

        let id = players[0].id;

        Self {
            units_data,
            buildings_data,
            players,
            terrain_size: config.map_size as usize,
            ai_difficulty,
            map_shape: config.map_shape,
            player: Player::new(id, PlayerType::Human),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct PlayerConfig {
    pub map_size: MapSize,
    pub map_shape: MapShape,
    pub id: Id,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            map_size: MapSize::Medium,
            map_shape: MapShape::Jagged,
            id: Id::default(),
        }
    }
}

impl PlayerConfig {
    pub fn new(map_size: MapSize, map_shape: MapShape, id: Id) -> Self {
        Self {
            map_size,
            map_shape,
            id,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct LocalPlayerConfig {
    pub seed: u32,
    pub races: Vec<(Id, PlayerRace)>,
    pub ai_difficulty: AiDifficulty,
}

impl Default for LocalPlayerConfig {
    fn default() -> Self {
        Self {
            seed: u32::MAX,
            races: Vec::new(),
            ai_difficulty: AiDifficulty::default(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct OnlinePlayerConfig {
    pub races: Vec<(Id, PlayerRace)>,
}

#[derive(Serialize, Debug, Clone)]
pub enum GameModeConfig {
    Local(LocalPlayerConfig),
    Multiplayer(OnlinePlayerConfig),
}
#[derive(Resource, Serialize, Debug, Clone)]
pub struct ConfigType {
    pub player_config: PlayerConfig,
    pub game_mode: GameModeConfig,
}
