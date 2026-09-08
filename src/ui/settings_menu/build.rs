use crate::ui::colors::{COLOR_SIDEBAR_BG, COLOR_TAB_ACTIVE_BG, COLOR_TAB_BG, COLOR_TEXT_PRIMARY};
use bevy::ecs::hierarchy::ChildOf;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

use super::{theme, widgets};
use crate::ui::colors::COLOR_PANEL_BG;
use crate::ui::initial_loading::fonts_loader_plugin::fonts::Fonts;
use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::{
    ColorblindMode, GameSpeed, GraphicsQuality, SettingsConfig,
};
use crate::ui::menus::AppState;
use crate::ui::update_ui_system::TranslationKey;
use bevy_persistent::Persistent;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SettingsTab {
    World,
    Localization,
    Sound,
    Graphics,
    Gameplay,
    Camera,
    Interface,
}

impl SettingsTab {
    const ALL: [SettingsTab; 7] = [
        Self::World,
        Self::Localization,
        Self::Sound,
        Self::Graphics,
        Self::Gameplay,
        Self::Camera,
        Self::Interface,
    ];

    fn title(self, localization: &Localization, language: Language) -> &str {
        match self {
            Self::World => localization.get("world", language),
            Self::Localization => localization.get("localization", language),
            Self::Sound => localization.get("voice", language),
            Self::Graphics => localization.get("graphic", language),
            Self::Gameplay => localization.get("gameplay", language),
            Self::Camera => localization.get("camera", language),
            Self::Interface => localization.get("interface", language),
        }
    }
}
fn tab_content_node(display: Display) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(4.0),
        display,
        ..default()
    }
}

#[derive(Clone, Copy)]
struct TabEntry {
    tab: SettingsTab,
    container: Entity,
    button: Entity,
}

pub fn spawn_settings_menu(
    mut commands: Commands,
    fonts: Res<Fonts>,
    localization: Res<Localization>,
    settings: ResMut<Persistent<SettingsConfig>>,
) {
    let font = fonts.get(settings.language);
    let panel = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                //position_type: PositionType::Absolute,
                //justify_content: JustifyContent::Center,
                //align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(AppState::SettingsMenu),
            BackgroundColor(COLOR_PANEL_BG),
            //BackgroundColor(theme::COLOR_OVERLAY),
        ))
        .id();

    let sidebar = commands
        .spawn((
            Node {
                width: Val::Px(220.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(COLOR_SIDEBAR_BG),
            ChildOf(panel),
        ))
        .id();

    let content = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(2.0),
                ..default()
            },
            ChildOf(panel),
        ))
        .id();

    theme::spawn_section_title(
        &mut commands,
        sidebar,
        &font,
        localization.get("settings", settings.language),
    );

    // 1) контейнеры вкладок + их содержимое.
    let mut entries: Vec<TabEntry> = Vec::new();
    for tab in SettingsTab::ALL {
        let display = if tab == SettingsTab::World {
            Display::Flex
        } else {
            Display::None
        };
        let container = commands
            .spawn((tab_content_node(display), ChildOf(content)))
            .id();
        build_tab_content(
            &mut commands,
            container,
            &font,
            tab,
            &settings,
            &localization,
        );
        entries.push(TabEntry {
            tab,
            container,
            button: Entity::PLACEHOLDER,
        });
    }

    // 2) кнопки вкладок (пока без обработчика — он ссылается на весь список).
    for entry in entries.iter_mut() {
        let active = entry.tab == SettingsTab::World;
        let button = widgets::spawn_tab_button(
            &mut commands,
            sidebar,
            &font,
            entry.tab.title(&localization, settings.language),
            active,
            |_| {},
        );
        entry.button = button;
    }

    // 3) теперь у каждой кнопки есть полный список — вешаем переключение.
    for i in 0..entries.len() {
        let target_tab = entries[i].tab;
        let button_id = entries[i].button;
        let snapshot = entries.clone();
        commands.entity(button_id).observe(
            move |_trigger: On<Activate>, mut commands: Commands| {
                for e in &snapshot {
                    let is_target = e.tab == target_tab;
                    commands
                        .entity(e.container)
                        .insert(tab_content_node(if is_target {
                            Display::Flex
                        } else {
                            Display::None
                        }));
                    commands
                        .entity(e.button)
                        .insert(BackgroundColor(if is_target {
                            COLOR_TAB_ACTIVE_BG
                        } else {
                            COLOR_TAB_BG
                        }));
                }
            },
        );
    }

    let footer = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                margin: UiRect {
                    top: Val::Auto,
                    ..default()
                },
                ..default()
            },
            ChildOf(sidebar),
        ))
        .id();

    let apply_button = commands
        .spawn((Button, theme::button_style(2), ChildOf(footer)))
        .id();
    commands.spawn((
        Text::new(localization.get("apply", settings.language)),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TranslationKey("apply"),
        TextColor(COLOR_TEXT_PRIMARY),
        ChildOf(apply_button),
    ));

    commands.entity(apply_button).observe(
        |_trigger: On<Pointer<Click>>, settings: Res<Persistent<SettingsConfig>>| {
            if let Err(err) = settings.persist() {
                eprintln!("crazy, could not save. Error message: {:?}", err);
            }
        },
    );

    let reset_btn = commands
        .spawn((Button, theme::button_style(0), ChildOf(footer)))
        .id();
    commands.spawn((
        Text::new(localization.get("reset_to_default", settings.language)),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TranslationKey("reset_to_default"),
        TextColor(COLOR_TEXT_PRIMARY),
        ChildOf(reset_btn),
    ));

    commands.entity(reset_btn).observe(
        |_trigger: On<Pointer<Click>>, mut settings: ResMut<Persistent<SettingsConfig>>| {
            if let Err(err) = settings.revert_to_default() {
                eprintln!(
                    "crazy, could not revert to default. Error message: {:?}",
                    err
                );
            }
        },
    );

    let close_btn = commands
        .spawn((Button, theme::button_style(1), ChildOf(footer)))
        .id();
    commands.spawn((
        Text::new(localization.get("close", settings.language)),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TranslationKey("close"),
        TextColor(COLOR_TEXT_PRIMARY),
        ChildOf(close_btn),
    ));
    commands.entity(close_btn).observe(
        |_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
            next_state.set(AppState::MainMenu);
        },
    );
}

