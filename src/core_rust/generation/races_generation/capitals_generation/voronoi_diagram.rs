use noise::{NoiseFn, Perlin};
use voronoice::Point;

use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::GenerationConfig;
use crate::{
    core_rust::generation::{
        races_generation::capitals_generation::{CapitalCoord, CapitalsPos},
        terrain_generator::PerlinPlaceholder,
    },
    world::components::CAPITAL_CONTROL_RADIUS,
};

pub struct PlayerInfluenceCell {
    pub player_id: Id,
    pub player_race: PlayerRace,
    pub capital: CapitalCoord,
    pub tile_indices: Vec<usize>,
}

pub struct PlayerInfluenceMap {
    /// id игрока для клеток под прямым контролем столицы, иначе `None`.
    pub map: Vec<Option<Id>>,
    pub cells: Vec<PlayerInfluenceCell>,
    /// Длина стороны карты (не количество клеток).
    pub size: usize,
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
                map: Vec::new(),
                cells: Vec::new(),
                size: 0,
            };
        }

        let sites = player_terrain_data
            .data
            .iter()
            .map(|(_, coord)| Point {
                x: coord.0 as f64,
                y: coord.1 as f64,
            })
            .collect::<Vec<_>>();

        let mut cells = player_terrain_data
            .data
            .iter()
            .map(|((id, race), coord)| PlayerInfluenceCell {
                player_id: *id,
                player_race: race.clone(),
                capital: *coord,
                tile_indices: Vec::new(),
            })
            .collect::<Vec<_>>();

        let mut map = vec![None; terrain_size * terrain_size];
        let noise_scale = config.noise_scale as f64;
        let noise_strength = config.noise_strength as f64;

        for row in 0..terrain_size {
            for col in 0..terrain_size {
                let capital_index = nearest_capital_index_noisy(
                    row,
                    col,
                    &sites,
                    &perlin_placeholder.perlin_main,
                    noise_scale,
                    noise_strength,
                );
                let tile_index = row * terrain_size + col;
                let capital = cells[capital_index].capital;

                if is_within_radius(row, col, capital, CAPITAL_CONTROL_RADIUS) {
                    map[tile_index] = Some(cells[capital_index].player_id);
                }

                cells[capital_index].tile_indices.push(tile_index);
            }
        }

        PlayerInfluenceMap {
            map,
            cells,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get_id(&self, row: usize, col: usize) -> Option<Id> {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        self.map[row * self.size + col]
    }
}

#[inline]
fn is_within_radius(row: usize, col: usize, capital: CapitalCoord, radius: usize) -> bool {
    row.abs_diff(capital.0) <= radius && col.abs_diff(capital.1) <= radius
}

fn nearest_capital_index_noisy(
    row: usize,
    col: usize,
    sites: &[Point],
    perlin: &Perlin,
    noise_scale: f64,
    noise_strength: f64,
) -> usize {
    let x = row as f64;
    let y = col as f64;

    let dx = perlin.get([x * noise_scale, y * noise_scale, 0.0]) * noise_strength;
    let dy = perlin.get([x * noise_scale, y * noise_scale, 100.0]) * noise_strength;

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
    let dx = ax - bx;
    let dy = ay - by;

    dx * dx + dy * dy
}
