use crate::game::game_logic_plugin::events_resources_structs::components::{
    AlreadySelectedTile, TileLayer,
};
use bevy::prelude::KeyCode::{KeyB, KeyC, KeyE, KeyP, KeyQ, KeyR, KeyW, KeyX, Space};
use bevy::prelude::{ButtonInput, KeyCode, Res};

fn route_keyboard_actions(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    already_selected_tile: Res<AlreadySelectedTile>,
) {
    let q_pressed = keyboard_input.just_pressed(KeyQ);
    let w_pressed = keyboard_input.just_pressed(KeyW);
    let e_pressed = keyboard_input.just_pressed(KeyE);
    let r_pressed = keyboard_input.just_pressed(KeyR);
    let x_pressed = keyboard_input.just_pressed(KeyX);
    let c_pressed = keyboard_input.just_pressed(KeyC);
    let b_pressed = keyboard_input.just_pressed(KeyB);
    let p_pressed = keyboard_input.just_pressed(KeyP);
    let space_pressed = keyboard_input.just_pressed(Space);

    match already_selected_tile.layer {
        Some(TileLayer::Unit) => {
            if q_pressed {
                // default attack
            } else if w_pressed {
                // first skill
            } else if e_pressed {
                // second skill
            } else if r_pressed {
                // ultimate check for special rules
            } else if space_pressed {
                // upgrade unit
            } else if x_pressed {
                // remove unit
            } else if c_pressed {
                // info
            } else if b_pressed {
                // build stuff
            } else if p_pressed {
                // pick resource
            } else {
                return ();
            }
        }
        Some(TileLayer::Building) => {
            if space_pressed {
                // upgrade building
            } else if x_pressed {
                // remove building
            } else if c_pressed {
                // info
            }
        }
        Some(TileLayer::Resource) => {
            if c_pressed {
                // info
            }
        }
        Some(TileLayer::Terrain) => {
            if c_pressed {
                // info
            }
        }
        _ => {
            return;
        }
    }
}
