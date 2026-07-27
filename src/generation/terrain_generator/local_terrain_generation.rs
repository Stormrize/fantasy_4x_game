use crate::world::enums::biome::BiomeType;
use crate::world::terrain_config::TerrainConfig;
use crate::generation::races_generation::players_or_bots::voronoi_diagram::{PlayerInfluenceCell, PlayerInfluenceMap};
use crate::generation::terrain_generator::noise::{fbm, normalise};
use crate::generation::terrain_generator::PerlinPlaceholder;
use crate::world::enums::biome::BiomeType::{LargeElf, LargeGnome, LargeHuman, MediumElf, MediumGnome, MediumHuman, SmallElf, SmallGnome, SmallHuman};
use crate::world::enums::races::FantasyRace;
use crate::world::terrain_map::{GlobalTerrainMap, TerrainMap};

impl TerrainMap {
    pub fn new(
        config: &TerrainConfig,
        perlin_placeholder: &PerlinPlaceholder,
        global_terrain: &GlobalTerrainMap,
        influence_map: &PlayerInfluenceMap,
    ) -> Self {
        let size = config.terrain_size;
        let mut map = global_terrain.map.clone();

        for cell in &influence_map.cells {
            apply_local_biomes(config, perlin_placeholder, global_terrain, &mut map, cell);
        }

        Self { map, size }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> BiomeType {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        let index = row * self.size + col;
        self.map[index]
    }
}

fn apply_local_biomes(
    config: &TerrainConfig,
    perlin_placeholder: &PerlinPlaceholder,
    global_terrain: &GlobalTerrainMap,
    map: &mut Vec<BiomeType>,
    cell: &PlayerInfluenceCell,
) {
    let size = config.terrain_size;
    let seed_offset = (cell.fantasy_race as u8 as f64 + 1.0) * config.local_seed_offset;

    for &tile_index in &cell.tile_indices {
        if global_terrain.map[tile_index] == BiomeType::Water {
            map[tile_index] = BiomeType::Water;
            continue;
        }
        let row = tile_index / size;
        let col = tile_index % size;

        let is_capital = row == cell.capital.0 && col == cell.capital.1;

        let nx = ((col as f64 - cell.capital.1 as f64) / size as f64) * config.local_frequency
            + seed_offset;
        let ny = ((row as f64 - cell.capital.0 as f64) / size as f64) * config.local_frequency
            + seed_offset;

        let value = normalise(fbm(
            &perlin_placeholder.perlin_local,
            nx,
            ny,
            config.local_octaves,
            config.local_persistence,
            config.local_lacunarity,
        ));

        map[tile_index] = choose_local_biome(value, config, cell.fantasy_race, is_capital);
    }
}

fn choose_local_biome(value: f64, config: &TerrainConfig, fantasy_race: FantasyRace, is_capital: bool) -> BiomeType {
    use FantasyRace::*;
    
    match fantasy_race {
        FireMages => {
            if value <= config.local_small_threshold || is_capital {
                SmallHuman
            } else if value <= config.local_middle_threshold {
                MediumHuman
            } else {
                LargeHuman
            }
        }
        WaterMages => {
            if value <= config.local_small_threshold || is_capital {
                SmallElf
            } else if value <= config.local_middle_threshold {
                MediumElf
            } else {
                LargeElf
            }
        }
        StoneMages => {
            if value <= config.local_small_threshold || is_capital {
                SmallGnome
            } else if value <= config.local_middle_threshold {
                MediumGnome
            } else {
                LargeGnome
            }
        },
    }
}