fn build_tab_content(
    commands: &mut Commands,
    container: Entity,
    font: &Handle<Font>,
    tab: SettingsTab,
    settings: &Persistent<SettingsConfig>,
    localization: &Localization,
) {
    match tab {
        SettingsTab::World => {
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("chunk_size", settings.language),
                8.0,
                64.0,
                8.0,
                0,
                "",
                settings.chunk_size as f32,
                |cfg, v| cfg.chunk_size = v.round() as usize,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("tile_size", settings.language),
                8.0,
                128.0,
                8.0,
                0,
                "",
                settings.tile_size,
                |cfg, v| cfg.tile_size = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("preload_margin_chunks", settings.language),
                0.0,
                8.0,
                1.0,
                0,
                "",
                settings.preload_margin_chunks as f32,
                |cfg, v| cfg.preload_margin_chunks = v.round() as u8,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("unload_margin_chunks", settings.language),
                0.0,
                8.0,
                1.0,
                0,
                "",
                settings.unload_margin_chunks as f32,
                |cfg, v| cfg.unload_margin_chunks = v.round() as u8,
            );
        }
        SettingsTab::Localization => {
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization.get("language", settings.language),
                &[
                    ("English", Language::En),
                    ("Русский", Language::Ru),
                    ("Deutsch", Language::De),
                ],
                settings.language,
                |cfg, v| cfg.language = v,
            );
        }
        SettingsTab::Sound => {
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("overall_volume", settings.language),
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.overall_volume as f32,
                |cfg, v| cfg.overall_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("music_volume", settings.language),
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.music_volume as f32,
                |cfg, v| cfg.music_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("sound_effects_volume", settings.language),
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.sound_effects_volume as f32,
                |cfg, v| cfg.sound_effects_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("ambience_volume", settings.language),
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.ambience_volume as f32,
                |cfg, v| cfg.ambience_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("ui_sounds_volume", settings.language),
                0.0,
                100.0,
                1.0,
                0,
                "%",
                settings.ui_sounds_volume as f32,
                |cfg, v| cfg.ui_sounds_volume = (v.round() as u8).min(100),
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("mute_when_unfocused", settings.language),
                settings.mute_when_unfocused,
                |cfg, v| cfg.mute_when_unfocused = v,
            );
        }
        SettingsTab::Graphics => {
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("fullscreen", settings.language),
                settings.fullscreen,
                |cfg, v| cfg.fullscreen = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("vsync", settings.language),
                settings.vsync,
                |cfg, v| cfg.vsync = v,
            );
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization.get("resolution", settings.language),
                &[
                    ("1280x720", (1280u32, 720u32)),
                    ("1600x900", (1600, 900)),
                    ("1920x1080", (1920, 1080)),
                    ("2560x1440", (2560, 1440)),
                ],
                (settings.resolution_width, settings.resolution_height),
                |cfg, (w, h)| {
                    cfg.resolution_width = w;
                    cfg.resolution_height = h;
                },
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("ui_scale", settings.language),
                0.5,
                2.0,
                0.05,
                2,
                "x",
                settings.ui_scale,
                |cfg, v| cfg.ui_scale = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("show_fps", settings.language),
                settings.show_fps,
                |cfg, v| cfg.show_fps = v,
            );
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization.get("graphics_quality", settings.language),
                &[
                    ("Низкое", GraphicsQuality::Low),
                    ("Среднее", GraphicsQuality::Medium),
                    ("Высокое", GraphicsQuality::High),
                ],
                settings.graphics_quality,
                |cfg, v| cfg.graphics_quality = v,
            );
        }
        SettingsTab::Gameplay => {
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization.get("game_speed", settings.language),
                &[
                    ("Обычно", GameSpeed::Standard),
                    ("Быстро", GameSpeed::Quick),
                ],
                settings.game_speed,
                |cfg, v| cfg.game_speed = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("autosave_interval_minutes", settings.language),
                1.0,
                60.0,
                1.0,
                0,
                "",
                settings.autosave_interval_minutes as f32,
                |cfg, v| cfg.autosave_interval_minutes = v.round() as u16,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("turn_notifications_enabled", settings.language),
                settings.turn_notifications_enabled,
                |cfg, v| cfg.turn_notifications_enabled = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("show_tutorial_hints", settings.language),
                settings.show_tutorial_hints,
                |cfg, v| cfg.show_tutorial_hints = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("unit_move_animation_speed", settings.language),
                0.25,
                3.0,
                0.25,
                2,
                "x",
                settings.unit_move_animation_speed,
                |cfg, v| cfg.unit_move_animation_speed = v,
            );
        }
        SettingsTab::Camera => {
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("edge_scroll_enabled", settings.language),
                settings.edge_scroll_enabled,
                |cfg, v| cfg.edge_scroll_enabled = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("edge_scroll_speed", settings.language),
                0.0,
                2000.0,
                50.0,
                0,
                "",
                settings.edge_scroll_speed,
                |cfg, v| cfg.edge_scroll_speed = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("camera_pan_speed", settings.language),
                0.0,
                2000.0,
                50.0,
                0,
                "",
                settings.camera_pan_speed,
                |cfg, v| cfg.camera_pan_speed = v,
            );
            widgets::spawn_slider_row(
                commands,
                container,
                font,
                localization.get("camera_zoom_speed", settings.language),
                0.1,
                5.0,
                0.1,
                2,
                "x",
                settings.camera_zoom_speed,
                |cfg, v| cfg.camera_zoom_speed = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("invert_zoom", settings.language),
                settings.invert_zoom,
                |cfg, v| cfg.invert_zoom = v,
            );
        }
        SettingsTab::Interface => {
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("show_grid_overlay", settings.language),
                settings.show_grid_overlay,
                |cfg, v| cfg.show_grid_overlay = v,
            );
            widgets::spawn_checkbox_row(
                commands,
                container,
                font,
                localization.get("show_resource_icons", settings.language),
                settings.show_resource_icons,
                |cfg, v| cfg.show_resource_icons = v,
            );
            widgets::spawn_radio_row(
                commands,
                container,
                font,
                localization.get("colorblind_mode", settings.language),
                &[
                    ("Выкл", ColorblindMode::None),
                    ("Протанопия", ColorblindMode::Protanopia),
                    ("Дейтеранопия", ColorblindMode::Deuteranopia),
                    ("Тританопия", ColorblindMode::Tritanopia),
                ],
                settings.colorblind_mode,
                |cfg, v| cfg.colorblind_mode = v,
            );
        }
    }
}
