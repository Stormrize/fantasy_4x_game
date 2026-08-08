use bevy::color::Color;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ResourceType {
    //Common
    Small(String),
    Medium(String),
    Large(String),

    //Rare

    //None
    #[default]
    None = 255,
}

// impl ResourceType {
//     pub const ALL: [ResourceType; 10] = [
//         ResourceType::SmallElf,
//         ResourceType::MediumElf,
//         ResourceType::LargeElf,
//         ResourceType::SmallHuman,
//         ResourceType::MediumHuman,
//         ResourceType::LargeHuman,
//         ResourceType::SmallGnome,
//         ResourceType::MediumGnome,
//         ResourceType::LargeGnome,
//         ResourceType::None,
//     ];
//
//     pub fn color(self) -> Color {
//         // TODO добавить пути к реальным текстурам
//
//         match self {
//             ResourceType::SmallElf => Color::srgb(0.0, 0.33, 0.0),
//             ResourceType::MediumElf => Color::srgb(0.0, 0.66, 0.0),
//             ResourceType::LargeElf => Color::srgb(0.0, 0.99, 0.0),
//             ResourceType::SmallHuman => Color::srgb(0.33, 0.0, 0.0),
//             ResourceType::MediumHuman => Color::srgb(0.66, 0.0, 0.0),
//             ResourceType::LargeHuman => Color::srgb(0.99, 0.0, 0.0),
//             ResourceType::SmallGnome => Color::srgb(0.13, 0.09, 0.06),
//             ResourceType::MediumGnome => Color::srgb(0.24, 0.17, 0.12),
//             ResourceType::LargeGnome => Color::srgb(0.40, 0.26, 0.13),
//             ResourceType::None => Color::srgb(0.0, 0.0, 0.0),
//         }
//     }
//
//     pub fn index(self) -> usize {
//         match self {
//             ResourceType::SmallElf => 0,
//             ResourceType::MediumElf => 1,
//             ResourceType::LargeElf => 2,
//             ResourceType::SmallHuman => 3,
//             ResourceType::MediumHuman => 4,
//             ResourceType::LargeHuman => 5,
//             ResourceType::SmallGnome => 6,
//             ResourceType::MediumGnome => 7,
//             ResourceType::LargeGnome => 8,
//             ResourceType::None => 9,
//         }
//     }
// }