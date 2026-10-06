use crate::game::game_logic_plugin::events_resources_structs::action_messages::Cycle;
use crate::game::game_logic_plugin::events_resources_structs::components::{
    AlreadySelectedTile, TileLayer,
};
use bevy::prelude::{MessageReader, ResMut};

pub fn cycle_selection_system(
    mut already_selected_tile: ResMut<AlreadySelectedTile>,
    mut cycle_action: MessageReader<Cycle>,
) {
    for cycle in cycle_action.read() {
        let next = match already_selected_tile.layer {
            Some(TileLayer::Unit) if cycle.0.tile_content.building => Some(TileLayer::Building),

            Some(TileLayer::Unit) if cycle.0.tile_content.resource => Some(TileLayer::Resource),

            Some(TileLayer::Unit) => Some(TileLayer::Terrain),

            Some(TileLayer::Building) if cycle.0.tile_content.resource => Some(TileLayer::Resource),

            Some(TileLayer::Building) => Some(TileLayer::Terrain),

            Some(TileLayer::Resource) => Some(TileLayer::Terrain),

            Some(TileLayer::Terrain) => None,

            _ => None,
        };
        already_selected_tile.layer = next;

        if already_selected_tile.layer.is_none() {
            already_selected_tile.pos = None;
        }
        println!("Cycled to:c{:?}", already_selected_tile.layer);
    }
}
