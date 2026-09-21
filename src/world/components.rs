use bevy::prelude::Component;
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Component, Clone, Debug, Eq, PartialEq, Hash)]
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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    LocalPlayerId(u8),
    MultiPlayerId(u64),
    Unoccupied,
}

impl Id {
    pub fn default_local_id() -> Self {
        Self::LocalPlayerId(255)
    }

    pub fn default_multi_id() -> Self {
        Self::MultiPlayerId(u64::MAX)
    }
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Cost {
    pub upgrade_cost: u16,
    pub maintenance: u16,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum MapSize {
    Tiny = 8,
    Small = 16,
    #[default]
    Medium = 32,
    Big = 64,
    Large = 128,
}

impl MapSize {
    pub fn len() -> usize {
        5
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[allow(dead_code)]
#[repr(u8)]
pub enum MapShape {
    Round,
    #[default]
    Jagged,
}

impl MapShape {
    pub fn len() -> usize {
        2
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum AiDifficulty {
    Easy,
    #[default]
    Normal,
    Hard,
    Crazy,
}
impl AiDifficulty {
    pub fn len() -> usize {
        4
    }
}
