use bevy::prelude::Resource;
use noise::Perlin;
use rayon::prelude::*;
use crate::terrain::enums::biome::BiomeType;
use super::noise::{fbm, normalise, sample};
use crate::terrain::terrain_config::TerrainConfig;

#[derive(Resource, Clone)]
pub struct PerlinPlaceholder {
    pub perlin_main: Perlin,
    pub perlin_detail: Perlin,
}

impl Default for PerlinPlaceholder {
    fn default() -> Self {
        Self {
            perlin_main: Perlin::new(42),
            perlin_detail: Perlin::new(42_u32.wrapping_add(1337)),
        }
    }
}

impl PerlinPlaceholder {
    pub fn new(config: &TerrainConfig) -> Self {
        Self {
            perlin_main: Perlin::new(config.seed),
            perlin_detail: Perlin::new(config.seed.wrapping_sub(1337)),
        }
    }
}

#[derive(Resource)]
pub struct TerrainMap {
    pub map: Vec<BiomeType>,
    pub size: usize,
}

impl TerrainMap {
    pub fn new(
        config: &TerrainConfig,
        perlin_placeholder: &PerlinPlaceholder,
    ) -> Self {
        let w = config.terrain_size;
        let h = config.terrain_size;
        let terrain_size = w * h;

        let mut data = vec![BiomeType::default(); terrain_size];

        data.par_chunks_mut(w)
            .enumerate()
            .for_each(|(row, row_data)| {
                for col in 0..w {
                    let nx = col as f64 / w as f64;
                    let ny = row as f64 / h as f64;

                    let large = fbm(
                        &perlin_placeholder.perlin_main,
                        nx * config.frequency,
                        ny * config.frequency,
                        config.octaves,
                        config.persistence,
                        config.lacunarity,
                    );

                    let large = normalise(large);

                    let detail = normalise(sample(
                        &perlin_placeholder.perlin_detail,
                        col as f64 * config.detail_frequency,
                        row as f64 * config.detail_frequency,
                    ));

                    let combined = large * (1.0 - config.detail_weight)
                        + detail * config.detail_weight;

                    let result = (combined.clamp(0.0, 1.0) * 255.0) as u8;

                    if result < config.water_threshold {
                        row_data[col] = BiomeType::Water;
                    } else if result < config.threshold_grass {
                        row_data[col] = BiomeType::Grass;
                    } else if result < config.threshold_forest {
                        row_data[col] = BiomeType::Forest;
                    } else {
                        row_data[col] = BiomeType::Hills;
                    }
                }
            });

        Self {
            map: data,
            size: w,
        }
    }

    #[inline]
    pub fn get(&self, col: usize, row: usize) -> BiomeType {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        let index = col * self.size + row;
        self.map[index]
    }
}