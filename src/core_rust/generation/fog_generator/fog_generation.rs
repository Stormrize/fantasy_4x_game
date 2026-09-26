use crate::core_rust::generation::races_generation::capitals_generation::CapitalsPos;
use crate::world::{game_configs::GenerationConfig, game_resources::fog_map::FogMap};

impl FogMap {
    pub fn new(config: &GenerationConfig, capital_coord: &CapitalsPos) -> Self {
        let terrain_size = config.terrain_size;
        let data = vec![false; terrain_size * terrain_size];

        Self {
            map: data,
            size: terrain_size,
        }
    }
}
