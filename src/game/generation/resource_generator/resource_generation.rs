use crate::world::enums::biome::BiomeType;
use crate::world::enums::resources::ResourceType;
use crate::world::game_config::GameConfig;
use bevy::prelude::Resource;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crate::world::resource_map::ResourceMap;
use crate::world::terrain_map::TerrainMap;

pub struct ResourceWindow;

impl ResourceWindow {
    pub const COMMON: usize = 3;
}

impl ResourceMap {
    pub fn new(config: &GameConfig, terrain: &TerrainMap) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let terrain_size = config.terrain_size;
        let mut map = vec![ResourceType::None; terrain_size * terrain_size];
        let mut windows: [Vec<(usize, usize)>; 9] = Default::default();

        for row in 0..terrain_size {
            for col in 0..terrain_size {
                let Some(resource) = resource_for_biome(terrain.get(row, col)) else {
                    continue;
                };
                let window = &mut windows[resource_window_index(resource)];

                window.push((row, col));

                if window.len() == ResourceWindow::COMMON {
                    place_random_resource(&mut map, terrain_size, window, resource, &mut rng);
                    window.clear();
                }
            }
        }

        for (resource_index, window) in windows.iter().enumerate() {
            if window.is_empty() {
                continue;
            }

            let resource = resource_by_window_index(resource_index);
            place_random_resource(&mut map, terrain_size, window, resource, &mut rng);
        }

        Self {
            map,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> ResourceType {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        self.map[row * self.size + col]
    }
}

fn place_random_resource(
    map: &mut [ResourceType],
    terrain_size: usize,
    window: &[(usize, usize)],
    resource: ResourceType,
    rng: &mut ChaCha8Rng,
) {
    let (row, col) = window[rng.random_range(0..window.len())];
    map[row * terrain_size + col] = resource;
}

fn resource_for_biome(biome: BiomeType) -> Option<ResourceType> {
    match biome {
        BiomeType::SmallElf => Some(ResourceType::SmallElf),
        BiomeType::MediumElf => Some(ResourceType::MediumElf),
        BiomeType::LargeElf => Some(ResourceType::LargeElf),
        BiomeType::SmallHuman => Some(ResourceType::SmallHuman),
        BiomeType::MediumHuman => Some(ResourceType::MediumHuman),
        BiomeType::LargeHuman => Some(ResourceType::LargeHuman),
        BiomeType::SmallGnome => Some(ResourceType::SmallGnome),
        BiomeType::MediumGnome => Some(ResourceType::MediumGnome),
        BiomeType::LargeGnome => Some(ResourceType::LargeGnome),
        BiomeType::Water => None,
    }
}

fn resource_window_index(resource: ResourceType) -> usize {
    match resource {
        ResourceType::SmallElf => 0,
        ResourceType::MediumElf => 1,
        ResourceType::LargeElf => 2,
        ResourceType::SmallHuman => 3,
        ResourceType::MediumHuman => 4,
        ResourceType::LargeHuman => 5,
        ResourceType::SmallGnome => 6,
        ResourceType::MediumGnome => 7,
        ResourceType::LargeGnome => 8,
        ResourceType::None => unreachable!("None is not generated from a biome"),
    }
}

fn resource_by_window_index(index: usize) -> ResourceType {
    match index {
        0 => ResourceType::SmallElf,
        1 => ResourceType::MediumElf,
        2 => ResourceType::LargeElf,
        3 => ResourceType::SmallHuman,
        4 => ResourceType::MediumHuman,
        5 => ResourceType::LargeHuman,
        6 => ResourceType::SmallGnome,
        7 => ResourceType::MediumGnome,
        8 => ResourceType::LargeGnome,
        _ => unreachable!("Resource window index is out of range"),
    }
}
