//! Сайдбар справа: заголовок + три дропдауна с дефолтными значениями
//! (размер карты, форма карты, сложность ИИ).

use bevy::prelude::{
    ChildOf, Commands, Entity, FontSize, FontSource, Node, Text, TextColor, TextFont, UiRect,
};
use bevy::text::FontWeight;
use bevy::ui::Val;
use bevy::utils::default;
use std::ops::Range;

use super::components::PendingGameSetup;
use crate::ui::colors::{COLOR_ACCENT_GOLD, COLOR_TEXT_PRIMARY};
use crate::ui::game_setup_menu::dropdown::spawn_dropdown;
use crate::ui::initial_loading::fonts_loader_plugin::fonts::Fonts;
use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};
use crate::world::components::{AiDifficulty, MapShape, MapSize};
use crate::world::ui_and_graphic_resources::data::components::TranslationKey;

const TITLE_KEY: &str = "setup_sidebar_title";

const MAP_SIZE_LABEL_KEY: &str = "setup_map_size_label";
const MAP_SIZE_OPTIONS: [(&str, MapSize); 5] = [
    ("setup_map_size_tiny", MapSize::Tiny),
    ("setup_map_size_small", MapSize::Small),
    ("setup_map_size_medium", MapSize::Medium),
    ("setup_map_size_big", MapSize::Big),
    ("setup_map_size_large", MapSize::Large),
];
const MAP_SIZE_DEFAULT_INDEX: usize = 2; // Medium

const MAP_SHAPE_LABEL_KEY: &str = "setup_map_shape_label";

const MAP_SHAPE_OPTIONS: [(&str, MapShape); 2] = [
    ("setup_map_shape_jagged", MapShape::Jagged),
    ("setup_map_shape_round", MapShape::Round),
    // ("setup_map_shape_continents", MapShape::Continents),
    // ("setup_map_shape_islands", MapShape::Islands),
    // ("setup_map_shape_pangaea", MapShape::Pangaea),
];
const MAP_SHAPE_DEFAULT_INDEX: usize = 0; // Jagged

const AI_DIFFICULTY_LABEL_KEY: &str = "setup_ai_difficulty_label";
const AI_DIFFICULTY_OPTIONS: [(&str, AiDifficulty); 4] = [
    ("setup_ai_difficulty_easy", AiDifficulty::Easy),
    ("setup_ai_difficulty_normal", AiDifficulty::Normal),
    ("setup_ai_difficulty_hard", AiDifficulty::Hard),
    ("setup_ai_difficulty_crazy", AiDifficulty::Crazy),
];
const AI_DIFFICULTY_DEFAULT_INDEX: usize = 1; // Normal

const BOTS_AMOUNT_LABEL_KEY: &str = "Amount of bots";

const BOTS_AMOUNT_RANGE: Range<i32> = 1..6;
const BOTS_AMOUNT_DEFAULT_INDEX: usize = 1;

pub fn spawn_sidebar(
    commands: &mut Commands,
    sidebar: Entity,
    localization: &Localization,
    fonts: &Fonts,
    language: Language,
) {
    spawn_title(commands, sidebar, localization, fonts, language);

    spawn_label(
        commands,
        sidebar,
        MAP_SIZE_LABEL_KEY,
        localization,
        fonts,
        language,
    );

    spawn_dropdown(
        commands,
        sidebar,
        localization,
        fonts,
        language,
        &MAP_SIZE_OPTIONS,
        MAP_SIZE_DEFAULT_INDEX,
        apply_map_size,
    );

    spawn_label(
        commands,
        sidebar,
        MAP_SHAPE_LABEL_KEY,
        localization,
        fonts,
        language,
    );

    spawn_dropdown(
        commands,
        sidebar,
        localization,
        fonts,
        language,
        &MAP_SHAPE_OPTIONS,
        MAP_SHAPE_DEFAULT_INDEX,
        apply_map_shape,
    );

    spawn_label(
        commands,
        sidebar,
        AI_DIFFICULTY_LABEL_KEY,
        localization,
        fonts,
        language,
    );

    spawn_dropdown(
        commands,
        sidebar,
        localization,
        fonts,
        language,
        &AI_DIFFICULTY_OPTIONS,
        AI_DIFFICULTY_DEFAULT_INDEX,
        apply_ai_difficulty,
    );
}

fn spawn_title(
    commands: &mut Commands,
    parent: Entity,
    localization: &Localization,
    fonts: &Fonts,
    language: Language,
) {
    commands.spawn((
        Text::new(localization.get(TITLE_KEY, language)),
        TextFont {
            font_size: FontSize::Px(24.0),
            font: FontSource::Handle(fonts.get(language).clone()),
            weight: FontWeight::SEMIBOLD,
            ..default()
        },
        TextColor(COLOR_ACCENT_GOLD),
        TranslationKey(TITLE_KEY),
        Node {
            margin: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        ChildOf(parent),
    ));
}

fn spawn_label(
    commands: &mut Commands,
    parent: Entity,
    label: &'static str,
    localization: &Localization,
    fonts: &Fonts,
    language: Language,
) {
    commands.spawn((
        Text::new(localization.get(label, language)),
        TextFont {
            font: FontSource::Handle(fonts.get(language).clone()),
            font_size: FontSize::Px(18.0),
            weight: FontWeight::MEDIUM,
            ..default()
        },
        Node {
            margin: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        TextColor(COLOR_TEXT_PRIMARY),
        TranslationKey(label),
        ChildOf(parent),
    ));
}

fn apply_map_size(cfg: &mut PendingGameSetup, value: MapSize) {
    cfg.map_size = value;
}

fn apply_map_shape(cfg: &mut PendingGameSetup, value: MapShape) {
    cfg.map_shape = value;
}

fn apply_ai_difficulty(cfg: &mut PendingGameSetup, value: AiDifficulty) {
    cfg.ai_difficulty = value;
}
