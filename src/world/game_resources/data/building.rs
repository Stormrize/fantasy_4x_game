use crate::world::components::{GoldAmount, Id, PlayerRace};
use crate::world::game_resources::data::buffs_and_defuffs::{ActiveBuffs, ActiveDebuffs};
use bevy::prelude::{Bundle, Component};

#[derive(Component, Debug, Clone, Copy)]
pub enum BuildingLevel {
    Standard = 0,
    Advanced = 1,
    Complex = 2,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum BuildingType {
    Capital,
    EmptyCities,
    MediumBuilding,
    LargeBuilding,
    SpawningBuilding,
    House,
    ResearchCenter,
}

impl BuildingType {
    pub fn index(&self) -> usize {
        use BuildingType::*;
        match self {
            Capital => 0,
            EmptyCities => 1,
            MediumBuilding => 2,
            LargeBuilding => 3,
            SpawningBuilding => 4,
            House => 5,
            ResearchCenter => 6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProductionType {
    Gold,
    Food,
    Production,
    Science,
}

#[derive(Component, Debug, Clone)]
struct ProductionAmount(pub u16);

#[derive(Component, Debug, Clone)]
pub struct BuildingStats {
    pub health: u16,
    pub defense: u16,
}

#[derive(Component, Debug, Clone)]
pub struct Production {
    pub production_type: ProductionType,
    pub amount: u16,
}

#[derive(Component, Debug, Clone)]
pub struct Economy {
    pub upgrade_cost: GoldAmount,
    pub maintenance: GoldAmount,
}

#[derive(Bundle, Debug, Clone)]
pub struct Building {
    pub id: Id,
    pub player_race: PlayerRace,
    pub building_type: BuildingType,
    pub level: BuildingLevel,
    pub stats: BuildingStats,
    pub production: Production,
    pub economy: Economy,
    pub active_buffs: ActiveBuffs,
    pub active_debuffs: ActiveDebuffs,
}

// impl Building {
//     fn placeholder() -> Self {
//         Self {
//             id: Id::default_local_id(),
//             player_race: PlayerRace::default(),
//             building_type: None,
//         }
//     }
// }
