use bevy::color::Color;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BiomeType {
    Water,
    //Elf
    #[default]
    SmallElf,
    MediumElf,
    LargeElf,

    //Human
    SmallHuman,
    MediumHuman,
    LargeHuman,

    //Gnome
    SmallGnome,
    MediumGnome,
    LargeGnome,
}

impl BiomeType {
    pub const ALL: [BiomeType; 10] = [
        BiomeType::Water,
        BiomeType::SmallElf,
        BiomeType::MediumElf,
        BiomeType::LargeElf,
        BiomeType::SmallHuman,
        BiomeType::MediumHuman,
        BiomeType::LargeHuman,
        BiomeType::SmallGnome,
        BiomeType::MediumGnome,
        BiomeType::LargeGnome
    ];

    pub fn color(self) -> Color {
        // TODO добавить пути к реальным текстурам

        match self {
            BiomeType::Water => Color::srgb(0.14, 0.42, 0.72),
            BiomeType::SmallElf => Color::srgb(0.0, 0.99, 0.0),
            BiomeType::MediumElf => Color::srgb(0.0, 0.66, 0.0),
            BiomeType::LargeElf => Color::srgb(0.0, 0.33, 0.0),
            BiomeType::SmallHuman => Color::srgb(0.99, 0.0, 0.0),
            BiomeType::MediumHuman => Color::srgb(0.66, 0.0, 0.0),
            BiomeType::LargeHuman => Color::srgb(0.33, 0.0, 0.0),
            BiomeType::SmallGnome => Color::srgb(0.40, 0.26, 0.13),
            BiomeType::MediumGnome => Color::srgb(0.24, 0.17, 0.12),
            BiomeType::LargeGnome => Color::srgb(0.13, 0.09, 0.06),
        }
    }

    pub fn index(self) -> usize {
        match self {
            BiomeType::Water => 0,
            BiomeType::SmallElf => 1,
            BiomeType::MediumElf => 2,
            BiomeType::LargeElf => 3,
            BiomeType::SmallHuman => 4,
            BiomeType::MediumHuman => 5,
            BiomeType::LargeHuman => 6,
            BiomeType::SmallGnome => 7,
            BiomeType::MediumGnome => 8,
            BiomeType::LargeGnome => 9,
        }
    }
    pub fn visual_height(self) -> u8 {
        match self {
            BiomeType::Water => 18,  //0.14
            BiomeType::SmallElf => 20,
            BiomeType::MediumElf => 25,
            BiomeType::LargeElf => 35,
            BiomeType::SmallHuman => 45,
            BiomeType::MediumHuman => 55,
            BiomeType::LargeHuman => 70,
            BiomeType::SmallGnome => 80,
            BiomeType::MediumGnome => 95,
            BiomeType::LargeGnome => 120,
        }
    }
}