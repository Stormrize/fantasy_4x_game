use crate::world::components::{Id, PlayerRace};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ResourceType {
    Common(CommonResource),
    Rare(RareResource),
}

impl ResourceType {
    pub fn goes_to_inventory(&self) -> bool {
        matches!(self, ResourceType::Rare(_))
    }

    pub fn owner_race(&self) -> &PlayerRace {
        match self {
            ResourceType::Common(c) => &c.owner_race,
            ResourceType::Rare(r) => &r.owner_race,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            ResourceType::Common(common) => common.kind as usize,
            ResourceType::Rare(rare) => rare.kind as usize + 3,
        }
    }
}

// regular resources

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonKind {
    Food,    // small biome
    Gold,    // medium biome
    Science, // large biome
}

impl CommonKind {
    pub const ALL: [CommonKind; 3] = [CommonKind::Food, CommonKind::Gold, CommonKind::Science];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn amount_range(self) -> (u32, u32) {
        match self {
            CommonKind::Food => (4, 8),
            CommonKind::Gold => (4, 8),
            CommonKind::Science => (1, 3),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CommonResource {
    pub kind: CommonKind,
    pub amount: u32,
    pub owner_id: Option<Id>,
    pub owner_race: PlayerRace,
}

// rare resources

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RareKind {
    ManaScroll,   // + to mana
    HealScroll,   // heals all units
    AttackScroll, // + to all basic unit stats
    RevealScroll, // reveals terrain
}

impl RareKind {
    pub const ALL: [RareKind; 4] = [
        RareKind::ManaScroll,
        RareKind::HealScroll,
        RareKind::AttackScroll,
        RareKind::RevealScroll,
    ];

    /// smaller - rarer
    pub fn weight(self) -> u32 {
        match self {
            RareKind::ManaScroll => 4,
            RareKind::HealScroll => 3,
            RareKind::RevealScroll => 3,
            RareKind::AttackScroll => 2,
        }
    }

    pub fn base_power(self) -> u32 {
        match self {
            RareKind::ManaScroll => 20,
            RareKind::HealScroll => 30,
            RareKind::AttackScroll => 1,
            RareKind::RevealScroll => 8,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RareResource {
    pub kind: RareKind,
    pub owner_race: PlayerRace,
    pub power: u32,
}
