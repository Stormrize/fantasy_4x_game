use crate::io::in_game_loading::buildings_data_loader_plugin::BuildingsData;
use crate::io::in_game_loading::units_data_loader_plugin::UnitsData;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::world::components::{AiDifficulty, Id, MapShape, MapSize, PlayerRace};
use crate::world::ui_and_graphic_resources::data::visual_race_data::RaceAssets;
use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CameraConfig {
    pub min_orthographic_scale: f32, //в зависимости от настроек
    pub max_orthographic_scale: f32, //в зависимости от настроек
    pub camera_viewport_height: f32, //в зависимости от настроек
    pub edge_scroll_enabled: bool,
    pub edge_scroll_speed: f32,
    pub camera_pan_speed: f32,
    pub camera_zoom_speed: f32,
    pub invert_zoom: bool,
    pub shadow_maps_enabled: bool,
}

impl CameraConfig {
    pub fn new(settings: &SettingsConfig, game_config: &PlayerConfig) -> Self {
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
#[derive(Resource, Debug, Default)]
pub struct GenerationConfig {
    pub seed: u32,
    pub terrain_size: usize,
    pub players: Vec<(Id, PlayerRace)>,
    // note добавить типа структуру которая все параметры упорядывает
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
    pub fn new(player_config: &PlayerConfig, local_player_config: &LocalPlayerConfig) -> Self {
        Self {
            seed: local_player_config.seed,
            terrain_size: player_config.map_size as usize,
            players: local_player_config.races.clone(),

            // todo менять в зависимости от карты
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

#[derive(Resource)]
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
        player_config: &PlayerConfig,
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


pub struct GameLogicConfig {
    pub units_data: HashMap<PlayerRace, UnitsData>,
    pub buildings_data: HashMap<PlayerRace, BuildingsData>,
    pub terrain_size: usize,
    pub player_id: Id
}

impl GameLogicConfig {
    pub fn new(
        units_data: HashMap<PlayerRace, UnitsData>,
        buildings_data: HashMap<PlayerRace, BuildingsData>,
        config: PlayerConfig,
    ) -> Self {
        Self {
            units_data,
            buildings_data,
            terrain_size: config.map_size as usize,
            player_id: config.id,
        }
    }
}

#[derive(Debug, Clone)]
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
            id: Id::default_local_id(),
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct OnlinePlayerConfig {
    pub player_id: Id,
    pub allowed_races: Vec<PlayerRace>,
}

#[derive(Debug)]
pub enum GameMode {
    Local(LocalPlayerConfig),
    Multiplayer(OnlinePlayerConfig),
}
#[derive(Resource, Debug)]
pub struct ConfigType {
    pub player_config: PlayerConfig,
    pub game_mode: GameMode,
}