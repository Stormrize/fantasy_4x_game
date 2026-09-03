use crate::game::generation::races_generation::players_or_bots::generate_players_positions::{
    CapitalCoord, PlayersTerrainData,
};
use crate::game::generation::races_generation::players_pos_generator::helpers::{
    compute_land_components, farthest_point_seeds, flood_fill_balance, largest_component_id,
};
use crate::world::game_configs::GenerationConfig;
use crate::world::game_resources::terrain_map::TerrainMap;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

impl PlayersTerrainData {
    pub fn calculate_player_positions_flood_fill(
        config: &GenerationConfig,
        terrain: &TerrainMap,
    ) -> Option<PlayersTerrainData> {
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let players = &config.players;
        let terrain_size = config.terrain_size;

        if players.is_empty() {
            return None;
        }

        let comp = compute_land_components(terrain, terrain_size);
        let main_id = largest_component_id(&comp)?;

        let land: Vec<(usize, usize)> = comp
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c == main_id)
            .map(|(idx, _)| (idx % terrain_size, idx / terrain_size))
            .collect();

        if land.len() < players.len() {
            println!(
                "Error: main landmass too small for {} players.",
                players.len()
            );
            return None;
        }

        let seeds = farthest_point_seeds(&land, players.len(), &mut rng);
        let regions = flood_fill_balance(terrain, terrain_size, &seeds);

        // regions[i].len() — размер территории i-го игрока, можно логировать баланс

        let spawn_points = players
            .iter()
            .zip(seeds.iter())
            .map(|(&(id, ref race), &(x, y))| ((id, race.clone()), CapitalCoord(x, y)))
            .collect();

        Some(PlayersTerrainData { data: spawn_points })
    }
}
