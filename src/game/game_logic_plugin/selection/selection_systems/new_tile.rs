use crate::game::game_logic_plugin::events_resources_structs::components::{
    AlreadySelectedTile, JustSelectedTile, TileContent,
};

pub fn new_tile_selection_system(
    just_selected_tile: JustSelectedTile,
    already_selected_tile: &mut AlreadySelectedTile,
) {
    already_selected_tile.pos = Some(just_selected_tile.pos);
    already_selected_tile.layer = Some(just_selected_tile.tile_content.get_top_layer());

    println!(
        "Selected {:?} {:?}",
        already_selected_tile.pos, already_selected_tile.layer
    );
}
