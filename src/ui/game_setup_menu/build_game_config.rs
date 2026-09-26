//! Сборка конфига локальной игры из текущих выборов игрока.
use bevy::prelude::{Commands, NextState};
use rand::prelude::SliceRandom;
use rand::random_range;

use super::components::PendingGameSetup;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::menus::AppState;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::{ConfigType, GameMode, LocalPlayerConfig, PlayerConfig};

pub fn start_local_game(
    commands: &mut Commands,
    chosen_race: PlayerRace,
    races_ui_registry: &RacesUiRegistry,
    pending: &PendingGameSetup,
    next_state: &mut NextState<AppState>,
) {
    let seed = random_range(0..u32::MAX);

    let mut possible_races: Vec<PlayerRace> = races_ui_registry.0.keys().cloned().collect();

    if !possible_races.contains(&chosen_race) {
        panic!("race was not found in races ui registry! Look Up initial loading");
    }

    possible_races.retain(|race| *race != chosen_race);

    let mut rng = rand::rng();
    possible_races.shuffle(&mut rng);

    // todo 3 is hardcoded, gotta be fixed, by adding a ui(maybe scrollable) element <bots>
    let bots_amount = 3;

    let mut selected_races = Vec::with_capacity(bots_amount);
    selected_races.push(chosen_race.clone());
    selected_races.extend(possible_races.into_iter().take(bots_amount - 1));

    selected_races.shuffle(&mut rng);

    let id = selected_races
        .iter()
        .position(|race| *race == chosen_race)
        .expect("chosen race was not selected");

    let races = selected_races
        .into_iter()
        .enumerate()
        .map(|(i, race)| (Id::LocalPlayerId(i as u8), race))
        .collect();

    let local_config = LocalPlayerConfig {
        seed,
        races,
        ai_difficulty: pending.ai_difficulty,
    };

    let player_config = PlayerConfig::new(
        pending.map_size,
        pending.map_shape,
        Id::LocalPlayerId(id as u8),
    );
    let game_mode = GameMode::Local(local_config);

    let config_type = ConfigType {
        player_config,
        game_mode,
    };

    commands.insert_resource(config_type);

    next_state.set(AppState::InGameAssetLoading);
}
