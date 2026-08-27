use crate::world::ui_and_graphic_resources::player_registry::PlayerId;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BuildingType {
    Capital(PlayerId),
    EmptyCities(PlayerId),
    MediumBuilding(PlayerId),
    LargeBuilding(PlayerId),
    SpawningBuilding(PlayerId),
    House(PlayerId),
    ResearchCenter(PlayerId),
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

    pub fn inside(self) -> Option<PlayerId> {
        use BuildingType::*;
        match self {
            Capital(p) | EmptyCities(p) | MediumBuilding(p) | LargeBuilding(p)
            | SpawningBuilding(p) | House(p) | ResearchCenter(p) => Some(p),

            None => Option::None,
        }
    }
}
