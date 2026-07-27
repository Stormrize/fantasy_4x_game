use bevy::tasks::{ComputeTaskPool, ParallelSlice, ParallelSliceMut};
use super::noise::{fbm, normalise, sample};
use crate::world::enums::biome::BiomeType;
use crate::world::game_config::TerrainConfig;
use crate::generation::terrain_generator::PerlinPlaceholder;
use crate::world::terrain_map::GlobalTerrainMap;
impl GlobalTerrainMap {
    pub fn new(config: &TerrainConfig, perlin_placeholder: &PerlinPlaceholder) -> Self {
        let terrain_size = config.terrain_size;

        let mut data = vec![BiomeType::default(); terrain_size * terrain_size];

        //let task_pool = ComputeTaskPool::get();


        for row in 0..terrain_size {
            for col in 0..terrain_size {
                let nx = row as f64 / terrain_size as f64;
                let ny = col as f64 / terrain_size as f64;

                let large = normalise(fbm(
                    &perlin_placeholder.perlin_main,
                    nx * config.global_frequency,
                    ny * config.global_frequency,
                    config.global_octaves,
                    config.global_persistence,
                    config.global_lacunarity,
                ));

                let detail = normalise(sample(
                    &perlin_placeholder.perlin_detail,
                    col as f64 * config.global_detail_frequency,
                    row as f64 * config.global_detail_frequency,
                ));

                let combined = large * (1.0 - config.global_detail_weight)
                    + detail * config.global_detail_weight;

                let result = (combined.clamp(0.0, 1.0) * 255.0) as u8;

                let index = row * terrain_size + col;

                if result < config.water_threshold {
                    data[index] = BiomeType::Water;
                }
            }
        }

        Self { map: data, size: terrain_size }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> BiomeType {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        let index = row * self.size + col;
        self.map[index]
    }
}