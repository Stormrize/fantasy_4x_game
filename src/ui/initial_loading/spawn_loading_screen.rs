use bevy::camera::Camera2d;
use bevy::prelude::{Commands, DespawnOnExit};
use crate::ui::menus::AppState;

pub(super) fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        DespawnOnExit(AppState::InGameAssetLoading)
    ));
}
