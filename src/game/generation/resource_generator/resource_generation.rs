use crate::world::{
    game_resources::{
        data::biome::BiomeType, data::resources::ResourceType, resource_map::ResourceMap,
        terrain_map::TerrainMap,
    },
    game_configs::GenerationConfig
};
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct ResourceWindow;

impl ResourceWindow {
    pub const COMMON: usize = 3;
}

impl ResourceMap {
    pub fn new(config: &GenerationConfig, terrain: &TerrainMap) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let terrain_size = config.terrain_size;
        let mut map = vec![ResourceType::None; terrain_size * terrain_size];
        let mut windows: [Vec<(usize, usize)>; 3] = Default::default();

        for row in 0..terrain_size {
            for col in 0..terrain_size {
                let biome = &terrain.get(row, col);
                let resource = resource_for_biome(biome);

                if resource == ResourceType::None {
                    continue;
                }

                let window = &mut windows[resource_window_index(&resource)];

                window.push((row, col));

                if window.len() == ResourceWindow::COMMON {
                    place_random_resource(&mut map, terrain_size, window, resource, &mut rng);
                    window.clear();
                }
            }
        }

        //note resource_window_index неважно ресурс какой расы поэтому ресурсы других цветов могут быть на поле другой расы пофиксить позже

        Self {
            map,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> &ResourceType {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        &self.map[row * self.size + col]
    }
}

fn resource_for_biome(biome: &BiomeType) -> ResourceType {
    use BiomeType::*;
    use ResourceType::*;
    match biome {
        SmallBiome(player_race) => Small(player_race.clone()),
        MediumBiome(player_race) => Medium(player_race.clone()),
        LargeBiome(player_race) => Large(player_race.clone()),
        _ => None,
    }
}

fn resource_window_index(resource: &ResourceType) -> usize {
    use ResourceType::*;
    match resource {
        Small(_) => 0,
        Medium(_) => 1,
        Large(_) => 2,
        _ => unreachable!("resource_for_biome(ResourceType::None)"),
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
