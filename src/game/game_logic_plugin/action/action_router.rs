use crate::game::game_logic_plugin::action::game_rules::{
    check_attack_validity, check_move_validity,
};
use crate::game::game_logic_plugin::events_resources_structs::action_messages::{
    ActionCommandWriters, AttackEntityCommand, MoveEntityCommand,
};
use crate::game::game_logic_plugin::events_resources_structs::components::{AlreadySelectedTile, JustSelectedTile, TileLayer};
use crate::game::game_logic_plugin::selection::selection_systems::clear::clear_selection_system;
use crate::world::game_config::GameConfig;

pub fn route_action(
    config: &GameConfig,
    just_selected_tile: JustSelectedTile,
    already_selected_tile: &mut AlreadySelectedTile,
    commands: &mut ActionCommandWriters,
) {

    let player_or_none = already_selected_tile.tile_content.unit.inside();
    
    let Some(player) = player_or_none else {
        clear_selection_system(already_selected_tile);
        return;  
    };
    
    if *player != config.player {
        clear_selection_system(already_selected_tile);
        return;
    }

    match just_selected_tile.tile_content.get_top_layer() {
        TileLayer::Unit => {
            if !check_attack_validity() {
                //todo можно лучше если сервер
                clear_selection_system(already_selected_tile);
                return;
            }
            commands.attack_cmd.write(AttackEntityCommand {
                entity_pos: already_selected_tile.pos.unwrap(),
                target_pos: just_selected_tile.pos,
            });
        }
        TileLayer::Terrain => {
            if !check_move_validity() {
                //todo уменьшать скорость ходьбы при определенных ресурсах
                clear_selection_system(already_selected_tile);
                return;
            }
            commands.move_cmd.write(MoveEntityCommand {
                entity_pos: already_selected_tile.pos.unwrap(),
                target_pos: just_selected_tile.pos,
            });
        }

        TileLayer::Resource => {
            if !check_move_validity() {
                clear_selection_system(already_selected_tile);
                return;
            }
            //todo уменьшать скорость ходьбы при определенных ресурсах
            commands.move_cmd.write(MoveEntityCommand {
                entity_pos: already_selected_tile.pos.unwrap(),
                target_pos: just_selected_tile.pos,
            });
        }
        TileLayer::Building => {
            if !check_move_validity() {
                clear_selection_system(already_selected_tile);
                return;
            }
            //todo захват на следующем ходе
            commands.move_cmd.write(MoveEntityCommand {
                entity_pos: already_selected_tile.pos.unwrap(),
                target_pos: just_selected_tile.pos,
            });
        }
        TileLayer::Fog => {
            clear_selection_system(already_selected_tile);
            return;
        }
    }
}
