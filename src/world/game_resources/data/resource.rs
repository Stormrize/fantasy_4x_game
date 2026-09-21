use crate::world::components::{Id, PlayerRace};

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ResourceType {
    //Common
    Small(Id, PlayerRace),
    Medium(Id, PlayerRace),
    Large(Id, PlayerRace),
    //Rare
}

impl ResourceType {
    pub fn index(&self) -> usize {
        use ResourceType::*;
        match self {
            Small(_, _) => 0,
            Medium(_, _) => 1,
            Large(_, _) => 2,
        }
    }

    pub fn id_inside(&self) -> Option<&Id> {
        use ResourceType::*;
        match self {
            Small(id, _) | Medium(id, _) | Large(id, _) => Some(id),
        }
    }

    pub fn race_inside(&self) -> Option<&PlayerRace> {
        use ResourceType::*;
        match self {
            Small(_, p) | Medium(_, p) | Large(_, p) => Some(p),
        }
    }
}
