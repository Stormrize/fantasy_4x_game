use crate::world::ui_and_graphic_resources::player_registry::PlayerId;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Unit {
    //вода
    WaterMageBeginner(PlayerId),
    WaterMageMaster(PlayerId),

    //огонь
    FireMageBeginner(PlayerId),
    FireMageMaster(PlayerId),

    //камень
    StoneMageBeginner(PlayerId),
    StoneMageMaster(PlayerId),

    ResourceGathererBuilder(PlayerId),
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
            ResourceGathererBuilder(_) => 6,
            None => unreachable!(),
        }
    }

    pub fn inside(self) -> Option<PlayerId> {
        use Unit::*;
        match self {
            WaterMageBeginner(p)
            | WaterMageMaster(p)
            | FireMageBeginner(p)
            | FireMageMaster(p)
            | StoneMageBeginner(p)
            | StoneMageMaster(p)
            | ResourceGathererBuilder(p) => Some(p),

            None => Option::None,
        }
    }
}
