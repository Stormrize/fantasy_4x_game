use crate::world::{game_config::GameConfig, game_resources::fog_map::FogMap};

impl FogMap {
    pub fn new(config: &GameConfig) -> Self {
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

        self.map[row * self.size + col]
    }
}
