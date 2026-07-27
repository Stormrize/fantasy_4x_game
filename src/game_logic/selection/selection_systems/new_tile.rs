use bevy::prelude::{MessageReader, ResMut};
use crate::game_logic::selection::events_resources_structs::components::SelectedEntity;
use crate::game_logic::selection::events_resources_structs::messages::NewTileSelectionEvent;

pub fn new_tile_selection_system(
    mut events: MessageReader<NewTileSelectionEvent>,
    mut selected: ResMut<SelectedEntity>,
) {
    for event in events.read() {
        selected.position = Some(event.tile.pos);
        selected.layer = Some(event.tile.get_top_layer());

        println!(
            "Selected {:?} {:?}",
            selected.position,
            selected.layer
        );
    }
}