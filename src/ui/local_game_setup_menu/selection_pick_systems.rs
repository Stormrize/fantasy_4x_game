use crate::ui::local_game_setup_menu::components_resources::{
    DifficultyButton, GameSetupSelection, RaceButton, StartButton,
};
use bevy::prelude::{
    BackgroundColor, BorderColor, Button, Changed, Color, DetectChanges, Interaction, Query, Res,
    ResMut, Srgba, With,
};

pub(super) fn handle_race_selection(
    interactions: Query<(&Interaction, &RaceButton), (Changed<Interaction>, With<Button>)>,
    mut selection: ResMut<GameSetupSelection>,
) {
    for (interaction, race_button) in &interactions {
        if *interaction == Interaction::Pressed {
            selection.race = Some(race_button.0.clone());
        }
    }
}

pub(super) fn update_race_button_visuals(
    selection: Res<GameSetupSelection>,
    mut buttons: Query<(&RaceButton, &mut BorderColor)>,
) {
    if !selection.is_changed() {
        return;
    }
    for (race_button, mut border_color) in &mut buttons {
        *border_color = if selection.race.as_ref() == Some(&race_button.0) {
            BorderColor::all(Color::WHITE)
        } else {
            BorderColor::all(Color::NONE)
        };
    }
}
pub(super) fn update_start_button_visuals(
    selection: Res<GameSetupSelection>,
    mut start_button: Query<&mut BackgroundColor, With<StartButton>>,
) {
    if !selection.is_changed() {
        return;
    }
    let Ok(mut background) = start_button.single_mut() else {
        return;
    };
    *background = if selection.race.is_some() {
        BackgroundColor(Color::Srgba(Srgba::new(0.2, 0.8, 0.2, 0.6)))
    } else {
        BackgroundColor(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.1)))
    };
}
