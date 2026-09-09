//! Наполнение каждой вкладки конкретными виджетами.

use bevy::prelude::*;
use bevy_persistent::Persistent;

use super::tabs::SettingsTab;
use super::widgets::{self, OptionLabel};
use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::{
    ColorblindMode, GameSpeed, GraphicsQuality, SettingsConfig,
};

pub fn build_tab_content(
    commands: &mut Commands,
    container: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    tab: SettingsTab,
    settings: &Persistent<SettingsConfig>,
) {
    let language = settings.language;

    match tab {
        SettingsTab::World => {
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "chunk_size",
                8.0,
                64.0,
                8.0,
                0,
                "",
                settings.chunk_size as f32,
                tab,
                |cfg, v| cfg.chunk_size = v.round() as usize,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "tile_size",
                8.0,
                128.0,
                8.0,
                0,
                "",
                settings.tile_size,
                tab,
                |cfg, v| cfg.tile_size = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "preload_margin_chunks",
                0.0,
                8.0,
                1.0,
                0,
                "",
                settings.preload_margin_chunks as f32,
                tab,
                |cfg, v| cfg.preload_margin_chunks = v.round() as u8,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "unload_margin_chunks",
                0.0,
                8.0,
                1.0,
                0,
                "",
                settings.unload_margin_chunks as f32,
                tab,
                |cfg, v| cfg.unload_margin_chunks = v.round() as u8,
            );
        }
        SettingsTab::Localization => {
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization,
                language,
                "language",
                &[
                    (OptionLabel::Literal("English"), Language::En),
                    (OptionLabel::Literal("Русский"), Language::Ru),
                    (OptionLabel::Literal("Deutsch"), Language::De),
                ],
                settings.language,
                tab,
                |cfg, v| cfg.language = v,
            );
        }
        SettingsTab::Sound => {
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "overall_volume",
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.overall_volume as f32,
                tab,
                |cfg, v| cfg.overall_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "music_volume",
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.music_volume as f32,
                tab,
                |cfg, v| cfg.music_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "sound_effects_volume",
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.sound_effects_volume as f32,
                tab,
                |cfg, v| cfg.sound_effects_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "ambience_volume",
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.ambience_volume as f32,
                tab,
                |cfg, v| cfg.ambience_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "ui_sounds_volume",
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.ui_sounds_volume as f32,
                tab,
                |cfg, v| cfg.ui_sounds_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "mute_when_unfocused",
                settings.mute_when_unfocused,
                tab,
                |cfg, v| cfg.mute_when_unfocused = v,
            );
        }
        SettingsTab::Graphics => {
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "fullscreen",
                settings.fullscreen,
                tab,
                |cfg, v| cfg.fullscreen = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "vsync",
                settings.vsync,
                tab,
                |cfg, v| cfg.vsync = v,
            );
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization,
                language,
                "resolution",
                &[
                    (OptionLabel::Literal("1280x720"), (1280u32, 720u32)),
                    (OptionLabel::Literal("1600x900"), (1600, 900)),
                    (OptionLabel::Literal("1920x1080"), (1920, 1080)),
                    (OptionLabel::Literal("2560x1440"), (2560, 1440)),
                ],
                (settings.resolution_width, settings.resolution_height),
                tab,
                |cfg, (w, h)| {
                    cfg.resolution_width = w;
                    cfg.resolution_height = h;
                },
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "ui_scale",
                0.5,
                2.0,
                0.05,
                2,
                "x",
                settings.ui_scale,
                tab,
                |cfg, v| cfg.ui_scale = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "show_fps",
                settings.show_fps,
                tab,
                |cfg, v| cfg.show_fps = v,
            );
            // ПРЕДПОЛОЖЕНИЕ: ключи "graphics_quality_low/medium/high" я придумал —
            // раньше эти подписи были захардкожены по-русски прямо в коде.
            // Поправь под свои реальные ключи локализации, если называешь иначе.
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization,
                language,
                "graphics_quality",
                &[
                    (
                        OptionLabel::Key("graphics_quality_low"),
                        GraphicsQuality::Low,
                    ),
                    (
                        OptionLabel::Key("graphics_quality_medium"),
                        GraphicsQuality::Medium,
                    ),
                    (
                        OptionLabel::Key("graphics_quality_high"),
                        GraphicsQuality::High,
                    ),
                ],
                settings.graphics_quality,
                tab,
                |cfg, v| cfg.graphics_quality = v,
            );
        }
        SettingsTab::Gameplay => {
            // ПРЕДПОЛОЖЕНИЕ: ключи "game_speed_standard/quick" придуманы — раньше
            // было захардкожено по-русски.
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization,
                language,
                "game_speed",
                &[
                    (OptionLabel::Key("game_speed_standard"), GameSpeed::Standard),
                    (OptionLabel::Key("game_speed_quick"), GameSpeed::Quick),
                ],
                settings.game_speed,
                tab,
                |cfg, v| cfg.game_speed = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "autosave_interval_minutes",
                1.0,
                60.0,
                1.0,
                0,
                "",
                settings.autosave_interval_minutes as f32,
                tab,
                |cfg, v| cfg.autosave_interval_minutes = v.round() as u16,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "turn_notifications_enabled",
                settings.turn_notifications_enabled,
                tab,
                |cfg, v| cfg.turn_notifications_enabled = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "show_tutorial_hints",
                settings.show_tutorial_hints,
                tab,
                |cfg, v| cfg.show_tutorial_hints = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "unit_move_animation_speed",
                0.25,
                3.0,
                0.25,
                2,
                "x",
                settings.unit_move_animation_speed,
                tab,
                |cfg, v| cfg.unit_move_animation_speed = v,
            );
        }
        SettingsTab::Camera => {
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "edge_scroll_enabled",
                settings.edge_scroll_enabled,
                tab,
                |cfg, v| cfg.edge_scroll_enabled = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "edge_scroll_speed",
                0.0,
                2000.0,
                50.0,
                0,
                "",
                settings.edge_scroll_speed,
                tab,
                |cfg, v| cfg.edge_scroll_speed = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "camera_pan_speed",
                0.0,
                2000.0,
                50.0,
                0,
                "",
                settings.camera_pan_speed,
                tab,
                |cfg, v| cfg.camera_pan_speed = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization,
                language,
                "camera_zoom_speed",
                0.1,
                5.0,
                0.1,
                2,
                "x",
                settings.camera_zoom_speed,
                tab,
                |cfg, v| cfg.camera_zoom_speed = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "invert_zoom",
                settings.invert_zoom,
                tab,
                |cfg, v| cfg.invert_zoom = v,
            );
        }
        SettingsTab::Interface => {
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "show_grid_overlay",
                settings.show_grid_overlay,
                tab,
                |cfg, v| cfg.show_grid_overlay = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization,
                language,
                "show_resource_icons",
                settings.show_resource_icons,
                tab,
                |cfg, v| cfg.show_resource_icons = v,
            );
            // ПРЕДПОЛОЖЕНИЕ: ключи "colorblind_mode_*" придуманы — раньше было
            // захардкожено по-русски.
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization,
                language,
                "colorblind_mode",
                &[
                    (
                        OptionLabel::Key("colorblind_mode_none"),
                        ColorblindMode::None,
                    ),
                    (
                        OptionLabel::Key("colorblind_mode_protanopia"),
                        ColorblindMode::Protanopia,
                    ),
                    (
                        OptionLabel::Key("colorblind_mode_deuteranopia"),
                        ColorblindMode::Deuteranopia,
                    ),
                    (
                        OptionLabel::Key("colorblind_mode_tritanopia"),
                        ColorblindMode::Tritanopia,
                    ),
                ],
                settings.colorblind_mode,
                tab,
                |cfg, v| cfg.colorblind_mode = v,
            );
        }
    }
}
