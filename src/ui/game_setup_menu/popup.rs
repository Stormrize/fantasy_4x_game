//! Всплывающее окно с информацией о расе + кнопки "Начать игру" / "Закрыть".

use super::build_game_config::start_local_game;
use super::colors::*;
use super::components::PendingGameSetup;
use crate::ui::colors::{COLOR_ACCENT_GOLD, COLOR_TEXT_PRIMARY};
use crate::ui::initial_loading::fonts_loader_plugin::fonts::Fonts;
use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};
use crate::ui::menus::AppState;
use crate::world::components::PlayerRace;
use crate::world::ui_and_graphic_resources::data::components::TranslationKey;
use crate::world::ui_and_graphic_resources::data::ui_race_data::UiRaceData;
use crate::world::ui_and_graphic_resources::player_registry::RacesUiRegistry;
use bevy::prelude::{
    AlignItems, Button, ChildOf, Click, Commands, Component, Entity, FontSize, FontSource,
    GlobalZIndex, ImageNode, JustifyContent, NextState, Node, On, Out, Over, Pointer, PositionType,
    Query, Res, ResMut, Text, TextColor, TextFont, UiRect, With,
};
use bevy::ui::{BackgroundColor, BorderColor, FlexDirection, Val};
use bevy::utils::default;

const START_GAME_KEY: &str = "setup_start_game";
const CLOSE_KEY: &str = "setup_close";

/// Метка корня попапа — по ней его находят, чтобы закрыть или заменить
/// попапом другой расы.
#[derive(Component)]
pub struct RacePopupRoot;

pub fn open_race_popup(
    commands: &mut Commands,
    menu_root: Entity,
    race: PlayerRace,
    data: &UiRaceData,
    localization: &Localization,
    fonts: &Fonts,
    language: Language,
) {
    let overlay = commands
        .spawn((
            RacePopupRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(COLOR_POPUP_OVERLAY),
            GlobalZIndex(10),
            ChildOf(menu_root),
        ))
        .id();

    let card = commands
        .spawn((
            Node {
                width: Val::Percent(32.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(COLOR_POPUP_BG),
            BorderColor::all(COLOR_ACCENT_GOLD),
            ChildOf(overlay),
        ))
        .id();

    commands.spawn((
        Node {
            width: Val::Px(96.0),
            height: Val::Px(96.0),
            margin: UiRect::bottom(Val::Px(12.0)),
            ..default()
        },
        ImageNode::new(data.ui_icon.clone()),
        ChildOf(card),
    ));

    commands.spawn((
        Text::new(data.id.to_string()),
        TextFont {
            font_size: FontSize::Px(20.0),
            font: FontSource::Handle(fonts.get(language).clone()),
            ..default()
        },
        TextColor(COLOR_ACCENT_GOLD),
        ChildOf(card),
    ));

    commands.spawn((
        Text::new(data.description.clone()),
        TextFont {
            font_size: FontSize::Px(14.0),
            font: FontSource::Handle(fonts.get(language).clone()),
            ..default()
        },
        TextColor(COLOR_TEXT_PRIMARY),
        Node {
            width: Val::Percent(100.0),
            margin: UiRect::vertical(Val::Px(12.0)),
            ..default()
        },
        ChildOf(card),
    ));

    spawn_start_button(commands, card, race.clone(), localization, fonts, language);
    spawn_close_button(commands, card, localization, fonts, language);
}

fn spawn_start_button(
    commands: &mut Commands,
    parent: Entity,
    race: PlayerRace,
    localization: &Localization,
    fonts: &Fonts,
    language: Language,
) {
    let button = commands
        .spawn((
            Button,
            Node {
                width: Val::Percent(70.0),
                height: Val::Px(40.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(COLOR_BUTTON_START_BG),
            ChildOf(parent),
        ))
        .id();

    commands.spawn((
        Text::new(localization.get(START_GAME_KEY, language)),
        TextFont {
            font_size: FontSize::Px(16.0),
            font: FontSource::Handle(fonts.get(language).clone()),
            ..default()
        },
        TextColor(COLOR_BUTTON_START_TEXT),
        TranslationKey(START_GAME_KEY),
        ChildOf(button),
    ));

    commands
        .entity(button)
        .observe(
            move |_event: On<Pointer<Click>>,
                  mut commands: Commands,
                  pending: Res<PendingGameSetup>,
                  popups: Query<Entity, With<RacePopupRoot>>,
                  mut next_state: ResMut<NextState<AppState>>,
                  races_ui_registry: Res<RacesUiRegistry>| {
                start_local_game(
                    &mut commands,
                    race.clone(),
                    &races_ui_registry,
                    &pending,
                    &mut next_state,
                );
                for popup in &popups {
                    commands.entity(popup).despawn();
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Over>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(trigger.entity) {
                    color.0 = COLOR_BUTTON_START_HOVER;
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut colors: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = colors.get_mut(trigger.entity) {
                    color.0 = COLOR_BUTTON_START_BG;
                }
            },
        );
}

fn spawn_close_button(
    commands: &mut Commands,
    parent: Entity,
    localization: &Localization,
    fonts: &Fonts,
    language: Language,
) {
    let button = commands
        .spawn((
            Button,
            Node {
                width: Val::Percent(70.0),
                height: Val::Px(32.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(COLOR_DROPDOWN_BG),
            ChildOf(parent),
        ))
        .id();

    commands.spawn((
        Text::new(localization.get(CLOSE_KEY, language)),
        TextFont {
            font_size: FontSize::Px(14.0),
            font: FontSource::Handle(fonts.get(language).clone()),
            ..default()
        },
        TextColor(COLOR_TEXT_PRIMARY),
        TranslationKey(CLOSE_KEY),
        ChildOf(button),
    ));

    commands
        .entity(button)
        .observe(
            |mut event: On<Pointer<Click>>,
             mut commands: Commands,
             popups: Query<Entity, With<RacePopupRoot>>| {
                event.propagate(false);
                for popup in &popups {
                    commands.entity(popup).despawn();
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
