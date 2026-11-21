use bevy::input::ButtonInput;
use bevy::picking::hover::HoverMap;
use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use bevy::prelude::{MouseButton, Res};

use crate::game::camera_plugin::camera::MainCamera;
use crate::game::events::action_messages::TileClickedEvent;

pub fn click_to_world(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    hover_map: Res<HoverMap>,
    ui_nodes: Query<(), With<Node>>,
    mut tile_clicked_event: MessageWriter<TileClickedEvent>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Clicks on HUD panels must not select or move anything on the map.
    // The world ray is computed from raw cursor input, so UI picking alone
    // cannot swallow it; the hover map tells us whether the cursor is over a
    // pickable UI node instead.
    let pointer_over_ui = hover_map
        .get(&PointerId::Mouse)
        .is_some_and(|hits| hits.keys().any(|entity| ui_nodes.contains(*entity)));
    if pointer_over_ui {
        return;
    }
    let Some(window) = windows.iter().next() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
        && ray.direction.y.abs() > f32::EPSILON
    {
        let t = -ray.origin.y / ray.direction.y;

        if t >= 0.0 {
            let click_world_pos = ray.origin + ray.direction * t;
            let grid_pos = IVec2::new(
                click_world_pos.x.round() as i32,
                click_world_pos.z.round() as i32,
            );
            tile_clicked_event.write(TileClickedEvent { pos: grid_pos });
        }
    }
}
