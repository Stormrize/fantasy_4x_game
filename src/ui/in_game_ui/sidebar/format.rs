//! Localization keys, text labels and colors for sidebar rows.

use bevy::prelude::*;

use crate::game::game_logic_plugin::action::rules::movement::biome_cost;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;
use crate::world::components::Id;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType};
use crate::world::game_resources::data::resource::{CommonKind, RareKind};
use crate::world::game_resources::data::unit::{UnitLevel, UnitType};

use super::data::{BuildingView, UnitView};

// Owner

pub(crate) fn owner_label(ui: &UiContext, language: Language, owner: Id, local_id: Id) -> String {
    if owner == local_id {
        ui.get(language, "you").to_string()
    } else {
        format!("{} {}", ui.get(language, "player"), owner.0)
    }
}

// Units

/// `Fire Mages Damage II`; builders have no level.
pub(crate) fn unit_title(ui: &UiContext, language: Language, unit: &UnitView) -> String {
    let kind = ui.get(language, unit_kind_key(unit.kind));

    match unit.level {
        Some(level) => format!(
            "{} {kind} {}",
            theme::race_display_name(&unit.race.0),
            unit_level_roman(level)
        ),
        None => kind.to_string(),
    }
}

/// `HP 4/4  ·  MP 2/2  ·  (3, 7)` for a list row.
pub(crate) fn unit_summary(unit: &UnitView) -> String {
    format!(
        "HP {}/{}  ·  MP {}/{}  ·  ({}, {})",
        unit.hp, unit.max_hp, unit.moves, unit.moves_max, unit.pos.x, unit.pos.y
    )
}

pub(crate) fn unit_kind_key(kind: UnitType) -> &'static str {
    match kind {
        UnitType::DamageDealer => "unit_damage",
        UnitType::Support => "unit_support",
        UnitType::Skill => "unit_skill",
        UnitType::Helper => "unit_builder",
    }
}

pub(crate) fn unit_color(unit: &UnitView) -> Color {
    match unit.kind {
        UnitType::DamageDealer => theme::ENEMY,
        UnitType::Support => Color::srgb(0.482, 0.773, 0.443),
        UnitType::Skill => theme::RES_SCIENCE,
        UnitType::Helper => theme::RES_GOLD,
    }
}

pub(crate) fn unit_level_roman(level: UnitLevel) -> &'static str {
    match level {
        UnitLevel::Beginner => "I",
        UnitLevel::Master => "II",
        UnitLevel::Holy => "III",
    }
}

pub(crate) fn hp_color(hp: u16, max_hp: u16) -> Color {
    let ratio = if max_hp == 0 {
        0.0
    } else {
        hp as f32 / max_hp as f32
    };
    if ratio > 0.5 {
        theme::HP_GOOD
    } else if ratio > 0.25 {
        theme::HP_WARN
    } else {
        theme::HP_LOW
    }
}

pub(crate) fn skill_name(ui: &UiContext, language: Language, skill_type: u8) -> String {
    match skill_type {
        0 => ui.get(language, "skill_attack").to_string(),
        1..=3 => format!("{} {skill_type}", ui.get(language, "skill")),
        _ => ui.get(language, "skill").to_string(),
    }
}

// Buildings

/// `Capital II`
pub(crate) fn building_title(ui: &UiContext, language: Language, city: &BuildingView) -> String {
    format!(
        "{} {}",
        ui.get(language, building_key(city.kind)),
        level_roman(city.level)
    )
}

/// `+3g +2f +1s  ·  (3, 7)` for a list row.
pub(crate) fn building_summary(city: &BuildingView) -> String {
    format!(
        "+{}g +{}f +{}s  ·  ({}, {})",
        city.gold, city.food, city.science, city.pos.x, city.pos.y
    )
}

pub(crate) fn building_key(kind: BuildingType) -> &'static str {
    match kind {
        BuildingType::Capital => "building_capital",
        BuildingType::CapturedCity => "building_captured_city",
        BuildingType::BarbarianCity => "building_barbarian_city",
        BuildingType::SpawningBuilding => "building_spawning",
        BuildingType::House => "building_house",
        BuildingType::MediumBuilding => "building_medium",
        BuildingType::LargeBuilding => "building_large",
        BuildingType::ResearchCenter => "building_research",
        BuildingType::UniqueRaceBuilding => "building_unique",
    }
}

pub(crate) fn level_roman(level: BuildingLevel) -> &'static str {
    match level {
        BuildingLevel::Standard => "I",
        BuildingLevel::Advanced => "II",
        BuildingLevel::Complex => "III",
    }
}

// Terrain

pub(crate) fn biome_key(biome: BiomeType) -> &'static str {
    match biome {
        BiomeType::Water => "biome_water",
        BiomeType::SmallBiome => "biome_small",
        BiomeType::MediumBiome => "biome_medium",
        BiomeType::LargeBiome => "biome_large",
        BiomeType::Ground => "biome_ground",
    }
}

pub(crate) fn biome_color(biome: BiomeType) -> Color {
    match biome {
        BiomeType::Water => theme::MANA,
        BiomeType::SmallBiome => theme::RES_FOOD,
        BiomeType::MediumBiome => theme::RES_GOLD,
        BiomeType::LargeBiome => theme::RES_SCIENCE,
        BiomeType::Ground => theme::MUTED,
    }
}

pub(crate) fn move_cost_label(ui: &UiContext, language: Language, biome: BiomeType) -> String {
    let cost = biome_cost(biome);
    if cost == 0 {
        "-".to_string()
    } else {
        format!("{cost} {}", ui.get(language, "move_points"))
    }
}

// Resources

pub(crate) fn common_kind_key(kind: CommonKind) -> &'static str {
    match kind {
        CommonKind::Food => "resource_food",
        CommonKind::Gold => "resource_gold",
        CommonKind::Science => "resource_science",
    }
}

pub(crate) fn common_color(kind: CommonKind) -> Color {
    match kind {
        CommonKind::Food => theme::RES_FOOD,
        CommonKind::Gold => theme::RES_GOLD,
        CommonKind::Science => theme::RES_SCIENCE,
    }
}

pub(crate) fn rare_kind_key(kind: RareKind) -> &'static str {
    match kind {
        RareKind::ManaScroll => "rare_mana_scroll",
        RareKind::HealScroll => "rare_heal_scroll",
        RareKind::AttackScroll => "rare_attack_scroll",
        RareKind::RevealScroll => "rare_reveal_scroll",
    }
}

pub(crate) fn rare_description_key(kind: RareKind) -> &'static str {
    match kind {
        RareKind::ManaScroll => "rare_mana_scroll_desc",
        RareKind::HealScroll => "rare_heal_scroll_desc",
        RareKind::AttackScroll => "rare_attack_scroll_desc",
        RareKind::RevealScroll => "rare_reveal_scroll_desc",
    }
}
