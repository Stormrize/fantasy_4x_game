use crate::game::game_logic_plugin::events_resources_structs::action_messages::Clear;
use crate::game::game_logic_plugin::events_resources_structs::components::AlreadySelectedTile;
use bevy::prelude::{MessageReader, ResMut};

pub fn clear_selection_system(
    mut already_selected_tile: ResMut<AlreadySelectedTile>,
    mut clear: MessageReader<Clear>,
) {
    for _ in clear.read() {
        already_selected_tile.layer = None;
        already_selected_tile.pos = None;
        println!("clear");
    }
}
