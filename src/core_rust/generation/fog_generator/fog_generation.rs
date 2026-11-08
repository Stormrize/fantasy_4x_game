use crate::core_rust::generation::races_generation::capitals_generation::CapitalsPos;
use crate::world::components::{BUILDING_SEE_RADIUS, Id};
use crate::world::game_resources::fog_map::Fog;
use crate::world::{game_configs::GenerationConfig, game_resources::fog_map::FogMap};

impl FogMap {
    pub fn new(config: &GenerationConfig, capital_coord: &CapitalsPos) -> Self {
        let terrain_size = config.terrain_size;

        let all_bots: Vec<(Id, bool)> = config
            .players
            .iter()
            .skip(1)
            .map(|(id, _)| (*id, false))
            .collect::<Vec<_>>();

        let mut data = vec![
            Fog::new(
                false, // debug for now
                all_bots.clone(),
                false, //debug for now
                all_bots
            );
            terrain_size * terrain_size
        ];

        let radius = BUILDING_SEE_RADIUS as isize;

        // todo
        for ((id, _), coord) in capital_coord.data.iter() {
            let is_player = *id == config.players[0].0;

            let cx = coord.0 as isize;
            let cy = coord.1 as isize;
            println!("capital_coord: {}, {}", coord.0, coord.1);

            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    let map_x = cx + dx;
                    let map_y = cy + dy;

                    if map_x >= 0
                        && map_x < terrain_size as isize
                        && map_y >= 0
                        && map_y < terrain_size as isize
                    {
                        let index = (map_x as usize) * terrain_size + (map_y as usize);

                        if is_player {
                            data[index].general_visible_player = true;
                            data[index].full_visible_player = true;
                        } else {
                            data[index].general_visible_bots[(id.0 - 1) as usize] = (*id, true);
                            data[index].full_visible_bots[(id.0 - 1) as usize] = (*id, true);
                        }
                    }
                }
            }
        }

        Self {
            map: data,
            size: terrain_size,
            // debug for now: весь туман раскрыт локальному игроку
            debug_full_visibility: false,
        }
    }
}
