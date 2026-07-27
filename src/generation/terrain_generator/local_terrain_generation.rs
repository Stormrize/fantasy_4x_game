use crate::world::enums::biome::BiomeType;
use crate::world::game_config::TerrainConfig;
use crate::generation::races_generation::players_or_bots::voronoi_diagram::{PlayerInfluenceCell, PlayerInfluenceMap};
use crate::generation::terrain_generator::noise::{fbm, normalise};
use crate::generation::terrain_generator::PerlinPlaceholder;
use crate::world::enums::biome::BiomeType::{LargeElf, LargeGnome, LargeHuman, MediumElf, MediumGnome, MediumHuman, SmallElf, SmallGnome, SmallHuman};
use crate::world::enums::races::FantasyRace;
use crate::world::terrain_map::{GlobalTerrainMap, TerrainMap};

const CAPITAL_SMALL_BIOME_RADIUS: f64 = 0.05;

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

        let value = (value - capital_small_biome_bias(row, col, cell.capital, size)).clamp(0.0, 1.0);

        map[tile_index] = choose_local_biome(value, config, cell.fantasy_race);
    }
}

fn capital_small_biome_bias(
    row: usize,
    col: usize,
    capital: (usize, usize),
    terrain_size: usize,
) -> f64 {
    let dx = col as f64 - capital.1 as f64;
    let dy = row as f64 - capital.0 as f64;
    let d = dx.hypot(dy) / terrain_size as f64;

    (1.0 - d / CAPITAL_SMALL_BIOME_RADIUS).clamp(0.0, 1.0)
}

fn choose_local_biome(value: f64, config: &TerrainConfig, fantasy_race: FantasyRace) -> BiomeType {
    use FantasyRace::*;
    
    match fantasy_race {
        FireMages => {
            if value <= config.local_small_threshold {
                SmallHuman
            } else if value <= config.local_middle_threshold {
                MediumHuman
            } else {
                LargeHuman
            }
        }
        WaterMages => {
            if value <= config.local_small_threshold {
                SmallElf
            } else if value <= config.local_middle_threshold {
                MediumElf
            } else {
                LargeElf
            }
        }
        StoneMages => {
            if value <= config.local_small_threshold {
                SmallGnome
            } else if value <= config.local_middle_threshold {
                MediumGnome
            } else {
                LargeGnome
            }
        }
    }
}
