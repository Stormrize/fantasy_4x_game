use noise::{NoiseFn, Perlin};
use voronoice::Point;

use crate::core_rust::generation::{
    races_generation::capitals_generation::{CapitalCoord, CapitalsPos},
    terrain_generator::PerlinPlaceholder,
};
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::GenerationConfig;

pub struct PlayerInfluenceCell {
    pub player_id: Id,
    pub player_race: PlayerRace,
    pub capital: CapitalCoord,
    pub tile_indices: Vec<usize>,
}

pub struct PlayerInfluenceMap {
    pub map: Vec<Option<usize>>,
    pub cells: Vec<PlayerInfluenceCell>,
}

impl PlayerInfluenceMap {
    pub fn voronoi_diagram_for_players(
        config: &GenerationConfig,
        player_terrain_data: &CapitalsPos,
        perlin_placeholder: &PerlinPlaceholder,
    ) -> PlayerInfluenceMap {
        let terrain_size = config.terrain_size;

        if player_terrain_data.data.is_empty() {
            return PlayerInfluenceMap {
                map: vec![None; terrain_size * terrain_size],
                cells: Vec::new(),
            };
        }

        let sites = player_terrain_data
            .data
            .iter()
            .map(|coord| Point {
                x: coord.1.0 as f64,
                y: coord.1.1 as f64,
            })
            .collect::<Vec<_>>();

        let mut cells = player_terrain_data
            .data
            .iter()
            .map(|coord| PlayerInfluenceCell {
                player_id: coord.0.0,
                player_race: coord.0.1.clone(),
                capital: coord.1,
                tile_indices: Vec::new(),
            })
            .collect::<Vec<_>>();

        let mut map = vec![None; terrain_size * terrain_size];

        for row in 0..terrain_size {
            for col in 0..terrain_size {
                let capital = nearest_capital_index_noisy(
                    row,
                    col,
                    &sites,
                    &perlin_placeholder.perlin_main,
                    config.noise_scale,
                    config.noise_strength,
                );
                let tile_index = row * terrain_size + col;

                map[tile_index] = Some(capital);
                cells[capital].tile_indices.push(tile_index);
            }
        }

        PlayerInfluenceMap { map, cells }
    }
}

fn nearest_capital_index_noisy(
    row: usize,
    col: usize,
    sites: &[Point],
    perlin: &Perlin,
    noise_scale: f32,
    noise_strength: f32,
) -> usize {
    let x = row as f64;
    let y = col as f64;

    let dx =
        perlin.get([x * noise_scale as f64, y * noise_scale as f64, 0.0]) * noise_strength as f64;
    let dy =
        perlin.get([x * noise_scale as f64, y * noise_scale as f64, 100.0]) * noise_strength as f64;

    let noisy_x = x + dx;
    let noisy_y = y + dy;

    sites
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            let distance_a = squared_distance(noisy_x, noisy_y, a.x, a.y);
            let distance_b = squared_distance(noisy_x, noisy_y, b.x, b.y);

            distance_a.total_cmp(&distance_b)
        })
        .map(|(index, _)| index)
        .expect("voronoi sites should not be empty")
}

#[inline]
fn squared_distance(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let dx = ax - bx; // x
    let dy = ay - by; // z

    dx * dx + dy * dy
}
