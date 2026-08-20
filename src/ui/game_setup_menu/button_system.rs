use crate::ui::game_setup_menu::spawn_game_setup_menu::GameSetupMenuButtons;
use crate::ui::menus::AppState;
use crate::world::game_config::{GameConfig, PlayerRace};
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

pub(super) fn build_game_config(mut commands: Commands) {
    let seed = rand::random_range(0..u32::MAX);
    let current_players = vec![
        (0, PlayerRace::new(SmolStr::from("WaterMages"))),
        (1, PlayerRace::new(SmolStr::from("FireMages"))),
        (2, PlayerRace::new(SmolStr::from("StoneMages"))),
    ];
    let game_config = GameConfig::new(
        seed,
        current_players,
        32,
        4,
        1.0,
        0.1,
        100.,
        32.0 * 1.5,
        2,
        2,
        100,
    );
    commands.insert_resource(game_config);
}
