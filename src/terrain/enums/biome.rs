use bevy::color::Color;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BiomeType {
    #[default]
    Water,
    Grass,
    Forest,
    Hills
}

impl BiomeType {
    pub const ALL: [BiomeType; 4] = [
        BiomeType::Water,
        BiomeType::Grass,
        BiomeType::Forest,
        BiomeType::Hills,
    ];

    pub fn color(self) -> Color {
        // TODO добавить пути к реальным текстурам

        match self {
            BiomeType::Water => Color::srgb(0.14, 0.42, 0.72),
            BiomeType::Grass => Color::srgb(0.27, 0.62, 0.20),
            BiomeType::Forest => Color::srgb(0.10, 0.36, 0.10),
            BiomeType::Hills => Color::srgb(0.58, 0.50, 0.30),
        }
    }

    pub fn index(self) -> usize {
        match self {
            BiomeType::Water => 0,
            BiomeType::Grass => 1,
            BiomeType::Forest => 2,
            BiomeType::Hills => 3,
        }
    }
    pub fn visual_height(self) -> u8 {
        match self {
            BiomeType::Water => 18, //0.14
            BiomeType::Grass => 33, //0.26
            BiomeType::Forest => 45, //0.35
            BiomeType::Hills => 77, //0.60
        }
    }
}