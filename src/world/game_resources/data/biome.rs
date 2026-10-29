use crate::world::components::{Id, PlayerRace};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BiomeType {
    Water,
    SmallBiome,
    MediumBiome,
    LargeBiome,
    #[default]
    Ground,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Biome {
    pub biome_type: BiomeType,
    pub id: Option<Id>,
    pub owner_race: PlayerRace,
}

impl Biome {
    pub fn id_inside(&self) -> Option<Id> {
        self.id
    }

    pub fn owner_race(&self) -> &PlayerRace {
        &self.owner_race
    }
}

impl BiomeType {
    pub fn index(&self) -> usize {
        use BiomeType::*;
        match self {
            Water => 0,
            SmallBiome => 1,
            MediumBiome => 2,
            LargeBiome => 3,
            Ground => unreachable!(),
        }
    }
}
