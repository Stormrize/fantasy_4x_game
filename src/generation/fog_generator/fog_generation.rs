use crate::world::fog_map::FogMap;
use crate::world::terrain_config::TerrainConfig;

impl FogMap {
    pub fn new(config: &TerrainConfig) -> Self {
        let terrain_size = config.terrain_size;
        let data = vec![false; terrain_size * terrain_size];
        Self {
            map: data,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> bool {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        let index = row * self.size + col;
        self.map[index]
    }
}