use crate::ui::local_game_setup_menu::spawn_game_setup_menu::GameSetupMenuButtons;
use crate::ui::menus::AppState;
use crate::world::game_config::{MapShape, PlayerRace};
use crate::world::game_configs::{
    AiDifficulty, ConfigType, GameConfig, LocalPlayerConfig, MapSize,
};
use bevy::prelude::{Changed, Commands, Interaction, NextState, Query, ResMut};
use smol_str::SmolStr;

pub(super) fn button_system(
    mut interaction_query: Query<(&Interaction, &GameSetupMenuButtons), (Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                GameSetupMenuButtons::StartGame => {
                    next_state.set(AppState::InGameAssetLoading);
                }
            }
        }
    }
}

//todo доделать наконец
pub(super) fn build_game_config(mut commands: Commands) {
    let seed = rand::random_range(0..u32::MAX);
    let possible_races = vec![
        PlayerRace::new(SmolStr::from("WaterMages")),
        PlayerRace::new(SmolStr::from("FireMages")),
        PlayerRace::new(SmolStr::from("StoneMages")),
    ];

    let game_config = GameConfig::new(
        MapSize::Medium,
        MapShape::Jagged,
        PlayerRace::new(SmolStr::from("WaterMages")),
    );

    let local_game_config = LocalPlayerConfig {
        game_config,
        seed,
        races: possible_races,
        ai_difficulty: AiDifficulty::Hard,
    };

    let config_type = ConfigType::Local(local_game_config);
    // todo убить при переходе
    commands.insert_resource(config_type);
}
