//! Сборка конфига локальной игры из текущих выборов игрока.
use bevy::prelude::{Commands, NextState};
use rand::prelude::SliceRandom;
use rand::random_range;

use super::components::PendingGameSetup;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::menus::AppState;
use crate::world::components::{GameMode, Id, PlayerRace, PlayerType};
use crate::world::game_configs::{ConfigType, GameModeConfig, LocalPlayerConfig, PlayerConfig};
use crate::world::turn_manager::TurnManager;

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

    let races = selected_races
        .into_iter()
        .enumerate()
        .map(|(i, race)| (Id(i as u16), race))
        .collect::<Vec<(Id, PlayerRace)>>();

    let length = races.len() as u16;

    let local_config = LocalPlayerConfig {
        seed,
        races,
        ai_difficulty: pending.ai_difficulty,
    };

    let id = Id(0);
    commands.insert_resource(TurnManager::new(
        id,
        PlayerType::Human,
        length,
        GameMode::Offline,
    ));

    let player_config = PlayerConfig::new(pending.map_size, pending.map_shape, id);
    let game_mode = GameModeConfig::Local(local_config);

    let config_type = ConfigType {
        player_config,
        game_mode,
    };

    commands.insert_resource(config_type);

    next_state.set(AppState::InGameAssetLoading);
}
