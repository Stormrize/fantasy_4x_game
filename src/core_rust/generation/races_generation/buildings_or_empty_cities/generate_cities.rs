use bevy::prelude::Entity;
use crate::core_rust::generation::races_generation::players_or_bots::generate_players_positions::PlayersTerrainData;
use crate::world::{
    game_configs::GenerationConfig,
    game_resources::{
        buildings_map::BuildingMap,
        data::{biome::BiomeType, building::BuildingType},
        terrain_map::TerrainMap,
    },
};

impl BuildingMap {
    pub fn new(
        config: &GenerationConfig,
        plater_terrain_data: &PlayersTerrainData,
        terrain: &TerrainMap,
    ) -> BuildingMap {
        let terrain_size = config.terrain_size;
        let players_positions = &plater_terrain_data.data;
        let mut cities: Vec<Option<Entity>> = vec![None; terrain_size * terrain_size];
        let mut blocked = vec![false; terrain_size * terrain_size];

        for player in players_positions {
            let x = player.1.0;
            let y = player.1.1;
            let player_id = player.0;
            cities[x * terrain_size + y] = ;
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

                let race = match biome {
                    BiomeType::SmallBiome(race) => race,
                    _ => continue,
                };

                let index = x * terrain_size + y;

                if blocked[index] {
                    continue;
                }

                //todo
                cities[index] = BuildingType::EmptyCities(race.clone());
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
            map: cities,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> &BuildingType {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        &self.map[row * self.size + col]
    }
}
