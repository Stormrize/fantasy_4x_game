use crate::game::game_logic_plugin::events_resources_structs::components::AlreadySelectedTile;

pub fn clear_selection_system(already_selected_tile: &mut AlreadySelectedTile) {
    already_selected_tile.layer = None;
    already_selected_tile.pos = None;
    println!("clear");
}
