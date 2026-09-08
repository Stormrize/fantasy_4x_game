//! Обёртки над headless-виджетами из `bevy::ui_widgets`, которые сразу
//! стилизуют их под тёмное фэнтези и сами вешают `.observe()`, дёргающий
//! `Persistent<SettingsConfig>::update(...)`.
//!
//! Идея: у каждого builder'а последний аргумент — замыкание "как применить
//! новое значение к SettingsConfig". Чтобы добавить новое поле в меню,
//! обычно достаточно одного вызова одного из этих builder'ов в build.rs —
//! сюда лезть не нужно.

use bevy::ecs::hierarchy::{ChildOf, Children};
use bevy::prelude::*;
use bevy::ui::Checked;
use bevy::ui_widgets::{
    Activate, Button, Checkbox, RadioButton, RadioGroup, Slider, SliderOrientation, SliderRange,
    SliderStep, SliderThumb, SliderValue, TrackClick, ValueChange,
};
use std::fmt::Debug;

use super::theme;
use crate::ui::colors::{
    COLOR_ACCENT_CRIMSON, COLOR_ACCENT_GOLD, COLOR_TAB_ACTIVE_BG, COLOR_TAB_BG, COLOR_TEXT_MUTED,
    COLOR_TEXT_PRIMARY, COLOR_TRACK_BG,
};
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::SettingsConfig;
use bevy_persistent::Persistent;

fn percent_of(value: f32, min: f32, max: f32) -> f32 {
    if max > min {
        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    }
}

fn format_value(value: f32, decimals: u8, suffix: &str) -> String {
    format!("{:.*}{}", decimals as usize, value, suffix)
}

/// Строка с ползунком. `decimals`/`suffix` управляют тем, как форматируется
/// число справа (например `decimals: 0, suffix: "%"` -> "72%").
pub fn spawn_slider_row(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    label: &str,
    min: f32,
    max: f32,
    step: f32,
    decimals: u8,
    suffix: &'static str,
    initial: f32,
    on_change: impl Fn(&mut SettingsConfig, f32) + Send + Sync + 'static,
) -> Entity {
    let row = theme::spawn_row_container(commands, parent);
    theme::spawn_row_label(commands, row, font, label);

    let control = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            ChildOf(row),
        ))
        .id();

    let percent0 = percent_of(initial, min, max);

    let track = commands
        .spawn((
            Slider {
                track_click: TrackClick::Snap,
                orientation: SliderOrientation::Horizontal,
            },
            SliderRange::new(min, max),
            SliderStep(1.0),
            SliderValue(initial),
            Node {
                width: Val::Px(180.0),
                height: Val::Px(14.0),
                position_type: PositionType::Relative,
                border_radius: BorderRadius::all(Val::Px(7.0)),
                ..default()
            },
            BackgroundColor(COLOR_TRACK_BG),
            BorderColor::all(COLOR_ACCENT_GOLD),
            ChildOf(control),
        ))
        .id();

    let fill = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(percent0),
                border_radius: BorderRadius::all(Val::Px(7.0)),
                ..default()
            },
            BackgroundColor(COLOR_ACCENT_CRIMSON),
            ChildOf(track),
        ))
        .id();

    let thumb = commands
        .spawn((
            SliderThumb,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(percent0),
                top: Val::Px(-3.0),
                width: Val::Px(18.0),
                height: Val::Px(20.0),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(COLOR_ACCENT_GOLD),
            ChildOf(track),
        ))
        .id();

    let value_text = commands
        .spawn((
            Text::new(format_value(initial, decimals, suffix)),
            TextFont {
                font: FontSource::Handle(font.clone()),
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(COLOR_TEXT_MUTED),
            Node {
                width: Val::Px(56.0),
                ..default()
            },
            ChildOf(control),
        ))
        .id();

    // NB: сама библиотека не двигает ползунок визуально (это ответственность
    // стилиста) — поэтому и fill, и thumb, и подпись значения обновляем тут же.
    // Упрощение: положение thumb не учитывает его собственную ширину (небольшой
    // "нахлёст" на краях трека) — если важно, поправьте формулу percent_of
    // с поправкой на ширину thumb в пикселях.
    commands.entity(track).observe(
        move |trigger: On<ValueChange<f32>>,
              mut q_node: Query<&mut Node>,
              mut q_text: Query<&mut Text>,
              mut settings: ResMut<Persistent<SettingsConfig>>| {
            let percent = percent_of(trigger.value, min, max);
            if let Ok(mut node) = q_node.get_mut(fill) {
                node.width = Val::Percent(percent);
            }
            if let Ok(mut node) = q_node.get_mut(thumb) {
                node.left = Val::Percent(percent);
            }
            if let Ok(mut text) = q_text.get_mut(value_text) {
                *text = Text::new(format_value(trigger.value, decimals, suffix));
            }
            on_change(&mut settings, trigger.value);

            //let _ = settings.update(|cfg| on_change(cfg, trigger.value));
        },
    );

    row
}

