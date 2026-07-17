use bevy::color::Color;
use crate::terrain::enums::biome::BiomeType;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum BuildingType {
    Capital,
    Farm,
    Sawmill,
    StoneQuarries,
    MagicalCentre,
    House,
    ResearchCenter,
    #[default]
    None
}

impl BuildingType {
    pub const ALL: [BuildingType; 8] = [
        BuildingType::Capital,
        BuildingType::Farm,
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
            _ => Color::srgb(0.0, 0.0, 0.0),
        }
    }

    pub fn index(self) -> usize {
        match self {
            BuildingType::Capital => 0,
            BuildingType::Farm => 1,
            BuildingType::Sawmill => 2,
            BuildingType::StoneQuarries => 3,
            BuildingType::MagicalCentre => 4,
            BuildingType::House => 5,
            BuildingType::ResearchCenter => 6,
            BuildingType::None => 7,
        }
    }
}