use crate::world::components::PlayerRace;
use crate::world::game_configs::GenerationConfig;
use crate::world::game_resources::data::biome::BiomeType::Water;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::{
    core_rust::generation::terrain_generator::{
        PerlinPlaceholder,
        noise::{fbm, normalise, sample},
    },
    world::game_resources::data::biome::Biome,
};

impl TerrainMap {
    pub fn new_global_terrain_map(
        config: &GenerationConfig,
        perlin_placeholder: &PerlinPlaceholder,
    ) -> Self {
        let terrain_size = config.terrain_size;

        let mut data = vec![Biome::default(); terrain_size * terrain_size];

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
                    data[index] = Biome {
                        biome_type: Water,
                        id: None,
                        owner_race: PlayerRace::default(),
                    };
                }
            }
        }

        Self {
            map: data,
            size: terrain_size,
        }
    }
}
