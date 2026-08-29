use crate::world::game_config::PlayerRace;
use crate::world::game_resources::data::biome::BiomeType::{
    Ground, LargeBiome, MediumBiome, SmallBiome, Water,
};
use crate::world::ui_and_graphic_resources::player_registry::Id;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum BiomeType {
    Water(Id, PlayerRace),
    SmallBiome(Id, PlayerRace),
    MediumBiome(Id, PlayerRace),
    LargeBiome(Id, PlayerRace),
    Ground,
}

impl BiomeType {
    pub fn index(&self) -> usize {
        use BiomeType::*;
        match self {
            Water(_, _) => 0,
            SmallBiome(_, _) => 1,
            MediumBiome(_, _) => 2,
            LargeBiome(_, _) => 3,
            Ground => 255,
        }
    }

    pub fn id_inside(self) -> Option<Id> {
        use BiomeType::*;
        match self {
            SmallBiome(id, _) | MediumBiome(id, _) | LargeBiome(id, _) | Water(id, _) => Some(id),

            Ground => None,
        }
    }

    pub fn race_inside(self) -> Option<PlayerRace> {
        match self {
            SmallBiome(_, p) | MediumBiome(_, p) | LargeBiome(_, p) => Some(p),

            Ground | Water(_, _) => None,
        }
    }
}
