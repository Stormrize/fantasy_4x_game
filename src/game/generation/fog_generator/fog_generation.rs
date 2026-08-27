use crate::game::generation::races_generation::players_or_bots::generate_players_positions::PlayersTerrainData;

use crate::world::{game_configs::GenerationConfig, game_resources::fog_map::FogMap};

impl FogMap {
    pub fn new(config: &GenerationConfig, capital_coord: &PlayersTerrainData) -> Self {
        let terrain_size = config.terrain_size;
        // let player = &config.player;
        // let mut player_capital_coord = None;
        // let radius = 2;
        let mut data = vec![true; terrain_size * terrain_size];

        // for players in &capital_coord.data {
        //     if players.0.1 == *player {
        //         player_capital_coord = Some(players.1);
        //         break;
        //     }
        // }
        //
        // if player_capital_coord.is_none() {
        //     panic!("No capital coord for player found");
        // }
        //
        // let center_row = player_capital_coord.unwrap().0;
        // let center_col = player_capital_coord.unwrap().1;
        //
        // for row in center_row - radius..=center_row + radius {
        //     for col in center_col - radius..=center_col + radius {
        //         let index = row * terrain_size + col;
        //         data[index] = false;
        //     }
        // }

        Self {
            map: data,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> bool {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        self.map[row * self.size + col]
    }
}
