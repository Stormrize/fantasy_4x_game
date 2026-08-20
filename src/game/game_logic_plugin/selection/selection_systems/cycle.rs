use bevy::prelude::{MessageReader, ResMut};
use crate::game::game_logic_plugin::selection::events_resources_structs::components::{SelectedEntity, TileLayer};
use crate::game::game_logic_plugin::selection::events_resources_structs::messages::CycleSelectionEvent;

pub fn cycle_selection_system(
    mut events: MessageReader<CycleSelectionEvent>,
    mut selected: ResMut<SelectedEntity>,
) {
    for event in events.read() {
        let next = match selected.layer {
            Some(TileLayer::Unit) if event.tile.has_building => Some(TileLayer::Building),

            Some(TileLayer::Unit) if event.tile.has_resource => Some(TileLayer::Resource),

            Some(TileLayer::Unit) => Some(TileLayer::Terrain),

            Some(TileLayer::Building) if event.tile.has_resource => Some(TileLayer::Resource),

            Some(TileLayer::Building) => Some(TileLayer::Terrain),

            Some(TileLayer::Resource) => Some(TileLayer::Terrain),

            Some(TileLayer::Terrain) => None,

            _ => None,
        };
        selected.layer = next;
        println!("Cycled to:  {:?} {:?}", selected.position, selected.layer);
    }
}