/// Строка с чекбоксом (галочка отрисована отдельным квадратиком, чтобы не
/// зависеть от того, есть ли в шрифте глиф "✓").
pub fn spawn_checkbox_row(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    label: &str,
    initial: bool,
    on_change: impl Fn(&mut SettingsConfig, bool) + Send + Sync + 'static,
) -> Entity {
    let row = theme::spawn_row_container(commands, parent);
    theme::spawn_row_label(commands, row, font, label);

    let box_id = commands
        .spawn((
            Checkbox,
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                border: UiRect::all(Val::Px(1.5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(COLOR_TRACK_BG),
            BorderColor::all(COLOR_ACCENT_GOLD),
            ChildOf(row),
        ))
        .id();

    if initial {
        commands.entity(box_id).insert(Checked);
    }

    let mark_visibility = if initial {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    let mark_id = commands
        .spawn((
            Node {
                width: Val::Px(13.0),
                height: Val::Px(13.0),
                ..default()
            },
            BackgroundColor(COLOR_ACCENT_GOLD),
            mark_visibility,
            ChildOf(box_id),
        ))
        .id();

    commands.entity(box_id).observe(
        move |trigger: On<ValueChange<bool>>,
              mut commands: Commands,
              mut q_vis: Query<&mut Visibility>,
              mut settings: ResMut<Persistent<SettingsConfig>>| {
            if let Ok(mut vis) = q_vis.get_mut(mark_id) {
                if trigger.value {
                    *vis = Visibility::Visible;
                    commands.entity(box_id).insert(Checked);
                } else {
                    *vis = Visibility::Hidden;
                    commands.entity(box_id).remove::<Checked>();
                };
            }
            on_change(&mut settings, trigger.value);
            //let _ = settings.update(|cfg| on_change(cfg, trigger.value));
        },
    );
    row
}

/// Компонент-«привязка» на каждой radio-кнопке: что сделать с
/// SettingsConfig, если выбрали именно эту кнопку.
#[derive(Component)]
struct RadioOptionBinding(Box<dyn Fn(&mut SettingsConfig) + Send + Sync>);

/// Один обработчик на всю RadioGroup: `RadioGroup` в bevy::ui_widgets сам
/// НЕ управляет состоянием `Checked` у кнопок (см. документацию виджета) —
/// это явно отдано на откуп приложению, поэтому берём это на себя здесь.
fn on_radio_group_selection(
    trigger: On<ValueChange<Entity>>,
    q_children: Query<&Children>,
    q_bindings: Query<&RadioOptionBinding>,
    mut q_bg: Query<&mut BackgroundColor, With<RadioButton>>,
    mut commands: Commands,
    mut settings: ResMut<Persistent<SettingsConfig>>,
) {
    let group = trigger.source;
    let selected = trigger.value;

    if let Ok(children) = q_children.get(group) {
        for child in children.iter() {
            commands.entity(child).remove::<Checked>();
            if let Ok(mut bg) = q_bg.get_mut(child) {
                *bg = BackgroundColor(COLOR_TRACK_BG);
            }
        }
    }

    commands.entity(selected).insert(Checked);
    if let Ok(mut bg) = q_bg.get_mut(selected) {
        *bg = BackgroundColor(COLOR_ACCENT_CRIMSON);
    }

    if let Ok(binding) = q_bindings.get(selected) {
        binding.0(&mut settings);

        //let _ = settings.update(|cfg| binding.0(cfg));
    }
}

/// Строка с набором взаимоисключающих кнопок (замена выпадающему списку —
/// проще в реализации и не требует Popover/Menu из ui_widgets).
///
/// `T` — тип значения одной опции (enum вашей игры, кортеж и т.п.), должен
/// быть `Copy + PartialEq`.
pub fn spawn_radio_row<T>(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    label: &str,
    options: &[(&str, T)],
    current: T,
    on_change: impl Fn(&mut SettingsConfig, T) + Clone + Send + Sync + 'static,
) -> Entity
where
    T: Copy + PartialEq + Send + Sync + 'static,
{
    let row = theme::spawn_row_container(commands, parent);
    theme::spawn_row_label(commands, row, font, label);

    let group = commands
        .spawn((
            RadioGroup,
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                ..default()
            },
            ChildOf(row),
        ))
        .id();

    for &(text, value) in options {
        let selected = value == current;
        let on_change = on_change.clone();

        let btn = commands
            .spawn((
                RadioButton,
                RadioOptionBinding(Box::new(move |cfg: &mut SettingsConfig| {
                    on_change(cfg, value);
                })),
                Node {
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(if selected {
                    COLOR_ACCENT_CRIMSON
                } else {
                    COLOR_TRACK_BG
                }),
                BorderColor::all(COLOR_ACCENT_GOLD),
                ChildOf(group),
            ))
            .id();

        if selected {
            commands.entity(btn).insert(Checked);
        }

        commands.spawn((
            Text::new(text),
            TextFont {
                font: FontSource::Handle(font.clone()),
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(COLOR_TEXT_PRIMARY),
            ChildOf(btn),
        ));
    }

    commands.entity(group).observe(on_radio_group_selection);

    row
}

/// Кнопка-вкладка в сайдбаре. `on_click` получает только `&mut Commands`,
/// этого достаточно, чтобы переключить видимость контейнеров вкладок и
/// перекрасить кнопки — см. build.rs.
pub fn spawn_tab_button(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    label: &str,
    active: bool,
    on_click: impl Fn(&mut Commands) + Send + Sync + 'static,
) -> Entity {
    let btn = commands
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(if active {
                COLOR_TAB_ACTIVE_BG
            } else {
                COLOR_TAB_BG
            }),
            ChildOf(parent),
        ))
        .id();

    commands.spawn((
        Text::new(label),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextColor(COLOR_TEXT_PRIMARY),
        ChildOf(btn),
    ));

    commands
        .entity(btn)
        .observe(move |_trigger: On<Activate>, mut commands: Commands| {
            on_click(&mut commands);
        });

    btn
}
