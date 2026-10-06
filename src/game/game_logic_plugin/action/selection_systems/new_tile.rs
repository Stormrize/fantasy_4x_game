use crate::game::game_logic_plugin::events_resources_structs::action_messages::NewTile;
use crate::game::game_logic_plugin::events_resources_structs::components::{
    AlreadySelectedTile, JustSelectedTile,
};
use bevy::prelude::{MessageReader, ResMut};

pub fn new_tile_selection_system(
    mut already_selected_tile: ResMut<AlreadySelectedTile>,
    mut new_tile_action: MessageReader<NewTile>,
) {
    for new_tile in new_tile_action.read() {
        already_selected_tile.pos = Some(new_tile.0.pos);
        already_selected_tile.layer = Some(new_tile.0.tile_content.get_top_layer());

        println!("Selected {:?}", already_selected_tile.layer);
    }
}
