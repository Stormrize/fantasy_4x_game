//! Переиспользуемый виджет дропдауна: кнопка "Заголовок: значение" +
//! разворачивающийся список опций. При выборе опции — обновляет свою же
//! подпись (текст и TranslationKey, чтобы смена языка потом работала
//! корректно) и пишет значение в [`PendingGameSetup`] через `apply`.

use bevy::prelude::{
    AlignItems, Button, ChildOf, Click, Commands, Component, Display, Entity, FontSize, FontSource,
    GlobalZIndex, JustifyContent, Node, On, Out, Over, Pointer, PositionType, Query, Res, ResMut,
    Text, TextColor, TextFont, UiRect,
};
use bevy::ui::{BackgroundColor, BorderColor, FlexDirection, Val};
use bevy::utils::default;
use bevy_persistent::Persistent;

use crate::ui::colors::{COLOR_ACCENT_GOLD, COLOR_TEXT_PRIMARY};
use crate::ui::game_setup_menu::PendingGameSetup;
use crate::ui::game_setup_menu::colors::{
    COLOR_DROPDOWN_BG, COLOR_DROPDOWN_HOVER, COLOR_DROPDOWN_OPTION_BG,
};
use crate::ui::initial_loading::fonts_loader_plugin::fonts::Fonts;
use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::SettingsConfig;
use crate::world::ui_and_graphic_resources::data::components::TranslationKey;

#[derive(Component)]
struct DropdownState {
    is_open: bool,
}

pub fn spawn_dropdown<T>(
    commands: &mut Commands,
    parent: Entity,
    localization: &Localization,
    fonts: &Fonts,
    language: Language,
    options: &'static [(&'static str, T)],
    selected_index: usize,
    apply: fn(&mut PendingGameSetup, T),
) where
    T: Clone + Copy + Send + Sync + 'static,
{
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

    let button = commands
        .spawn((
            Button,
            DropdownState { is_open: false },
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                padding: UiRect::horizontal(Val::Px(10.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(COLOR_DROPDOWN_BG),
            BorderColor::all(COLOR_ACCENT_GOLD),
            ChildOf(row),
        ))
        .id();

    let selected_key = options[selected_index].0;
    let value_label = commands
        .spawn((
            Text::new(localization.get(selected_key, language)),
            TextFont {
                font_size: FontSize::Px(14.0),
                font: FontSource::Handle(fonts.get(language).clone()),
                ..default()
            },
            TextColor(COLOR_TEXT_PRIMARY),
            TranslationKey(selected_key),
            ChildOf(button),
        ))
        .id();

    let options_panel = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(36.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::horizontal(Val::Px(10.0)),
                display: Display::None,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(COLOR_DROPDOWN_OPTION_BG),
            BorderColor::all(COLOR_ACCENT_GOLD),
            GlobalZIndex(1),
            ChildOf(row),
        ))
        .id();

    observe_toggle(commands, button, options_panel);

    for (key, value) in options.iter() {
        let key: &'static str = *key;
        spawn_option(
            commands,
            options_panel,
            button,
            value_label,
            localization,
            fonts,
            language,
            key,
            *value,
            apply,
        );
    }
}

fn observe_toggle(commands: &mut Commands, button: Entity, options_panel: Entity) {
    commands
        .entity(button)
        .observe(
            move |_trigger: On<Pointer<Click>>,
                  mut states: Query<&mut DropdownState>,
                  mut nodes: Query<&mut Node>| {
                if let Ok(mut state) = states.get_mut(button) {
                    state.is_open = !state.is_open;
                    if let Ok(mut node) = nodes.get_mut(options_panel) {
                        node.display = if state.is_open {
                            Display::Flex
                        } else {
                            Display::None
                        };
                    }
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Over>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(trigger.entity) {
                    color.0 = COLOR_DROPDOWN_HOVER;
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(trigger.entity) {
                    color.0 = COLOR_DROPDOWN_BG;
                }
            },
        );
}

#[allow(clippy::too_many_arguments)]
fn spawn_option<T>(
    commands: &mut Commands,
    options_panel: Entity,
    button: Entity,
    value_label: Entity,
    localization: &Localization,
    fonts: &Fonts,
    language: Language,
    key: &'static str,
    value: T,
    apply: fn(&mut PendingGameSetup, T),
) where
    T: Clone + Send + Sync + 'static,
{
    let option = commands
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(28.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(COLOR_DROPDOWN_OPTION_BG),
            ChildOf(options_panel),
        ))
        .id();

    commands.spawn((
        Text::new(localization.get(key, language)),
        TextFont {
            font_size: FontSize::Px(13.0),
            font: FontSource::Handle(fonts.get(language).clone()),
            ..default()
        },
        TextColor(COLOR_TEXT_PRIMARY),
        TranslationKey(key),
        ChildOf(option),
    ));

    commands
        .entity(option)
        .observe(
            move |mut event: On<Pointer<Click>>,
                  mut texts: Query<&mut Text>,
                  mut keys: Query<&mut TranslationKey>,
                  mut states: Query<&mut DropdownState>,
                  mut nodes: Query<&mut Node>,
                  mut pending: ResMut<PendingGameSetup>,
                  localization: Res<Localization>,
                  settings: Res<Persistent<SettingsConfig>>| {
                event.propagate(false);
                let language = settings.language;

                if let Ok(mut text) = texts.get_mut(value_label) {
                    *text = Text::new(localization.get(key, language));
                }
                // держим TranslationKey в актуальном состоянии, чтобы смена
                // языка потом релокализовала именно выбранное значение,
                // а не то, что было выбрано изначально
                if let Ok(mut translation_key) = keys.get_mut(value_label) {
                    translation_key.0 = key;
                }
                if let Ok(mut state) = states.get_mut(button) {
                    state.is_open = false;
                }
                if let Ok(mut node) = nodes.get_mut(options_panel) {
                    node.display = Display::None;
                }

                apply(&mut pending, value.clone());
            },
        )
        .observe(
            |trigger: On<Pointer<Over>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(trigger.entity) {
                    color.0 = COLOR_ACCENT_GOLD;
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(trigger.entity) {
                    color.0 = COLOR_DROPDOWN_OPTION_BG;
                }
            },
        );
}
