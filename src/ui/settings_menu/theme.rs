use crate::ui::colors::{
    COLOR_ACCENT_GOLD, COLOR_APPLY_BG, COLOR_BUTTON_BG, COLOR_DANGER_BG, COLOR_TEXT_PRIMARY,
};
use bevy::prelude::{
    AlignItems, BackgroundColor, BorderColor, BorderRadius, Bundle, ChildOf, Commands, Entity,
    FlexDirection, Font, FontSource, Handle, JustifyContent, Node, Text, TextColor, TextFont,
    UiRect, Val, default,
};
use bevy::text::FontSize;

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

pub fn spawn_row_label(commands: &mut Commands, row: Entity, font: &Handle<Font>, label: &str) {
    commands.spawn((
        Text::new(label),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(COLOR_TEXT_PRIMARY),
        ChildOf(row),
    ));
}

/// Заголовок секции (используется и в сайдбаре, и можно переиспользовать
/// внутри вкладки, если понадобится подзаголовок).
pub fn spawn_section_title(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    text: &str,
) {
    commands.spawn((
        Text::new(text),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(COLOR_ACCENT_GOLD),
        Node {
            margin: UiRect {
                bottom: Val::Px(14.0),
                ..default()
            },
            ..default()
        },
        ChildOf(parent),
    ));
}

/// Общий стиль для кнопок (Reset / Close / и т.п.) — набор компонентов,
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
