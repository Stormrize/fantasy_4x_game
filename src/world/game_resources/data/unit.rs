use crate::world::components::{Id, PlayerRace};
use crate::world::game_resources::data::buffs_and_defuffs::{
    ActiveBuffs, ActiveDebuffs, Buff, Debuff,
};
use crate::world::game_resources::data::building::Economy;
use bevy::prelude::{Bundle, Component};
use smol_str::SmolStr;

#[derive(Component, Debug, Clone, Copy)]
pub enum UnitLevel {
    Beginner = 0,
    Intermediate = 1,
    Master = 2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum UnitType {
    Support,
    DamageDealer,
    Tank,
    Skill,
    ResourceGathererBuilder,
}

impl UnitType {
    pub fn index(&self) -> usize {
        use UnitType::*;
        match self {
            DamageDealer => 0,
            Support => 1,
            Tank => 2,
            Skill => 3,
            ResourceGathererBuilder => 4,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct BasicStats {
    pub health: u16,
    pub defense: u16,
    pub attack: u16,
    pub range: u8,
    pub movement_points: u8,
}

#[derive(Debug, Clone)]
struct UniqueLevelAbility {
    pub name: SmolStr,
    pub range: u16,
    pub area: u8,
    pub damage: u16,
    pub buff: Buff,
    pub debuff: Debuff,
}

#[derive(Component, Debug, Clone)]
pub struct LevelAbilities(pub Vec<UniqueLevelAbility>);

#[derive(Bundle)]
pub struct Unit {
    pub player_id: Id,
    pub player_race: PlayerRace,
    pub unit: UnitType,
    pub level: UnitLevel,
    pub economy: Economy,
    pub basic_stats: BasicStats,
    pub unique_level_abilities: LevelAbilities,
    pub active_buffs: ActiveBuffs,
    pub active_debuffs: ActiveDebuffs,
}
