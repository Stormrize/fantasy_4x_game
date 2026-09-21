use bevy::camera::Camera2d;
use bevy::prelude::{Camera, ClearColorConfig, Commands, default};

pub(super) fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}
