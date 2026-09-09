//! Переиспользуемые виджеты настроек: слайдер, чекбокс, радио-группа,
//! кнопка вкладки. Каждый виджет сам пишет значение в `SettingsConfig` и
//! помечает свою вкладку как изменённую — что происходит с этой пометкой
//! дальше, см. `change_tracking.rs`.

use bevy::ecs::hierarchy::{ChildOf, Children};
use bevy::prelude::*;
use bevy::ui::Checked;
use bevy::ui_widgets::{
    Button, Checkbox, RadioButton, RadioGroup, Slider, SliderOrientation, SliderRange, SliderStep,
    SliderThumb, SliderValue, TrackClick, ValueChange,
};

use bevy_persistent::Persistent;

use super::change_tracking::PendingSettingsChanges;
use super::tabs::SettingsTab;
use super::theme;
use crate::ui::colors::{
    COLOR_ACCENT_CRIMSON, COLOR_ACCENT_GOLD, COLOR_TAB_ACTIVE_BG, COLOR_TAB_BG, COLOR_TEXT_MUTED,
    COLOR_TEXT_PRIMARY, COLOR_TRACK_BG,
};
use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::SettingsConfig;
use crate::world::ui_and_graphic_resources::data::components::TranslationKey;

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
/// число справа (например `decimals: 0, suffix: "%"` -> "72%"). `kind` —
/// какую вкладку пометить изменённой, когда ползунок подвинули.
#[allow(clippy::too_many_arguments)]
pub fn spawn_slider_row(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    language: Language,
    label_key: &'static str,
    min: f32,
    max: f32,
    step: f32,
    decimals: u8,
    suffix: &'static str,
    initial: f32,
    kind: SettingsTab,
    on_change: impl Fn(&mut SettingsConfig, f32) + Send + Sync + 'static,
) -> Entity {
    let row = theme::spawn_row_container(commands, parent);
    theme::spawn_row_label(commands, row, font, localization, language, label_key);

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
            SliderStep(step),
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

    // NB: сама библиотека не двигает ползунок визуально — это ответственность
    // стилиста, поэтому и fill, и thumb, и подпись значения обновляем тут же.
    // Упрощение: позиция thumb не учитывает его собственную ширину (небольшой
    // "нахлёст" на краях трека) — если важно, поправьте percent_of с поправкой
    // на ширину thumb в пикселях.
    commands.entity(track).observe(
        move |trigger: On<ValueChange<f32>>,
              mut q_node: Query<&mut Node>,
              mut q_text: Query<&mut Text>,
              mut settings: ResMut<Persistent<SettingsConfig>>,
              mut pending: ResMut<PendingSettingsChanges>| {
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
            pending.0.insert(kind);
        },
    );

    row
}

/// Строка с чекбоксом (галочка отрисована отдельным квадратиком, чтобы не
/// зависеть от того, есть ли в шрифте глиф "✓").
#[allow(clippy::too_many_arguments)]
pub fn spawn_checkbox_row(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    language: Language,
    label_key: &'static str,
    initial: bool,
    kind: SettingsTab,
    on_change: impl Fn(&mut SettingsConfig, bool) + Send + Sync + 'static,
) -> Entity {
    let row = theme::spawn_row_container(commands, parent);
    theme::spawn_row_label(commands, row, font, localization, language, label_key);

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
              mut settings: ResMut<Persistent<SettingsConfig>>,
              mut pending: ResMut<PendingSettingsChanges>| {
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
            pending.0.insert(kind);
        },
    );
    row
}

/// Подпись опции в радио-группе: либо ключ локализации (тогда подпись сама
/// обновится при смене языка), либо готовый текст как есть — так делают
/// названия языков ("Русский", "English") или числа ("1920x1080"), которые
/// переводить не нужно.
#[derive(Clone, Copy)]
pub enum OptionLabel {
    Key(&'static str),
    Literal(&'static str),
}

impl OptionLabel {
    fn resolve<'a>(self, localization: &'a Localization, language: Language) -> &'a str {
        match self {
            Self::Key(key) => localization.get(key, language),
            Self::Literal(text) => text,
        }
    }

    fn translation_key(self) -> Option<&'static str> {
        match self {
            Self::Key(key) => Some(key),
            Self::Literal(_) => None,
        }
    }
}

/// Значение конкретной radio-кнопки — по нему единый observer на группе
/// узнаёт, что применить, когда выбрали именно эту кнопку.
#[derive(Component)]
struct RadioOptionValue<T>(T);

/// Строка с набором взаимоисключающих кнопок (замена выпадающему списку —
/// проще в реализации и не требует Popover/Menu из ui_widgets).
///
/// `T` — тип значения одной опции (enum вашей игры, кортеж и т.п.), должен
/// быть `Copy + PartialEq`. В отличие от старой версии с
/// `RadioOptionBinding(Box<dyn Fn(...)>)` на каждой кнопке, тут один-
/// единственный observer висит на самой `RadioGroup`: `apply` — обычный
/// fn-указатель (не dyn), а он мономорфизируется под конкретный `T`
/// отдельно для каждого вызова `spawn_radio_row`.
#[allow(clippy::too_many_arguments)]
pub fn spawn_radio_row<T>(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    language: Language,
    label_key: &'static str,
    options: &[(OptionLabel, T)],
    current: T,
    kind: SettingsTab,
    apply: fn(&mut SettingsConfig, T),
) -> Entity
where
    T: Copy + PartialEq + Send + Sync + 'static,
{
    let row = theme::spawn_row_container(commands, parent);
    theme::spawn_row_label(commands, row, font, localization, language, label_key);

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

    for &(label, value) in options {
        let selected = value == current;

        let btn = commands
            .spawn((
                RadioButton,
                RadioOptionValue(value),
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

        let mut option_text = commands.spawn((
            Text::new(label.resolve(localization, language)),
            TextFont {
                font: FontSource::Handle(font.clone()),
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(COLOR_TEXT_PRIMARY),
            ChildOf(btn),
        ));
        if let Some(translation_key) = label.translation_key() {
            option_text.insert(TranslationKey(translation_key));
        }
    }

    commands.entity(group).observe(
        move |trigger: On<ValueChange<Entity>>,
              q_children: Query<&Children>,
              q_values: Query<&RadioOptionValue<T>>,
              mut q_bg: Query<&mut BackgroundColor, With<RadioButton>>,
              mut commands: Commands,
              mut settings: ResMut<Persistent<SettingsConfig>>,
              mut pending: ResMut<PendingSettingsChanges>| {
            let selected = trigger.value;

            if let Ok(children) = q_children.get(trigger.source) {
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

            if let Ok(value) = q_values.get(selected) {
                apply(&mut settings, value.0);
                pending.0.insert(kind);
            }
        },
    );

    row
}

/// Кнопка-вкладка в сайдбаре. Переключение самой вкладки вешается снаружи,
/// в `layout.rs` — ему нужен полный список кнопок сразу, а не по одной.
pub fn spawn_tab_button(
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    localization: &Localization,
    language: Language,
    key: &'static str,
    active: bool,
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
        Text::new(localization.get(key, language)),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextColor(COLOR_TEXT_PRIMARY),
        TranslationKey(key),
        ChildOf(btn),
    ));

    btn
}
