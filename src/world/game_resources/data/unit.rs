use bevy::prelude::{Bundle, Component};
use serde::Deserialize;
use smol_str::SmolStr;

use crate::world::components::{Cost, Gold, Id, Mana, PlayerRace};
use crate::world::game_resources::data::buffs_and_debuffs::{Buffs, Debuffs};

#[derive(Component, Debug, Clone, Copy, Default)]
pub enum UnitLevel {
    #[default]
    Beginner = 0,
    Master = 1,
    Holy = 2,
}

impl UnitLevel {
    pub fn next(&self) -> Option<UnitLevel> {
        match self {
            Self::Beginner => Some(Self::Master),
            Self::Master => Some(Self::Holy),
            Self::Holy => None,
        }
    }
}

#[derive(Component, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum UnitType {
    Support,
    DamageDealer,
    Skill,
    Helper,
}

impl UnitType {
    pub fn index(&self) -> usize {
        use UnitType::*;
        match self {
            DamageDealer => 0,
            Support => 1,
            Skill => 2,
            Helper => 3,
        }
    }
}

//----------------------------------------//

#[derive(Component, Deserialize, Debug, Clone, Copy)]
pub struct CombatBasicStats {
    pub health: u16,
    pub defense: u16,
    pub attack: u16,
    pub range: u16,
}

#[derive(Deserialize, Debug, Clone)]
struct UniqueLevelAbility {
    pub name: SmolStr,
    pub buffs: Buffs,
    pub debuffs: Debuffs,
    pub mana_amount_needs: Mana,
    pub damage: u16,
    pub cool_down_turns: u16,
    pub range: u8,
    pub area: u8,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct LevelAbilities(pub Vec<UniqueLevelAbility>);

#[derive(Component, Deserialize, Debug, Clone)]
pub struct MovementPoints(pub u16);

/// Максимальные очки движения юнита. Восстанавливаются в начале хода игрока,
/// которому принадлежит юнит.
#[derive(Component, Debug, Clone, Copy)]
pub struct MovementPointsMax(pub u16);

#[derive(Component, Deserialize, Debug, Clone)]
pub struct FieldOfView(pub u16);

#[derive(Component, Deserialize, Debug, Clone)]
pub struct LevelCost(pub Vec<Cost>);

#[derive(Bundle, Deserialize, Debug, Clone)]
pub struct CombatUnitData {
    pub unit_type: UnitType,
    pub level_economy: LevelCost,
    pub basic_stats: CombatBasicStats,
    pub movement_points: MovementPoints,
    pub field_of_view: FieldOfView,
    pub unique_level_abilities: LevelAbilities,
}

#[derive(Bundle, Debug, Clone)]
pub struct CombatUnit {
    pub player_id: Id,
    pub player_race: PlayerRace,
    pub level: UnitLevel,
    pub unit_data: CombatUnitData, // read from config
    pub active_buffs: Buffs,
    pub active_debuffs: Debuffs,
}

//----------------------------------------//

#[derive(Component, Debug, Clone)]
pub struct HelperBasicStats {
    pub health: u16,
    pub defense: u16,
    pub build_gather_points: u8,
}

/// Цена найма юнита (у строителя — в городе).
#[derive(Component, Debug, Clone, Copy)]
pub struct UnitCost(pub Gold);

#[derive(Bundle, Debug, Clone)]
pub struct HelperData {
    pub unit_type: UnitType,
    pub basic_stats: HelperBasicStats,
    pub movement_points: MovementPoints,
    pub field_of_view: FieldOfView,
    pub unit_cost: UnitCost,
}

pub const HELPER: HelperData = HelperData {
    unit_type: UnitType::Helper,
    basic_stats: HelperBasicStats {
        health: 1,
        defense: 0,
        build_gather_points: 3,
    },
    movement_points: MovementPoints(2),
    field_of_view: FieldOfView(2),
    unit_cost: UnitCost(10),
};

#[derive(Bundle, Debug, Clone)]
pub struct Helper {
    pub player_id: Id,
    pub player_race: PlayerRace,
    pub unit_data: HelperData,
    pub active_buffs: Buffs,
    pub active_debuffs: Debuffs,
}
