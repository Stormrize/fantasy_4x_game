use crate::core_rust::generation::races_generation::capitals_generation::CapitalsPos;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_resources::data::building::BuildingType;
use crate::world::{
    game_configs::GenerationConfig,
    game_resources::{data::biome::BiomeType, terrain_map::TerrainMap},
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Building {
    pub id: Id,
    pub player_race: PlayerRace,
    pub building_type: BuildingType,
}

#[derive(Deserialize, Debug)]
pub struct BuildingMapNoBevy {
    pub buildings: Vec<Option<Building>>,
    pub size: usize,
}

impl BuildingMapNoBevy {
    pub fn new(
        config: &GenerationConfig,
        player_terrain_data: &CapitalsPos,
        terrain: &TerrainMap,
    ) -> BuildingMapNoBevy {
        let terrain_size = config.terrain_size;
        let players_positions = &player_terrain_data.data;
        let mut cities: Vec<Option<Building>> = vec![None; terrain_size * terrain_size];
        let mut blocked = vec![false; terrain_size * terrain_size];

        for ((id, player_race), coord) in players_positions {
            let x = coord.0;
            let y = coord.1;
            cities[x * terrain_size + y] = Some(Building {
                id: *id,
                player_race: player_race.clone(),
                building_type: BuildingType::Capital,
            });
            let min_y = y.saturating_sub(3);
            let max_y = (y + 3).min(terrain_size - 1);
            let min_x = x.saturating_sub(3);
            let max_x = (x + 3).min(terrain_size - 1);

            for nx in min_x..=max_x {
                for ny in min_y..=max_y {
                    let index = nx * terrain_size + ny;
                    blocked[index] = true;
                }
            }
        }

        for x in 3..terrain_size - 2 {
            for y in 3..terrain_size - 2 {
                let biome = terrain.get(x, y);

                let (id, player_race) = match biome {
                    BiomeType::SmallBiome(id, race) => (id, race),
                    _ => continue,
                };

                let index = x * terrain_size + y;

                if blocked[index] {
                    continue;
                }

                cities[index] = Some(Building {
                    id: *id,
                    player_race: player_race.clone(),
                    building_type: BuildingType::BarbarianCity,
                });
                let min_y = y.saturating_sub(5);
                let max_y = (y + 5).min(terrain_size - 1);
                let min_x = x.saturating_sub(5);
                let max_x = (x + 5).min(terrain_size - 1);
                for nx in min_x..=max_x {
                    for ny in min_y..=max_y {
                        let index = nx * terrain_size + ny;
                        blocked[index] = true;
                    }
                }
            }
        }
        Self {
            buildings: cities,
            size: terrain_size,
        }
    }
}
