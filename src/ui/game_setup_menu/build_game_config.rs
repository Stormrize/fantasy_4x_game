//! Сборка конфига локальной игры из текущих выборов игрока.

use super::components::PendingGameSetup;
use crate::ui::menus::AppState;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::{ConfigType, GameConfig, LocalPlayerConfig};
use crate::world::ui_and_graphic_resources::player_registry::RacesUiRegistry;
use bevy::prelude::{Commands, NextState};
use rand::prelude::SliceRandom;
use rand::random_range;

pub fn start_local_game(
    commands: &mut Commands,
    race: PlayerRace,
    races_ui_registry: &RacesUiRegistry,
    pending: &PendingGameSetup,
    next_state: &mut NextState<AppState>,
) {
    let game_config = GameConfig::new(pending.map_size, pending.map_shape.clone(), race);

    let seed = random_range(0..u32::MAX);

    let mut possible_races: Vec<PlayerRace> = races_ui_registry.0.keys().cloned().collect();

    let mut rng = rand::rng();

    possible_races.shuffle(&mut rng);

    possible_races.truncate(3);

    let races = possible_races
        .into_iter()
        .enumerate()
        .map(|(id, race)| (Id::LocalPlayerId(id as u8), race))
        .collect::<Vec<_>>();

    let local_config = LocalPlayerConfig {
        game_config,
        seed,
        races,
        ai_difficulty: pending.ai_difficulty,
    };

    println!("Config: {:?}", local_config);

    commands.insert_resource(ConfigType::Local(local_config));

    next_state.set(AppState::InGameAssetLoading);
}
