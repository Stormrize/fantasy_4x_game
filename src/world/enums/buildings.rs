use crate::world::enums::biome::BiomeType;
use bevy::color::Color;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BuildingType {
    Capital,
    EmptyCities,
    Sawmill,
    StoneQuarries,
    MagicalCentre,
    House,
    ResearchCenter,
    #[default]
    None,
}

impl BuildingType {
    pub const ALL: [BuildingType; 8] = [
        BuildingType::Capital,
        BuildingType::EmptyCities,
        BuildingType::Sawmill,
        BuildingType::StoneQuarries,
        BuildingType::MagicalCentre,
        BuildingType::House,
        BuildingType::ResearchCenter,
        BuildingType::None,
    ];

    pub fn color(self) -> Color {
        // TODO добавить пути к реальным текстурам

        match self {
            BuildingType::Capital => Color::srgb(0.0, 1.0, 0.0),
            BuildingType::EmptyCities => Color::srgb(0.0, 0.0, 1.0),
            _ => Color::srgb(0.0, 0.0, 0.0),
        }
    }

    pub fn index(self) -> usize {
        match self {
            BuildingType::Capital => 0,
            BuildingType::EmptyCities => 1,
            BuildingType::Sawmill => 2,
            BuildingType::StoneQuarries => 3,
            BuildingType::MagicalCentre => 4,
            BuildingType::House => 5,
            BuildingType::ResearchCenter => 6,
            BuildingType::None => 7,
        }
    }
}
