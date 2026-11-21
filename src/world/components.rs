use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

pub type Gold = u16;
pub type Food = u16;
pub type Science = u16;
pub type Mana = u16;

pub const CAPITAL_CONTROL_RADIUS: usize = 2;
pub const BUILDING_SEE_RADIUS: usize = 3;

#[derive(Component, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayerRace(pub SmolStr);

impl PlayerRace {
    pub fn new(name: SmolStr) -> Self {
        Self(name)
    }
}

impl Default for PlayerRace {
    fn default() -> Self {
        Self(SmolStr::from(""))
    }
}

#[derive(
    Component, Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct Id(pub u16);

impl Default for Id {
    fn default() -> Self {
        Self(u16::MAX)
    }
}

impl Id {
    pub const BARBARIAN_RANGE: std::ops::Range<usize> = 100..400;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    Human,
    Bot,
}

#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Online,
    Offline,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Cost {
    pub new_level_cost: Gold,
    pub maintenance: Gold,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum MapSize {
    Tiny = 8,
    Small = 16,
    #[default]
    Medium = 32,
    Big = 64,
    Large = 128,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Default)]
#[allow(dead_code)]
#[repr(u8)]
pub enum MapShape {
    Round,
    #[default]
    Jagged,
}

#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq)]
pub enum AiDifficulty {
    Easy,
    #[default]
    Normal,
    Hard,
    Crazy,
}
