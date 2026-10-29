use crate::core_rust::generation::races_generation::capitals_generation::CapitalCoord;
use crate::core_rust::generation::races_generation::capitals_generation::voronoi_diagram::{
    PlayerInfluenceCell, PlayerInfluenceMap,
};
use crate::core_rust::generation::{
    terrain_generator::PerlinPlaceholder,
    terrain_generator::noise::{fbm, normalise},
};
use crate::world::game_configs::GenerationConfig;
use crate::world::game_resources::data::biome::{Biome, BiomeType};
use crate::world::game_resources::terrain_map::TerrainMap;

const CAPITAL_SMALL_BIOME_RADIUS: f64 = 0.05;

impl TerrainMap {
    pub fn final_terrain_map(
        &mut self,
        config: &GenerationConfig,
        perlin_placeholder: &PerlinPlaceholder,
        influence_map: &PlayerInfluenceMap,
    ) {
        for (cell_index, cell) in influence_map.cells.iter().enumerate() {
            apply_local_biomes(
                config,
                perlin_placeholder,
                self,
                cell,
                influence_map,
                cell_index as u8,
            );
        }
    }
}

fn apply_local_biomes(
    config: &GenerationConfig,
    perlin_placeholder: &PerlinPlaceholder,
    terrain_map: &mut TerrainMap,
    cell: &PlayerInfluenceCell,
    influence_map: &PlayerInfluenceMap,
    cell_index: u8,
) {
    let size = config.terrain_size;
    let seed_offset = (cell_index as f64 + 1.0) * config.local_seed_offset;

    for &tile_index in &cell.tile_indices {
        let row = tile_index / size;
        let col = tile_index % size;

        // Вода остаётся водой: игроку достаётся только та, что рядом со столицей.
        if terrain_map.get_biome_type(row, col) == BiomeType::Water {
            terrain_map.map[tile_index] = Biome {
                biome_type: BiomeType::Water,
                id: influence_map.get_id(row, col),
                owner_race: cell.player_race.clone(),
            };
            continue;
        }

        let nx = ((row as f64 - cell.capital.1 as f64) / size as f64) * config.local_frequency
            + seed_offset;
        let ny = ((col as f64 - cell.capital.0 as f64) / size as f64) * config.local_frequency
            + seed_offset;

        let value = normalise(fbm(
            &perlin_placeholder.perlin_local,
            nx,
            ny,
            config.local_octaves,
            config.local_persistence,
            config.local_lacunarity,
        ));

        // Возле столицы биом мельче: сдвигаем значение вниз.
        let value = ((value - capital_small_biome_bias(row, col, cell.capital, size))
            .clamp(0.0, 1.0)
            * 255.0) as u8;

        terrain_map.map[tile_index] = Biome {
            biome_type: choose_local_biome(value, config),
            id: influence_map.get_id(row, col),
            owner_race: cell.player_race.clone(),
        };
    }
}

fn capital_small_biome_bias(
    row: usize,
    col: usize,
    capital: CapitalCoord,
    terrain_size: usize,
) -> f64 {
    let dx = col.abs_diff(capital.1) as f64;
    let dy = row.abs_diff(capital.0) as f64;
    let distance = dx.hypot(dy) / terrain_size as f64;

    (1.0 - distance / CAPITAL_SMALL_BIOME_RADIUS).clamp(0.0, 1.0)
}

fn choose_local_biome(value: u8, config: &GenerationConfig) -> BiomeType {
    if value <= config.local_small_threshold {
        BiomeType::SmallBiome
    } else if value <= config.local_middle_threshold {
        BiomeType::MediumBiome
    } else {
        BiomeType::LargeBiome
    }
}
