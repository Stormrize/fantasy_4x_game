use bevy::prelude::{MessageReader, ResMut};
use crate::game::game_logic_plugin::selection::events_resources_structs::components::SelectedEntity;
use crate::game::game_logic_plugin::selection::events_resources_structs::messages::ClearSelectionEvent;

pub fn clear_selection_system(
    mut events: MessageReader<ClearSelectionEvent>,
    mut selected: ResMut<SelectedEntity>,
) {
    for _ in events.read() {
        selected.clear();
    }
}
