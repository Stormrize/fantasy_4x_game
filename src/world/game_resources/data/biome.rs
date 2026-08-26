use crate::world::game_config::PlayerRace;
use crate::world::game_resources::data::resources::ResourceType::{Large, Medium, Small};

#[derive(Default, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum BiomeType {
    Water(PlayerRace),
    SmallBiome(PlayerRace),
    MediumBiome(PlayerRace),
    LargeBiome(PlayerRace),
    #[default]
    Ground,
}

impl BiomeType {
    pub fn index(&self) -> usize {
        use BiomeType::*;
        match self {
            Water(_) => 0,
            SmallBiome(_) => 1,
            MediumBiome(_) => 2,
            LargeBiome(_) => 3,
            Ground => 255,
        }
    }

    pub fn inside(&self) -> Option<&PlayerRace> {
        use BiomeType::*;
        match self {
            SmallBiome(p) | MediumBiome(p) | LargeBiome(p) | Water(p) => Some(p),

            Ground => Option::None,
        }
    }
}
