use crate::world::ui_and_graphic_resources::player_registry::PlayerId;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ResourceType {
    //Common
    Small(PlayerId),
    Medium(PlayerId),
    Large(PlayerId),

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

    pub fn inside(self) -> Option<PlayerId> {
        use ResourceType::*;
        match self {
            Small(p) | Medium(p) | Large(p) => Some(p),

            None => Option::None,
        }
    }
}
