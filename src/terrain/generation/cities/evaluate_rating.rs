use crate::terrain::generation::cities::rate_tiles::{RADIUS, rate_the_tiles};
use crate::terrain::generation::resources::resource_generation::ResourceMap;
use crate::terrain::generation::terrain::terrain_generation::TerrainMap;
use crate::terrain::terrain_config::TerrainConfig;

//too simple right now, could be more checks
pub fn evaluate_rating(terrain: &TerrainMap, resources: &ResourceMap, config: &TerrainConfig) -> Vec<(u16, u16)> {
    let terrain_size = config.terrain_size;
    let (tile_rating, max_score, min_score) = rate_the_tiles(terrain, resources, config);
    let threshold_score = max_score / 2 + min_score;
    let mut cities = vec![];
    let mut blocked = vec![false; terrain_size * terrain_size];
    for y in 0..terrain_size {
        for x in 0..terrain_size {
            let index = y * terrain_size + x;
            if tile_rating[index] < threshold_score {
                continue;
            }

            if blocked[index] {
                continue;
            }
            cities.push((y as u16, x as u16));
            let min_y = y.saturating_sub(RADIUS + 2);
            let max_y = (y + RADIUS + 2).min(terrain_size - 1);
            let min_x = x.saturating_sub(RADIUS + 2);
            let max_x = (x + RADIUS + 2).min(terrain_size - 1);
            for ny in min_y..=max_y {
                for nx in min_x..=max_x {
                    let index = ny * terrain_size + nx;
                    blocked[index] = true;
                }
            }
        }
    }
    println!("evaluate rating len: {}", cities.len());
    cities
}
