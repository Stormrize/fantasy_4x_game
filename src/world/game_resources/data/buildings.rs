use crate::world::game_config::PlayerRace;

#[derive(Default, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum BuildingType {
    Capital(PlayerRace),
    EmptyCities(PlayerRace),
    MediumBuilding(PlayerRace),
    LargeBuilding(PlayerRace),
    SpawningBuilding(PlayerRace),
    House(PlayerRace),
    ResearchCenter(PlayerRace),
    #[default]
    None,
}

impl BuildingType {
    pub fn index(&self) -> usize {
        use BuildingType::*;
        match self {
            Capital(_) => 0,
            EmptyCities(_) => 1,
            MediumBuilding(_) => 2,
            LargeBuilding(_) => 3,
            SpawningBuilding(_) => 4,
            House(_) => 5,
            ResearchCenter(_) => 6,
            None => unreachable!(),
        }
    }

    pub fn inside(&self) -> Option<&PlayerRace> {
        use BuildingType::*;
        match self {
            Capital(p) | EmptyCities(p) | MediumBuilding(p) | LargeBuilding(p)
            | SpawningBuilding(p) | House(p) | ResearchCenter(p) => Some(p),

            None => Option::None,
        }
    }
}
