use crate::ui::initial_loading::localization_loader_plugin::public_api::Language;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SettingsConfig {
    // ---------- World ----------
    pub chunk_size: usize,
    pub tile_size: f32,
    pub preload_margin_chunks: u8,
    pub unload_margin_chunks: u8,

    // ---------- Localization ----------
    pub language: Language,

    // ---------- Voice ----------
    pub overall_volume: u8,
    pub music_volume: u8,
    pub sound_effects_volume: u8,
    pub ambience_volume: u8,
    pub ui_sounds_volume: u8,
    pub mute_when_unfocused: bool,

    // ---------- Graphic ----------
    pub fullscreen: bool,
    pub vsync: bool,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub ui_scale: f32,
    pub show_fps: bool,
    pub graphics_quality: GraphicsQuality,

    // ---------- Gameplay ----------
    pub game_speed: GameSpeed,
    pub autosave_interval_minutes: u16,
    pub turn_notifications_enabled: bool,
    pub show_tutorial_hints: bool,
    pub unit_move_animation_speed: f32,

    // ---------- Camera / управление ----------
    pub edge_scroll_enabled: bool,
    pub edge_scroll_speed: f32,
    pub camera_pan_speed: f32,
    pub camera_zoom_speed: f32,
    pub invert_zoom: bool,

    // ---------- UI / доступность ----------
    pub show_grid_overlay: bool,
    pub show_resource_icons: bool,
    pub colorblind_mode: ColorblindMode,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            chunk_size: 4,
            tile_size: 1.0,
            preload_margin_chunks: 2,
            unload_margin_chunks: 3,
            language: Language::En,

            overall_volume: 80,
            music_volume: 70,
            sound_effects_volume: 80,
            ambience_volume: 60,
            ui_sounds_volume: 80,
            mute_when_unfocused: true,

            fullscreen: false,
            vsync: true,
            resolution_width: 1920,
            resolution_height: 1080,
            ui_scale: 1.0,
            show_fps: false,
            graphics_quality: GraphicsQuality::default(),

            game_speed: GameSpeed::default(),
            autosave_interval_minutes: 5,
            turn_notifications_enabled: true,
            show_tutorial_hints: true,
            unit_move_animation_speed: 1.0,

            edge_scroll_enabled: true,
            edge_scroll_speed: 800.0,
            camera_pan_speed: 600.0,
            camera_zoom_speed: 1.0,
            invert_zoom: false,

            show_grid_overlay: false,
            show_resource_icons: true,
            colorblind_mode: ColorblindMode::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphicsQuality {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameSpeed {
    Quick,
    #[default]
    Standard,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorblindMode {
    #[default]
    None,
    Protanopia,
    Deuteranopia,
    Tritanopia,
}
