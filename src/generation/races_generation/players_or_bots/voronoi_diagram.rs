use crate::world::game_config::TerrainConfig;
use bevy::prelude::Resource;
use noise::{NoiseFn, Perlin};
use voronoice::{Point};
use crate::generation::races_generation::players_or_bots::generate_players_positions::PlayerTerrainData;
use crate::generation::terrain_generator::PerlinPlaceholder;
use crate::world::enums::races::FantasyRace;
#[derive(Clone)]
pub struct PlayerInfluenceCell {
    pub fantasy_race: FantasyRace,
    pub capital: (usize, usize),
    pub tile_indices: Vec<usize>,
}

pub struct PlayerInfluenceMap {
    pub map: Vec<Option<usize>>,
    pub cells: Vec<PlayerInfluenceCell>,
    pub size: usize,
}

impl PlayerInfluenceMap {
    pub fn voronoi_diagram_for_players(config: &TerrainConfig, player_terrain_data: &PlayerTerrainData, perlin_placeholder: &PerlinPlaceholder) -> PlayerInfluenceMap {
        let terrain_size = config.terrain_size;

        if player_terrain_data.data.is_empty() {
            return PlayerInfluenceMap {
                map: vec![None; terrain_size * terrain_size],
                cells: Vec::new(),
                size: terrain_size,
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
                fantasy_race: coord.0.1,
                capital: coord.1,
                tile_indices: Vec::new(),
            })
            .collect::<Vec<_>>();

        let mut map = vec![None; terrain_size * terrain_size];

        for row in 0..terrain_size {
            for col in 0..terrain_size {

                let capital = nearest_capital_index_noisy(row, col, &sites, &perlin_placeholder.perlin_main, config.noise_scale, config.noise_strength);
                let tile_index = row * terrain_size + col;

                map[tile_index] = Some(capital);
                cells[capital].tile_indices.push(tile_index);
            }
        }


        PlayerInfluenceMap {
            map,
            cells,
            size: terrain_size,
        }
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

    let dx = perlin.get([x * noise_scale as f64, y * noise_scale as f64, 0.0]) * noise_strength as f64;
    let dy = perlin.get([x * noise_scale as f64, y * noise_scale as f64, 100.0]) * noise_strength as f64;

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

#[cfg(false)]
fn nearest_capital_index(row: usize, col: usize, sites: &[Point]) -> usize {
    let y = row as f64;
    let x = col as f64;

    sites
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            let distance_a = squared_distance(x, y, a.x, a.y);
            let distance_b = squared_distance(x, y, b.x, b.y);

            distance_a.total_cmp(&distance_b)
        })
        .map(|(index, _)| index)
        .expect("voronoi sites should not be empty")
}

#[inline]
fn squared_distance(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let dx = ax - bx;  // x
    let dy = ay - by;  // z

    dx * dx + dy * dy
}





#[allow(dead_code)]
#[cfg(false)]
impl PlayerInfluenceMap {
    #[inline]
    pub fn get_cell_index(&self, row: usize, col: usize) -> Option<usize> {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        self.map[row * self.size + col]
    }

    pub fn voronoi_diagram_for_players(
        config: &TerrainConfig,
        player_terrain_data: &CapitalCoord,
    ) -> PlayerInfluenceMap {
        let terrain_size = config.terrain_size;

        if player_terrain_data.data.is_empty() {
            return PlayerInfluenceMap {
                map: vec![None; terrain_size * terrain_size],
                cells: Vec::new(),
                size: terrain_size,
            };
        }

        let sites = player_terrain_data
            .data
            .iter()
            .map(|coord| Point {
                y: coord.0.0 as f64,
                x: coord.0.1 as f64,
            })
            .collect::<Vec<_>>();

        let center = Point {
            x: (terrain_size / 2) as f64,
            y: (terrain_size / 2) as f64,
        };
        let bbox = BoundingBox::new(center, terrain_size as f64, terrain_size as f64);

        let _diagram = VoronoiBuilder::default()
            .set_sites(sites.clone())
            .set_bounding_box(bbox)
            .build()
            .expect("failed to build player voronoi diagram");

        let mut cells = player_terrain_data
            .data
            .iter()
            .map(|coord| PlayerInfluenceCell {
                player_index: coord.1,
                capital: coord.0,
                tile_indices: Vec::new(),
            })
            .collect::<Vec<_>>();

        let mut map = vec![None; terrain_size * terrain_size];

        for row in 0..terrain_size {
            for col in 0..terrain_size {
                let cell_index = nearest_site_index(row, col, &sites);
                let tile_index = row * terrain_size + col;

                map[tile_index] = Some(cell_index);
                cells[cell_index].tile_indices.push(tile_index);
            }
        }

        PlayerInfluenceMap {
            map,
            cells,
            size: terrain_size,
        }
    }
}