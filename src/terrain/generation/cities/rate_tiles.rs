use crate::terrain::enums::biome::BiomeType;
use crate::terrain::generation::resources::resource_generation::ResourceMap;
use crate::terrain::generation::terrain::terrain_generation::TerrainMap;
use crate::terrain::terrain_config::TerrainConfig;

pub const RADIUS: usize = 1;


pub fn rate_the_tiles(
    terrain: &TerrainMap,
    resources: &ResourceMap,
    config: &TerrainConfig,
) -> (Vec<u16>, u16, u16) {
    let terrain_size = config.terrain_size;
    let mut data = vec![0u16; terrain_size * terrain_size];
    let mut maximal_score = 0;
    let mut minimal_score = u16::MAX;

    for y in 0..terrain_size {
        for x in 0..terrain_size {
            let biome= terrain.get(y, x);
            if biome != BiomeType::Grass {
                continue;
            }

            let min_y = y.saturating_sub(RADIUS);
            let max_y = (y + RADIUS).min(terrain_size - 1);

            let min_x = x.saturating_sub(RADIUS);
            let max_x = (x + RADIUS).min(terrain_size - 1);

            let mut score = 0u16;
            let mut seen = [0_u8; 256];

            for ny in min_y..=max_y {
                for nx in min_x..=max_x {
                    if ny == y && nx == x {
                        continue;
                    }
                    let resource = resources.get(ny, nx);

                    let idx = resource as usize;
                    let value = resource.score_rate() as u16;

                    if seen[idx] != 0 {
                        let add = value / seen[idx] as u16;
                        score += add;

                    } else {
                        seen[idx] = seen[idx] + 2;
                        score += value;
                    }
                }
            }
            if score > maximal_score {
                maximal_score = score;
            }
            if score < minimal_score && minimal_score != 0 {
                minimal_score = score;
            }
            let index = y * terrain_size + x;
            data[index] = score;
        }
    }
    println!("rate tiles len: {}", data.len());
    (data, maximal_score, minimal_score)
}