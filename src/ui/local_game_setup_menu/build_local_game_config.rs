use crate::ui::initial_loading::localization_loader_plugin::public_api::{
    CurrentLanguage, Language,
};
use crate::ui::local_game_setup_menu::components_resources::{
    ChangeLanguageTry, GameSetupSelection, StartButton,
};
use crate::ui::menus::AppState;
use crate::world::game_config::PlayerRace;
use crate::world::game_configs::{ConfigType, GameConfig, LocalPlayerConfig};
use crate::world::ui_and_graphic_resources::player_registry::{Id, RacesUiRegistry};
use bevy::prelude::{Changed, Commands, Interaction, NextState, Query, Res, ResMut, With, warn};
use rand::prelude::SliceRandom;

fn build_local_game_config(
    race: PlayerRace,
    selection: &GameSetupSelection,
    races_ui_registry: &RacesUiRegistry,
) -> ConfigType {
    let seed = rand::random_range(0..u32::MAX);

    let mut possible_races: Vec<PlayerRace> = races_ui_registry.0.keys().cloned().collect();
    let mut rng = rand::rng();
    possible_races.shuffle(&mut rng);
    possible_races.truncate(selection.bot_count.min(possible_races.len()));

    let players: Vec<(Id, PlayerRace)> = possible_races
        .into_iter()
        .enumerate()
        .map(|(index, player_race)| (Id::LocalPlayerId(index as u8), player_race))
        .collect();

    let game_config = GameConfig::new(selection.map_size, selection.map_shape, race);

    let local_game_config = LocalPlayerConfig {
        game_config,
        seed,
        races: players,
        ai_difficulty: selection.ai_difficulty,
    };

    ConfigType::Local(local_game_config)
}

pub(super) fn handle_start_button(
    mut commands: Commands,
    interactions: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    selection: Res<GameSetupSelection>,
    races_ui_registry: Res<RacesUiRegistry>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok(interaction) = interactions.single() else {
        return;
    };
    if *interaction != Interaction::Pressed {
        return;
    }

    let Some(race) = selection.race.clone() else {
        warn!("Нельзя начать игру: не выбрана раса");
        return;
    };

    let config_type = build_local_game_config(race, &selection, &races_ui_registry);
    commands.insert_resource(config_type);
    next_state.set(AppState::InGameAssetLoading);
}

pub(super) fn handle_change_language_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<ChangeLanguageTry>)>,
    mut current_language: ResMut<CurrentLanguage>,
) {
    let Ok(interaction) = interactions.single() else {
        return;
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    current_language.0 = Language::De;
}
