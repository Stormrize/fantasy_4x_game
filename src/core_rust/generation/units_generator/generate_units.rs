use rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::core_rust::generation::races_generation::capitals_generation::CapitalsPos;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::GenerationConfig;
use crate::world::game_resources::data::unit::UnitType;

#[derive(Clone)]
struct Unit {
    pub id: Id,
    pub player_race: PlayerRace,
    pub unit_type: UnitType,
}

pub struct UnitsMapNoBevy {
    pub units: Vec<Option<Unit>>,
    pub size: usize,
}

impl UnitsMapNoBevy {
    pub fn new(config: &GenerationConfig, capital_coord: &CapitalsPos) -> Self {
        let terrain_size = config.terrain_size;
        let mut data: Vec<Option<Unit>> = vec![None; terrain_size * terrain_size];
        let map = &capital_coord.data;
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let mut possible_positions: [(i32, i32); 8] = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 0),
            (1, 1),
        ];

        for ((id, player_race), coord) in map {
            possible_positions.shuffle(&mut rng);
            for i in 0..2 {
                let x = (coord.0 as i32 + possible_positions[i].0) as usize;
                let y = (coord.1 as i32 + possible_positions[i].1) as usize;

                let index = x * terrain_size + y;
                data[index] = Some(Unit {
                    id: *id,
                    player_race: player_race.clone(),
                    unit_type: UnitType::Helper,
                });
            }
        }

        Self {
            units: data,
            size: terrain_size,
        }
    }
}
