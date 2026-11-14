use bevy::input::mouse::MouseScrollUnit;
use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;

use crate::io::startup_loading::fonts_loader_plugin::Fonts;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::colors::COLOR_ACCENT_GOLD;
use crate::ui::offline_game_config::PendingGameSetup;
use crate::ui::offline_game_config::colors::COLOR_TRACK_BG;

const COLS: usize = 5;
const CENTER: i32 = (COLS / 2) as i32;
const STEP: f32 = 100.0 / COLS as f32;

#[derive(Component)]
pub struct NumberPicker {
    min: u8,
    max: u8,
    value: u8,
    accum: f32,
    apply: fn(&mut PendingGameSetup, usize),
}

/// style.
fn col_visuals(picker_value: u8, min: u8, max: u8, col: usize) -> (String, Color) {
    let offset = col as i32 - CENTER;
    let v = picker_value as i32 + offset;
    let text = if v >= min as i32 && v <= max as i32 {
        v.to_string()
    } else {
        String::new()
    };
    let alpha = if offset == 0 { 1.0 } else { 0.65 };
    (text, Color::srgba(1.0, 1.0, 1.0, alpha))
}

/// Ширина одного шага в ЛОГИЧЕСКИХ пикселях.
fn step_px(node: &ComputedNode) -> f32 {
    node.size().x * node.inverse_scale_factor() * STEP / 100.0
}

pub fn spawn_number_picker(
    commands: &mut Commands,
    parent: Entity,
    fonts: &Fonts,
    language: Language,
    range: (u8, u8),
    default_value: u8,
    apply: fn(&mut PendingGameSetup, usize),
) {
    let value = default_value.clamp(range.0, range.1);
    let row = commands
        .spawn((
            Node {
                width: Val::Percent(60.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

    let track: Entity = commands
        .spawn((
            ScrollArea,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(COLOR_TRACK_BG),
            BorderColor::all(COLOR_ACCENT_GOLD),
            NumberPicker {
                min: range.0,
                max: range.1,
                value,
                accum: 0.0,
                apply,
            },
            ChildOf(row),
        ))
        .id();

    // cols with numbers
    for col in 0..COLS {
        let (text, color) = col_visuals(value, range.0, range.1, col);
        commands.spawn((
            Node {
                width: Val::Percent(STEP),
                flex_shrink: 0.0,
                ..default()
            },
            TextLayout {
                justify: Justify::Center,
                ..default()
            },
            Text::new(text),
            TextFont {
                font_size: FontSize::Px(18.0),
                font: FontSource::Handle(fonts.get(language).clone()),
                ..default()
            },
            TextColor(color),
            Pickable::IGNORE,
            ChildOf(track),
        ));
    }

    // frame for chosen number
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0 - STEP / 2.0),
            top: Val::Px(0.0),
            width: Val::Percent(STEP),
            height: Val::Percent(100.0),
            border: UiRect::horizontal(Val::Px(1.0)),
            ..default()
        },
        BorderColor::all(COLOR_ACCENT_GOLD),
        Pickable::IGNORE,
        ChildOf(track),
    ));

    // Mouse wheel
    commands.entity(track).observe(
        move |mut ev: On<PointerScroll>,
              nodes: Query<&ComputedNode>,
              mut pickers: Query<&mut NumberPicker>,
              children: Query<&Children>,
              mut cols: Query<(&mut Text, &mut TextColor)>,
              mut pending: ResMut<PendingGameSetup>| {
            ev.propagate(false);

            let Ok(node) = nodes.get(track) else {
                return;
            };
            // Line: одна «строка» колеса = один шаг; Pixel: уже логические пиксели
            let k = match ev.unit {
                MouseScrollUnit::Line => step_px(node),
                MouseScrollUnit::Pixel => 1.0,
            };
            let delta = (-ev.y + ev.x) * k;
            scroll_picker(
                track,
                &nodes,
                delta,
                &mut pickers,
                &children,
                &mut cols,
                &mut pending,
            );
        },
    );

    // Swipe
    commands.entity(track).observe(
        move |mut ev: On<PointerDrag>,
              nodes: Query<&ComputedNode>,
              mut pickers: Query<&mut NumberPicker>,
              children: Query<&Children>,
              mut cols: Query<(&mut Text, &mut TextColor)>,
              mut pending: ResMut<PendingGameSetup>| {
            ev.propagate(false);
            // ev.delta — логические пиксели
            scroll_picker(
                track,
                &nodes,
                -ev.delta.x,
                &mut pickers,
                &children,
                &mut cols,
                &mut pending,
            );
        },
    );
}

/// delta_px (логические пиксели) > 0 увеличивает значение, < 0 уменьшает.
fn scroll_picker(
    track: Entity,
    nodes: &Query<&ComputedNode>,
    delta_px: f32,
    pickers: &mut Query<&mut NumberPicker>,
    children: &Query<&Children>,
    cols: &mut Query<(&mut Text, &mut TextColor)>,
    pending: &mut PendingGameSetup,
) {
    let Ok(node) = nodes.get(track) else {
        return;
    };
    let step = step_px(node);
    if step <= 0.0 {
        return;
    }
    let Ok(mut picker) = pickers.get_mut(track) else {
        return;
    };

    picker.accum += delta_px;
    let steps = (picker.accum / step).trunc();
    if steps == 0.0 {
        return;
    }
    picker.accum -= steps * step;

    let new_value =
        (picker.value as i32 + steps as i32).clamp(picker.min as i32, picker.max as i32) as u8;
    if new_value == picker.value {
        picker.accum = 0.0;
        return;
    }
    picker.value = new_value;

    // Первые COLS детей — колонки, последний — рамка
    if let Ok(kids) = children.get(track) {
        for (col, kid) in kids.iter().take(COLS).enumerate() {
            if let Ok((mut text, mut color)) = cols.get_mut(kid) {
                let (t, c) = col_visuals(picker.value, picker.min, picker.max, col);
                text.0 = t;
                color.0 = c;
            }
        }
    }

    (picker.apply)(pending, picker.value as usize);
}
