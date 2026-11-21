//! Сборка экрана: sidebar с вкладками + область контента + футер с
//! кнопками. Наполнение вкладок — в `tab_content.rs`.

use bevy::ecs::hierarchy::ChildOf;
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy_persistent::Persistent;

use super::change_tracking::{PendingSettingsChanges, SettingsApplied};
use super::tab_content::build_tab_content;
use super::tabs::SettingsTab;
use super::theme;
use super::widgets;
use crate::io::startup_loading::fonts_loader_plugin::Fonts;
use crate::io::startup_loading::localization_loader_plugin::{Language, Localization};
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::colors::{COLOR_PANEL_BG, COLOR_SIDEBAR_BG, COLOR_TAB_ACTIVE_BG, COLOR_TAB_BG};
use crate::ui::menus::AppState;

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
    let language = settings.language;
    let font = fonts.get(language);

    let panel = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            DespawnOnExit(AppState::SettingsMenu),
            BackgroundColor(COLOR_PANEL_BG),
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
        font,
        &localization,
        language,
        "settings_menu",
    );

    // 1) контейнеры вкладок + их содержимое
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
            font,
            &localization,
            tab,
            &settings,
        );
        entries.push(TabEntry {
            tab,
            container,
            button: Entity::PLACEHOLDER,
        });
    }

    // 2) кнопки вкладок
    for entry in entries.iter_mut() {
        let active = entry.tab == SettingsTab::World;
        entry.button = widgets::spawn_tab_button(
            &mut commands,
            sidebar,
            font,
            &localization,
            language,
            entry.tab.key(),
            active,
        );
    }

    // 3) переключение вешаем отдельно — обработчику нужен полный список кнопок,
    // который на шаге 2 ещё не был готов
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

    spawn_footer(&mut commands, sidebar, font, &localization, language);
}

fn spawn_footer(
    commands: &mut Commands,
    sidebar: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    language: Language,
) {
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

    let apply_btn = theme::spawn_button(commands, footer, font, localization, language, "apply", 2);
    commands.entity(apply_btn).observe(
        |_click: On<PointerClick>,
         mut commands: Commands,
         settings: Res<Persistent<SettingsConfig>>,
         mut pending: ResMut<PendingSettingsChanges>| {
            if let Err(err) = settings.persist() {
                eprintln!("не удалось сохранить настройки: {err:?}");
            }
            let changed = std::mem::take(&mut pending.0);
            if !changed.is_empty() {
                commands.trigger(SettingsApplied(changed));
            }
        },
    );

    let reset_btn = theme::spawn_button(
        commands,
        footer,
        font,
        localization,
        language,
        "reset_to_default",
        0,
    );
    commands.entity(reset_btn).observe(
        |_click: On<PointerClick>,
         mut settings: ResMut<Persistent<SettingsConfig>>,
         mut commands: Commands| {
            if let Err(err) = settings.set(SettingsConfig::default()) {
                eprintln!("could reset to defaults: {err:?}");
            };
            let mut hashset = HashSet::new();
            for tab in SettingsTab::ALL {
                hashset.insert(tab);
            }
            commands.trigger(SettingsApplied(hashset));
        },
    );

    let close_btn = theme::spawn_button(commands, footer, font, localization, language, "close", 1);
    commands.entity(close_btn).observe(
        |_click: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
            next_state.set(AppState::MainMenu);
        },
    );
}
