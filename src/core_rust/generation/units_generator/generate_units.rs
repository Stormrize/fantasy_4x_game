use rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::core_rust::generation::races_generation::capitals_generation::CapitalsPos;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::GenerationConfig;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::unit::UnitType;
use crate::world::game_resources::terrain_map::TerrainMap;

#[derive(Clone)]
pub struct Unit {
    pub id: Id,
    pub player_race: PlayerRace,
    pub unit_type: UnitType,
}

pub struct UnitsMapNoBevy {
    pub units: Vec<Option<Unit>>,
    pub size: usize,
}

impl UnitsMapNoBevy {
    pub fn new(
        config: &GenerationConfig,
        capital_coord: &CapitalsPos,
        terrain: &TerrainMap,
    ) -> Self {
        let terrain_size = config.terrain_size;
        let mut data: Vec<Option<Unit>> = vec![None; terrain_size * terrain_size];
        let map = &capital_coord.data;
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let mut wished_positons: [(i32, i32); 8] = [
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
            let mut helpers_spawned: u8 = 0;
            wished_positons.shuffle(&mut rng);
            for position in wished_positons {
                let x = (coord.0 as i32 + position.0) as usize;
                let y = (coord.1 as i32 + position.1) as usize;

                if !matches!(terrain.get(x, y), BiomeType::SmallBiome(_, _)) {
                    continue;
                }

                let index = x * terrain_size + y;
                data[index] = Some(Unit {
                    id: *id,
                    player_race: player_race.clone(),
                    unit_type: UnitType::Helper,
                });

                helpers_spawned += 1;
                if helpers_spawned == 2 {
                    break;
                }
            }
            if helpers_spawned < 2 {
                println!("fix me helpers not enough");
            }
        }

        Self {
            units: data,
            size: terrain_size,
        }
    }
}
