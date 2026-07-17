use bevy::color::Color;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ResourceType {
    //Common
    Food = 0,
    Stone = 1,
    Wood = 2,
    MagicShards = 3,

    //Rare

    //None
    #[default]
    None = 255
}


impl ResourceType {

    pub const ALL: [ResourceType; 5] = [
        ResourceType::Food,
        ResourceType::Wood,
        ResourceType::Stone,
        ResourceType::MagicShards,
        ResourceType::None,
    ];
    
    
    pub fn score_rate(&self) -> u8 {
        match self {
            ResourceType::Food =>  25,
            ResourceType::Wood => 20,
            ResourceType::Stone => 20,
            ResourceType::MagicShards => 40,
            ResourceType::None => 0
        }
    }

    pub fn color(self) -> Color {
        // TODO добавить пути к реальным текстурам

        match self {
            ResourceType::Food => Color::srgb(1.0, 1.0, 0.0),
            ResourceType::Wood => Color::srgb(0.55, 0.27, 0.075),
            ResourceType::Stone => Color::srgb(0.5, 0.5, 0.5),
            ResourceType::MagicShards => Color::srgb(0.5, 0.0, 0.5),
            ResourceType::None => Color::srgb(0.0, 0.0, 0.0),
        }
    }

    pub fn index(self) -> usize {
        match self {
            ResourceType::Food => 0,
            ResourceType::Wood => 1,
            ResourceType::Stone => 2,
            ResourceType::MagicShards => 3,
            ResourceType::None => 4,
        }
    }
}