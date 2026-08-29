use crate::world::game_config::PlayerRace;
use crate::world::ui_and_graphic_resources::player_registry::Id;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum BuildingType {
    Capital,
    EmptyCities,
    MediumBuilding,
    LargeBuilding,
    SpawningBuilding,
    House,
    ResearchCenter,
}

pub struct Building {
    pub player_race: PlayerRace,
    pub id: Id,
    pub building_type: BuildingType,
    pub health: u8,
    pub level: u8,
}

impl BuildingType {
    pub fn index(&self) -> usize {
        use BuildingType::*;
        match self {
            Capital => 0,
            EmptyCities => 1,
            MediumBuilding => 2,
            LargeBuilding => 3,
            SpawningBuilding => 4,
            House => 5,
            ResearchCenter => 6,
        }
    }
}
