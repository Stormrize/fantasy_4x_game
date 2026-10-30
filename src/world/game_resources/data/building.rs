use bevy::prelude::{Bundle, Component};
use serde::Deserialize;

use crate::world::components::{Cost, Food, Gold, Id, PlayerRace, Science};
use crate::world::game_resources::data::buffs_and_debuffs::{Buffs, Debuffs};

pub const BUILDING_LEVELS: usize = 3;

#[derive(Component, Debug, Clone, Copy)]
pub enum BuildingLevel {
    Standard = 0,
    Advanced = 1,
    Complex = 2,
}

impl BuildingLevel {
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Standard => Some(Self::Advanced),
            Self::Advanced => Some(Self::Complex),
            Self::Complex => None,
        }
    }
}

#[derive(Component, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    Capital,
    CapturedCity,
    BarbarianCity,
    SpawningBuilding,
    House,
    MediumBuilding,
    LargeBuilding,
    ResearchCenter,
    UniqueRaceBuilding, // read from config
}

impl BuildingType {
    pub fn index(&self) -> usize {
        use BuildingType::*;
        match self {
            Capital => 0,
            CapturedCity => 1,
            BarbarianCity => 2,
            SpawningBuilding => 3,
            House => 4,
            MediumBuilding => 5,
            LargeBuilding => 6,
            ResearchCenter => 7,
            UniqueRaceBuilding => 8,
        }
    }
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct BuildingStats {
    pub health: u16,
    pub defense: u16,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Yields {
    pub gold: Gold,
    pub food: Food,
    pub science: Science,
}

impl Yields {
    pub const NONE: Self = Self {
        gold: 0,
        food: 0,
        science: 0,
    };
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct BuildingUtility {
    /// Население / жилые места.
    pub housing: u16,

    /// Сколько юнитов может одновременно спавниться
    /// или сколько spawn capacity дает здание.
    pub spawn_capacity: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuildingLevelStats {
    combat_stats: BuildingStats,       // 32
    yields: Yields,                    // 64
    economy: Cost,                     // 32
    building_utility: BuildingUtility, // 32
}

impl BuildingLevelStats {
    pub fn upgrade_cost(&self) -> Gold {
        self.economy.new_level_cost
    }
    pub fn maintenance(&self) -> Gold {
        self.economy.maintenance
    }
    pub fn housing(&self) -> u16 {
        self.building_utility.housing
    }
    pub fn spawn_capacity(&self) -> u16 {
        self.building_utility.spawn_capacity
    }
    pub fn gold(&self) -> Gold {
        self.yields.gold
    }
    pub fn food(&self) -> Food {
        self.yields.food
    }
    pub fn science(&self) -> Science {
        self.yields.science
    }
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Levels(pub [BuildingLevelStats; BUILDING_LEVELS]);

impl Levels {
    pub fn stats(&self, level: BuildingLevel) -> &BuildingLevelStats {
        &self.0[level as usize]
    }
}

#[derive(Bundle, Deserialize, Debug, Clone)]
pub struct BuildingData {
    pub building_type: BuildingType,
    pub levels: Levels,
}

impl BuildingData {
    pub fn stats(&self, level: BuildingLevel) -> &BuildingLevelStats {
        &self.levels.0[level as usize]
    }

    /// Данные здания по его типу. `UniqueRaceBuilding` читается из конфигов расы,
    /// поэтому здесь недоступен.
    pub fn from_type(building_type: BuildingType) -> Option<Self> {
        Some(match building_type {
            BuildingType::Capital => CAPITAL,
            BuildingType::CapturedCity => CAPTURED_CITY,
            BuildingType::BarbarianCity => BARBARIAN_CITY,
            BuildingType::SpawningBuilding => SPAWNING_BUILDING,
            BuildingType::House => HOUSE,
            BuildingType::MediumBuilding => MEDIUM_BUILDING,
            BuildingType::LargeBuilding => LARGE_BUILDING,
            BuildingType::ResearchCenter => RESEARCH_CENTER,
            BuildingType::UniqueRaceBuilding => return None,
        })
    }
}

#[allow(clippy::too_many_arguments)]
const fn level(
    health: u16,
    defense: u16,
    gold: Gold,
    food: Food,
    science: Science,
    new_level_cost: Gold,
    maintenance: Gold,
    housing: u16,
    spawn_capacity: u16,
) -> BuildingLevelStats {
    BuildingLevelStats {
        combat_stats: BuildingStats { health, defense },
        yields: Yields {
            gold,
            food,
            science,
        },
        economy: Cost {
            new_level_cost,
            maintenance,
        },
        building_utility: BuildingUtility {
            housing,
            spawn_capacity,
        },
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct Building {
    pub id: Id,
    pub player_race: PlayerRace,
    pub level: BuildingLevel,
    pub building_data: BuildingData,
    pub active_buffs: Buffs,
    pub active_debuffs: Debuffs,
}

//---------------------------------------------------------------------------------------------------------------------------//

pub const CAPITAL: BuildingData = BuildingData {
    building_type: BuildingType::Capital,
    levels: Levels([
        level(400, 100, 10, 10, 2, 50, 0, 5, 0),
        level(650, 250, 25, 25, 4, 80, 0, 10, 0),
        level(1000, 450, 40, 25, 8, 115, 0, 20, 0),
    ]),
};

pub const CAPTURED_CITY: BuildingData = BuildingData {
    building_type: BuildingType::CapturedCity,
    levels: Levels([
        level(250, 50, 4, 2, 1, 35, 0, 2, 0),
        level(450, 140, 10, 6, 3, 60, 0, 5, 0),
        level(700, 260, 20, 12, 5, 90, 0, 10, 0),
    ]),
};

pub const BARBARIAN_CITY: BuildingData = BuildingData {
    building_type: BuildingType::BarbarianCity,
    levels: Levels([
        level(300, 60, 5, 0, 0, 35, 0, 0, 0),
        level(450, 150, 15, 0, 0, 60, 0, 0, 0),
        level(600, 230, 30, 0, 0, 90, 0, 0, 0),
    ]),
};

pub const SPAWNING_BUILDING: BuildingData = BuildingData {
    building_type: BuildingType::SpawningBuilding,
    levels: Levels([
        // можно спавнить Master Unitов
        level(100, 20, 0, 0, 0, 15, 5, 0, 1),
        // можно спавнить Holy юнитов
        level(200, 80, 0, 0, 0, 30, 8, 0, 1),
        // можно больше за раз спавнить
        level(400, 200, 0, 0, 0, 55, 15, 0, 2),
    ]),
};

// спавнится только на small biome
pub const HOUSE: BuildingData = BuildingData {
    building_type: BuildingType::House,
    levels: Levels([
        level(80, 10, 0, 0, 0, 10, 1, 5, 0),
        level(160, 30, 0, 0, 0, 25, 2, 12, 0),
        level(300, 60, 0, 0, 0, 50, 4, 20, 0),
    ]),
};

// спавниться только на medium biome
pub const MEDIUM_BUILDING: BuildingData = BuildingData {
    building_type: BuildingType::MediumBuilding,
    levels: Levels([
        level(120, 25, 5, 0, 0, 20, 2, 0, 0),
        level(250, 70, 12, 0, 0, 40, 4, 0, 0),
        level(450, 140, 24, 0, 0, 75, 7, 0, 0),
    ]),
};

// спавниться только на large biome
pub const LARGE_BUILDING: BuildingData = BuildingData {
    building_type: BuildingType::LargeBuilding,
    levels: Levels([
        level(200, 40, 10, 0, 0, 35, 4, 0, 0),
        level(400, 100, 24, 0, 0, 65, 8, 0, 0),
        level(700, 180, 50, 0, 0, 115, 14, 0, 0),
    ]),
};

// спавниться на small biome только, можно позже добавить бонус за соседство с large biome
pub const RESEARCH_CENTER: BuildingData = BuildingData {
    building_type: BuildingType::ResearchCenter,
    levels: Levels([
        level(100, 20, 0, 0, 5, 30, 4, 0, 0),
        level(220, 60, 0, 0, 14, 60, 8, 0, 0),
        level(400, 120, 0, 0, 30, 105, 14, 0, 0),
    ]),
};
