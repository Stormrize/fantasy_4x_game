use crate::world::game_resources::{
    data::biome::BiomeType,
    terrain_map::{GlobalTerrainMap, TerrainMap},
};

use crate::game::generation::races_generation::players_or_bots::generate_players_positions::CapitalCoord;
use crate::game::generation::terrain_generator::noise::{fbm, normalise};
use crate::game::generation::{
    races_generation::players_or_bots::voronoi_diagram::{PlayerInfluenceCell, PlayerInfluenceMap},
    terrain_generator::PerlinPlaceholder,
};
use crate::world::game_configs::GenerationConfig;
use crate::world::game_resources::data::biome::BiomeType::{
    LargeBiome, MediumBiome, SmallBiome, Water,
};
use crate::world::ui_and_graphic_resources::player_registry::PlayerId;

const CAPITAL_SMALL_BIOME_RADIUS: f64 = 0.05;

impl TerrainMap {
    pub fn new(
        config: &GenerationConfig,
        perlin_placeholder: &PerlinPlaceholder,
        global_terrain: &GlobalTerrainMap,
        influence_map: &PlayerInfluenceMap,
    ) -> Self {
        let size = config.terrain_size;
        let mut map = global_terrain.map.clone();

        let mut counter = 0;
        for cell in &influence_map.cells {
            apply_local_biomes(
                config,
                perlin_placeholder,
                global_terrain,
                &mut map,
                cell,
                counter,
            );
            counter += 1;
        }

        Self { map, size }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> &BiomeType {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        &self.map[row * self.size + col]
    }
}

fn apply_local_biomes(
    config: &GenerationConfig,
    perlin_placeholder: &PerlinPlaceholder,
    global_terrain: &GlobalTerrainMap,
    map: &mut Vec<BiomeType>,
    cell: &PlayerInfluenceCell,
    seed_offset: u8,
) {
    let size = config.terrain_size;
    let seed_offset = (seed_offset as f64 + 1.0) * config.local_seed_offset;

    for &tile_index in &cell.tile_indices {
        let row = tile_index / size;
        let col = tile_index % size;

        match global_terrain.get(row, col) {
            Water(_) => {
                map[tile_index] = Water(cell.player_id);
                continue;
            }
            _ => (),
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

        // добавляем small_biome возле столицы
        let value = ((value - capital_small_biome_bias(row, col, cell.capital, size))
            .clamp(0.0, 1.0)
            * 255.0) as u8;

        map[tile_index] = choose_local_biome(value, config, cell.player_id);
    }
}

fn capital_small_biome_bias(
    row: usize,
    col: usize,
    capital: CapitalCoord,
    terrain_size: usize,
) -> f64 {
    let dx = col as f64 - capital.1 as f64;
    let dy = row as f64 - capital.0 as f64;
    let d = dx.hypot(dy) / terrain_size as f64;

    (1.0 - d / CAPITAL_SMALL_BIOME_RADIUS).clamp(0.0, 1.0)
}

fn choose_local_biome(value: u8, config: &GenerationConfig, player_id: PlayerId) -> BiomeType {
    if value <= config.local_small_threshold {
        SmallBiome(player_id)
    } else if value <= config.local_middle_threshold {
        MediumBiome(player_id)
    } else {
        LargeBiome(player_id)
    }
}
