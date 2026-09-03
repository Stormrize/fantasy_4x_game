use crate::ui::initial_loading::localization_loader_plugin::public_api::Language;
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::{
    ColorblindMode, GameSpeed, GraphicsQuality, SettingsConfig,
};
use bevy_persistent::Persistent;

/// Общая точка изменения настроек: применяет мутацию и сразу сохраняет
/// на диск. Все публичные сеттеры ниже — тонкие обёртки над этой функцией,
/// чтобы не дублировать `.expect(...)` в каждой из них.
fn update(settings: &mut Persistent<SettingsConfig>, apply: impl Fn(&mut SettingsConfig)) {
    settings
        .update(apply)
        .expect("Failed to persist settings config");
}

// ---------- Звук ----------

pub fn set_overall_volume(settings: &mut Persistent<SettingsConfig>, value: u8) {
    update(settings, |cfg| cfg.overall_volume = value.min(100));
}

pub fn set_music_volume(settings: &mut Persistent<SettingsConfig>, value: u8) {
    update(settings, |cfg| cfg.music_volume = value.min(100));
}

pub fn set_sound_effects_volume(settings: &mut Persistent<SettingsConfig>, value: u8) {
    update(settings, |cfg| cfg.sound_effects_volume = value.min(100));
}

// ---------- Локализация ----------

pub fn set_language(settings: &mut Persistent<SettingsConfig>, language: Language) {
    update(settings, |cfg| cfg.language = language);
}

// ---------- Мир / чанки ----------

pub fn set_chunk_size(settings: &mut Persistent<SettingsConfig>, value: usize) {
    update(settings, |cfg| cfg.chunk_size = value);
}

pub fn set_preload_margin_chunks(settings: &mut Persistent<SettingsConfig>, value: u8) {
    update(settings, |cfg| cfg.preload_margin_chunks = value);
}

// ---------- Графика (примеры на будущее) ----------

pub fn set_fullscreen(settings: &mut Persistent<SettingsConfig>, value: bool) {
    update(settings, |cfg| cfg.fullscreen = value);
}

pub fn set_graphics_quality(settings: &mut Persistent<SettingsConfig>, quality: GraphicsQuality) {
    update(settings, |cfg| cfg.graphics_quality = quality);
}

// ---------- Геймплей (примеры на будущее) ----------
pub fn set_game_speed(settings: &mut Persistent<SettingsConfig>, speed: GameSpeed) {
    update(settings, |cfg| cfg.game_speed = speed);
}

pub fn set_colorblind_mode(settings: &mut Persistent<SettingsConfig>, mode: ColorblindMode) {
    update(settings, |cfg| cfg.colorblind_mode = mode);
}

// ---------- Сброс ----------

pub fn reset_to_defaults(settings: &mut Persistent<SettingsConfig>) {
    update(settings, |cfg| *cfg = SettingsConfig::default());
}

// Остальные сеттеры (tile_size, unload_margin_chunks, vsync, resolution,
// ui_scale, autosave_interval_minutes, camera_*, show_grid_overlay и т.д.)
// добавляются по этому же шаблону — одна строка на поле.
