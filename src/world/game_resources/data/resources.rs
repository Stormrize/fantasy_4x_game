use crate::world::game_config::PlayerRace;
use crate::world::game_resources::data::buildings::BuildingType::{
    Capital, EmptyCities, House, LargeBuilding, MediumBuilding, ResearchCenter, SpawningBuilding,
};

#[derive(Default, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ResourceType {
    //Common
    Small(PlayerRace),
    Medium(PlayerRace),
    Large(PlayerRace),

    //Rare

    //None
    #[default]
    None = 255,
}

impl ResourceType {
    pub fn index(&self) -> usize {
        use ResourceType::*;
        match self {
            Small(_) => 0,
            Medium(_) => 1,
            Large(_) => 2,
            None => unreachable!(),
        }
    }

    pub fn inside(&self) -> Option<&PlayerRace> {
        use ResourceType::*;
        match self {
            Small(p) | Medium(p) | Large(p) => Some(p),

            None => Option::None,
        }
    }
}
