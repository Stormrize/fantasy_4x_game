//! Небольшие переиспользуемые "кирпичики" интерфейса: контейнер строки,
//! подпись строки, заголовок секции, кнопка со стилем.

use bevy::ecs::hierarchy::ChildOf;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::{Language, Localization};
use crate::ui::TranslationKey;
use crate::ui::colors::{
    COLOR_ACCENT_GOLD, COLOR_APPLY_BG, COLOR_BUTTON_BG, COLOR_DANGER_BG, COLOR_TEXT_PRIMARY,
};
/// Один "ряд" настройки: слева подпись, справа — сам виджет.
/// Возвращает entity ряда — в него потом добавляют подпись и контрол.
pub fn spawn_row_container(commands: &mut Commands, parent: Entity) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(0.0), Val::Px(6.0)),
                ..default()
            },
            ChildOf(parent),
        ))
        .id()
}

pub fn spawn_row_label(
    commands: &mut Commands,
    row: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    language: Language,
    key: &'static str,
) -> Entity {
    commands
        .spawn((
            Text::new(localization.get(key, language)),
            TextFont {
                font: FontSource::Handle(font.clone()),
                font_size: FontSize::Px(15.0),
                ..default()
            },
            TextColor(COLOR_TEXT_PRIMARY),
            TranslationKey(key),
            ChildOf(row),
        ))
        .id()
}

/// Заголовок секции (используется в сайдбаре).
pub fn spawn_section_title(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    language: Language,
    key: &'static str,
) -> Entity {
    commands
        .spawn((
            Text::new(localization.get(key, language)),
            TextFont {
                font: FontSource::Handle(font.clone()),
                font_size: FontSize::Px(22.0),
                ..default()
            },
            TextColor(COLOR_ACCENT_GOLD),
            TranslationKey(key),
            Node {
                margin: UiRect {
                    bottom: Val::Px(14.0),
                    ..default()
                },
                ..default()
            },
            ChildOf(parent),
        ))
        .id()
}

/// Кнопка с подписью (Apply/Reset/Close и т.п.) — сама только строит UI;
/// observer для клика вешает вызывающий код (см. `layout.rs`), поэтому
/// один и тот же хелпер годится для кнопок с разной логикой на клик.
pub fn spawn_button(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    language: Language,
    key: &'static str,
    color: u8,
) -> Entity {
    let btn = commands
        .spawn((Button, button_style(color), ChildOf(parent)))
        .id();

    commands.spawn((
        Text::new(localization.get(key, language)),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TranslationKey(key),
        TextColor(COLOR_TEXT_PRIMARY),
        ChildOf(btn),
    ));

    btn
}

/// Общий стиль для кнопок (Reset / Close / Apply) — набор компонентов,
/// который можно вставить как один bundle.
pub fn button_style(color: u8) -> impl Bundle {
    let bg = if color == 0 {
        COLOR_DANGER_BG
    } else if color == 1 {
        COLOR_BUTTON_BG
    } else {
        COLOR_APPLY_BG
    };

    (
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
            border: UiRect::all(Val::Px(1.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(COLOR_ACCENT_GOLD),
    )
}
