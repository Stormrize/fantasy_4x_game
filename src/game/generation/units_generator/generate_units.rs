use rand::prelude::{IndexedRandom, SliceRandom};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use crate::generation::races_generation::players_or_bots::generate_players_positions::PlayerTerrainData;
use crate::world::enums::units::Unit;
use crate::world::game_config::GameConfig;
use crate::world::unit_map::UnitsMap;

impl UnitsMap {
    pub fn new(config: &GameConfig, capital_coord: &PlayerTerrainData) -> Self {
        let terrain_size = config.terrain_size;
        let mut data = vec![Unit::default(); terrain_size * terrain_size];
        let map = &capital_coord.data;
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let mut possible_positions: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 0), (1, 1)];

        for coord in map {
            possible_positions.shuffle(&mut rng);
            for i in 0..2 {
                let x = (coord.1.0 as i32 + possible_positions[i].0) as usize;
                let y = (coord.1.1 as i32 + possible_positions[i].1) as usize;

                let index = x * terrain_size + y;

                data[index] = Unit::ResourceGatherer;
            }
        }

        Self {
            units: data,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Unit {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        let index = row * self.size + col;
        self.units[index]
    }
}