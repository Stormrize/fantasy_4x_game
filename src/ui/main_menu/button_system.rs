use crate::ui::main_menu::spawn_start_menu::MainMenuButtons;
use crate::ui::menus::AppState;
use bevy::prelude::{AppExit, Changed, MessageWriter, NextState, Query, ResMut, With};
use bevy::ui::Interaction;

pub fn button_system(
    mut interaction_query: Query<(&Interaction, &MainMenuButtons), (Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match button {
                MainMenuButtons::StartButton => {
                    next_state.set(AppState::GameSetupMenu);
                }
                MainMenuButtons::ResumeButton => {
                    next_state.set(AppState::InGameMenu);
                }
                MainMenuButtons::Multiplayer => {
                    next_state.set(AppState::MultiplayerSetupMenu);
                }
                MainMenuButtons::ExitButton => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}