use crate::game::game_logic_plugin::events_resources_structs::components::{
    AlreadySelectedTile, JustSelectedTile, TileLayer,
};

pub fn cycle_selection_system(
    just_selected_tile: JustSelectedTile,
    already_selected_tile: &mut AlreadySelectedTile,
) {
    let next = match already_selected_tile.layer {
        Some(TileLayer::Unit) if just_selected_tile.tile_content.building => {
            Some(TileLayer::Building)
        }

        Some(TileLayer::Unit) if just_selected_tile.tile_content.resource => {
            Some(TileLayer::Resource)
        }

        Some(TileLayer::Unit) => Some(TileLayer::Terrain),

        Some(TileLayer::Building) if just_selected_tile.tile_content.resource => {
            Some(TileLayer::Resource)
        }

        Some(TileLayer::Building) => Some(TileLayer::Terrain),

        Some(TileLayer::Resource) => Some(TileLayer::Terrain),

        Some(TileLayer::Terrain) => None,

        _ => None,
    };
    already_selected_tile.layer = next;

    if already_selected_tile.layer.is_none() {
        already_selected_tile.pos = None;
    }
    println!(
        "Cycled to:  {:?} {:?}",
        already_selected_tile.pos, already_selected_tile.layer
    );
}
