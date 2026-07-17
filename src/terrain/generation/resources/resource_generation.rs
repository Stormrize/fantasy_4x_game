use bevy::prelude::Resource;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use crate::terrain::enums::biome::BiomeType;
use crate::terrain::enums::resources::ResourceType;
use crate::terrain::generation::resources::resource_helper::place_resource;
use crate::terrain::generation::terrain::terrain_generation::TerrainMap;
use crate::terrain::terrain_config::TerrainConfig;

#[derive(Resource)]
pub struct ResourceMap {
    pub map: Vec<ResourceType>,
    pub size: usize,
}

pub struct ResourceAmounts;

impl ResourceAmounts {
    pub const COMMON: usize = 3;
    pub const RARE: usize = 30;
}

impl ResourceMap {
    pub fn new(
        terrain: &TerrainMap,
        config: &TerrainConfig,
    ) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let terrain_size = config.terrain_size * config.terrain_size;

        let mut map = vec![ResourceType::default(); terrain_size];

        Self::generate_resources(
            terrain,
            &mut map,
            &mut rng,
            ResourceAmounts::COMMON,
            config
        );

        Self {
            map,
            size: config.terrain_size
        }
    }

    fn generate_resources(
        terrain: &TerrainMap,
        map: &mut Vec<ResourceType>,
        rng: &mut ChaCha8Rng,
        window: usize,
        config: &TerrainConfig,
    ) {
        let mut candidates = Vec::with_capacity(window);
        let terrain_size = config.terrain_size;
        
        for y in 0..terrain_size {
            for x in 0..terrain_size {
                if terrain.get(y, x) == BiomeType::Water {
                    continue;
                }
        
                let index = y * terrain_size + x;
        
                if map[index] != ResourceType::None {
                    continue;
                }
        
                candidates.push((y, x));
        
                if candidates.len() == window {
                    place_resource(
                        terrain,
                        map, // Передаем плоский вектор дальше в хелпер
                        rng,
                        &candidates,
                        config
                    );
                    candidates.clear();
                }
            }
        }

        if !candidates.is_empty() {
            place_resource(
                terrain,
                map,
                rng,
                &candidates,
                config
            );
            candidates.clear();
        }
    }

    #[inline]
    pub fn get(
        &self,
        col: usize,
        row: usize,
    ) -> ResourceType {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        let index = row * self.size + col;
        self.map[index]
    }
}