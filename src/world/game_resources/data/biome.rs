use crate::world::ui_and_graphic_resources::player_registry::PlayerId;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BiomeType {
    Water(PlayerId),
    SmallBiome(PlayerId),
    MediumBiome(PlayerId),
    LargeBiome(PlayerId),
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

    pub fn inside(self) -> Option<PlayerId> {
        use BiomeType::*;
        match self {
            SmallBiome(p) | MediumBiome(p) | LargeBiome(p) | Water(p) => Some(p),

            Ground => None,
        }
    }
}
