use crate::world::game_config::PlayerRace;
use crate::world::game_resources::data::buildings::BuildingType;
use crate::world::game_resources::data::buildings::BuildingType::{
    Capital, EmptyCities, House, LargeBuilding, MediumBuilding, ResearchCenter, SpawningBuilding,
};

#[derive(Default, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Unit {
    //вода
    WaterMageBeginner(PlayerRace),
    WaterMageMaster(PlayerRace),

    //огонь
    FireMageBeginner(PlayerRace),
    FireMageMaster(PlayerRace),

    //камень
    StoneMageBeginner(PlayerRace),
    StoneMageMaster(PlayerRace),

    ResourceGatherer(PlayerRace),
    #[default]
    None,
}

impl Unit {
    pub fn index(&self) -> usize {
        use Unit::*;
        match self {
            WaterMageBeginner(_) => 0,
            WaterMageMaster(_) => 1,
            FireMageBeginner(_) => 2,
            FireMageMaster(_) => 3,
            StoneMageBeginner(_) => 4,
            StoneMageMaster(_) => 5,
            ResourceGatherer(_) => 6,
            None => unreachable!(),
        }
    }

    pub fn inside(&self) -> Option<&PlayerRace> {
        use Unit::*;
        match self {
            WaterMageBeginner(p) | WaterMageMaster(p) | FireMageBeginner(p) | FireMageMaster(p)
            | StoneMageBeginner(p) | StoneMageMaster(p) | ResourceGatherer(p) => Some(p),

            None => Option::None,
        }
    }
}
