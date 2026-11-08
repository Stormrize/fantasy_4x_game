use crate::game::game_logic_plugin::action::rules::move_rules::is_players_turn;
use crate::game::game_logic_plugin::events_resources_structs::action_messages::{
    Action, EndTurn, WorldView,
};
use crate::game::game_logic_plugin::events_resources_structs::components::{
    AlreadySelectedTile, TileLayer,
};
use crate::world::game_configs::GameLogicConfig;
use crate::world::turn_manager::TurnManager;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::KeyCode::{AltLeft, Digit1, Digit2, KeyB, KeyP, KeyQ, KeyR, KeyU, KeyX, Space};
use bevy::prelude::{ButtonInput, KeyCode, Res};

pub fn route_keyboard_actions(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    already_selected_tile: Res<AlreadySelectedTile>,
    turn_manager: Res<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    mut actions: MessageWriter<Action>,
    mut end_turn: MessageWriter<EndTurn>,
    mut world_view: MessageWriter<WorldView>,
) {
    // skills
    let q_pressed = keyboard_input.just_pressed(KeyQ);
    let one_pressed = keyboard_input.just_pressed(Digit1);
    let two_pressed = keyboard_input.just_pressed(Digit2);
    let r_pressed = keyboard_input.just_pressed(KeyR);
    // delete & upgrade
    let x_pressed = keyboard_input.just_pressed(KeyX);
    let u_pressed = keyboard_input.just_pressed(KeyU);
    // info
    let alt_pressed = keyboard_input.just_pressed(AltLeft);
    // build
    let b_pressed = keyboard_input.just_pressed(KeyB);
    // pick
    let p_pressed = keyboard_input.just_pressed(KeyP);
    // end turn
    let space_pressed = keyboard_input.just_pressed(Space);
    // open world view
    let tab_pressed = keyboard_input.just_pressed(KeyCode::Tab);

    if space_pressed {
        // Завершать ход можно только в свой ход, иначе человек прервал бы ход бота.
        if is_players_turn(game_logic_config.player, turn_manager.current_player) {
            end_turn.write(EndTurn);
            println!("send end turn");
        }
        return;
    } else if tab_pressed {
        world_view.write(WorldView);
    }

    match already_selected_tile.layer {
        Some(TileLayer::Unit) => {
            // todo: цель выбирается в боевом UI; пока target = entity.
            if q_pressed {
                actions.write(Action::Attack {
                    unit_pos: already_selected_tile.pos.unwrap(),
                    target_pos: already_selected_tile.pos.unwrap(),
                    skill_type: 0,
                });
                println!("send attack q");
            } else if one_pressed {
                actions.write(Action::Attack {
                    unit_pos: already_selected_tile.pos.unwrap(),
                    target_pos: already_selected_tile.pos.unwrap(),
                    skill_type: 1,
                });
                println!("send end turn 1");
            } else if two_pressed {
                actions.write(Action::Attack {
                    unit_pos: already_selected_tile.pos.unwrap(),
                    target_pos: already_selected_tile.pos.unwrap(),
                    skill_type: 2,
                });
                println!("send attack 2");
            } else if r_pressed {
                actions.write(Action::Attack {
                    unit_pos: already_selected_tile.pos.unwrap(),
                    target_pos: already_selected_tile.pos.unwrap(),
                    skill_type: 3,
                });
                println!("send attack 3");
            } else if u_pressed {
                actions.write(Action::Upgrade {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    layer: TileLayer::Unit,
                });
                println!("send upgrade unit");
            } else if x_pressed {
                actions.write(Action::Delete {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    layer: TileLayer::Unit,
                });
                println!("send delete unit");
            } else if alt_pressed {
                actions.write(Action::Info {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    layer: TileLayer::Unit,
                });
                println!("send info unit");
            } else if b_pressed {
                actions.write(Action::Build {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    building_type: None,
                });
                println!("send build");
            } else if p_pressed {
                actions.write(Action::Gather {
                    resource_pos: already_selected_tile.pos.unwrap(),
                });
                println!("send gather");
            }
        }
        Some(TileLayer::Building) => {
            if u_pressed {
                actions.write(Action::Upgrade {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    layer: TileLayer::Building,
                });
                println!("send upgrade building");
            } else if x_pressed {
                actions.write(Action::Delete {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    layer: TileLayer::Building,
                });
                println!("send delete building");
            } else if alt_pressed {
                actions.write(Action::Info {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    layer: TileLayer::Building,
                });
                println!("send info building");
            }
        }
        Some(TileLayer::Resource) => {
            if alt_pressed {
                actions.write(Action::Info {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    layer: TileLayer::Resource,
                });
                println!("send info resource");
            }
        }
        Some(TileLayer::Terrain) => {
            if alt_pressed {
                actions.write(Action::Info {
                    entity_pos: already_selected_tile.pos.unwrap(),
                    layer: TileLayer::Terrain,
                });
                println!("send info terrain");
            }
        }
        _ => {}
    }
}
