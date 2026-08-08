use crate::world::enums::biome::BiomeType;
use crate::world::enums::buildings::BuildingType;
use crate::world::game_config::GameConfig;
use bevy::prelude::Resource;
use crate::generation::races_generation::players_or_bots::generate_players_positions::PlayerTerrainData;
use crate::world::buildings_map::BuildingMap;
use crate::world::terrain_map::{GlobalTerrainMap, TerrainMap};

impl BuildingMap {
    pub fn new(config: &GameConfig, plater_terrain_data: &PlayerTerrainData, terrain: &TerrainMap) -> BuildingMap {
        let terrain_size = config.terrain_size;
        let players_positions = &plater_terrain_data.data;
        let mut cities = vec![(BuildingType::None, 255); terrain_size * terrain_size];
        let mut blocked = vec![false; terrain_size * terrain_size];

        for player in players_positions {
            let x = player.1.0;
            let y = player.1.1;
            cities[x * terrain_size + y] = (BuildingType::Capital, player.0.0);
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
                if biome != BiomeType::SmallElf && biome != BiomeType::SmallGnome && biome != BiomeType::SmallHuman {
                    continue;
                }

                let index = x * terrain_size + y;

                if blocked[index] {
                    continue;
                }
                cities[index] = (BuildingType::EmptyCities, 255);
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
    pub fn get(&self, row: usize, col: usize) -> (BuildingType, u8) {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        let index = row * self.size + col;
        self.map[index]
    }
}
