use bevy::input::ButtonInput;
use bevy::prelude::{MouseButton, Res};
use bevy::prelude::*;
use crate::game_logic::selection::events_resources_structs::messages::TileClickedEvent;

// TODO для телефонов + сделать правая мышь детали
pub fn click_to_world (
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut tile_clicked_event: MessageWriter<TileClickedEvent>
)  {
    if !mouse_input.just_pressed(MouseButton::Left) { return; }

    let Some(window) = windows.iter().next() else { return; };
    let Some(cursor_position) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_query.single() else { return; };

    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {

        if ray.direction.y.abs() > f32::EPSILON {
            let t = -ray.origin.y / ray.direction.y;

            if t >= 0.0 {
                let click_world_pos = ray.origin + ray.direction * t;
                let grid_pos = IVec2::new(click_world_pos.x.round() as i32, click_world_pos.z.round() as i32);
                tile_clicked_event.write(TileClickedEvent { grid_pos });
            }
        }
    }
}