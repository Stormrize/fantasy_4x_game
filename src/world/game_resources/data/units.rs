use crate::world::game_config::PlayerRace;
use crate::world::ui_and_graphic_resources::player_registry::Id;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Unit {
    //вода
    WaterMageBeginner(Id),
    WaterMageMaster(Id),

    //огонь
    FireMageBeginner(Id),
    FireMageMaster(Id),

    //камень
    StoneMageBeginner(Id),
    StoneMageMaster(Id),

    ResourceGathererBuilder(Id),
}

pub struct UnitS {
    pub player_id: Id,
    pub player_race: PlayerRace,
    pub unit: Unit,
    pub attack: u8,
    pub defense: u8,
    pub health: u8,
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
            ResourceGathererBuilder(_) => 6,
        }
    }

    pub fn inside(self) -> Option<Id> {
        use Unit::*;
        match self {
            WaterMageBeginner(p)
            | WaterMageMaster(p)
            | FireMageBeginner(p)
            | FireMageMaster(p)
            | StoneMageBeginner(p)
            | StoneMageMaster(p)
            | ResourceGathererBuilder(p) => Some(p),
        }
    }
}
