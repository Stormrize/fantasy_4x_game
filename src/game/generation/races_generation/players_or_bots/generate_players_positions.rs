use crate::world::enums::biome::BiomeType;
use crate::world::game_config::GameConfig;
use rand::{RngExt, SeedableRng};
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha8Rng;
use crate::world::enums::races::FantasyRace;
use crate::world::terrain_map::GlobalTerrainMap;

pub struct PlayerTerrainData {
    pub data: Vec<((u8, FantasyRace), (usize, usize))>,
}

impl PlayerTerrainData {
    pub fn calculate_player_positions(
        config: &GameConfig,
        global_terrain: &GlobalTerrainMap,
    ) -> Option<PlayerTerrainData> {
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let mut players = config.players.clone();
        players.shuffle(&mut rng);
        let terrain_size = config.terrain_size;
        let mut spawn_points = Vec::with_capacity(players.len());
        
        if players.len() == 0 {
            return None
        }

        let cols = (players.len() as f32).sqrt().ceil() as usize;
        let rows = ((players.len() as f32) / (cols as f32)).ceil() as usize;

        let sector_width = terrain_size as f32 / cols as f32;
        let sector_height = terrain_size as f32 / rows as f32;

        let base_max_offset_x = (sector_width * 0.2) as i32;
        let base_max_offset_y = (sector_height * 0.2) as i32;

        let mut player_index = 0;

        'outer: for row in 0..rows {
            for col in 0..cols {
                if player_index == players.len() {
                    break 'outer;
                }

                let ideal_x = (col as f32 * sector_width) + (sector_width / 2.0);
                let ideal_y = (row as f32 * sector_height) + (sector_height / 2.0);

                let mut found_pos = None;

                for multiplier in [1, 2] {
                    let current_max_x = base_max_offset_x * multiplier;
                    let current_max_y = base_max_offset_y * multiplier;

                    // Безопасный clamp координат к границам карты
                    let min_x = (ideal_x.round() as i32 - current_max_x).max(2);
                    let max_x = (ideal_x.round() as i32 + current_max_x).min(terrain_size as i32 - 2);
                    let min_y = (ideal_y.round() as i32 - current_max_y).max(2);
                    let max_y = (ideal_y.round() as i32 + current_max_y).min(terrain_size as i32 - 2);

                    let mut candidates = Vec::new();

                    for x in min_x..=max_x {
                        for y in min_y..=max_y {
                            if multiplier == 2 {
                                let inner_min_x = (ideal_x.round() as i32 - base_max_offset_x).max(2);
                                let inner_max_x = (ideal_x.round() as i32 + base_max_offset_x).min(terrain_size as i32 - 2);
                                let inner_min_y = (ideal_y.round() as i32 - base_max_offset_y).max(2);
                                let inner_max_y = (ideal_y.round() as i32 + base_max_offset_y).min(terrain_size as i32 - 2);

                                if x >= inner_min_x && x <= inner_max_x && y >= inner_min_y && y <= inner_max_y {
                                    continue;
                                }
                            }
                            candidates.push((x as usize, y as usize));
                        }
                    }


                    candidates.shuffle(&mut rng);

                    for (x, y) in candidates {
                        if global_terrain.get(x, y) != BiomeType::Water {
                            found_pos = Some((x, y));
                            break;
                        }
                    }

                    if found_pos.is_some() {
                        break;
                    }
                }

                if let Some((final_x, final_y)) = found_pos {
                    spawn_points.push((players[player_index], (final_x, final_y)));
                    player_index += 1;
                } else {
                    println!("Error: Could not spawn player {} even with 2x radius expansion.", player_index);
                    return None;
                }
            }
        }

        Some(PlayerTerrainData {
            data: spawn_points,
        })
    }
}