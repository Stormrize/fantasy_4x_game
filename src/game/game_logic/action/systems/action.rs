// use bevy::prelude::{MessageReader, MessageWriter, ResMut};
// use crate::game_logic::commands::{AttackEntityCommand, MoveEntityCommand, PickResourceCommand};
// use crate::game_logic::selection::components::{SelectedEntity, TileLayer};
// use crate::game_logic::selection::events::ActionSelectionEvent;
// 
// pub fn action_system(
//     mut events: MessageReader<ActionSelectionEvent>,
// 
//     mut selected: ResMut<SelectedEntity>,
// 
//     mut move_writer: MessageWriter<MoveEntityCommand>,
//     mut attack_writer: MessageWriter<AttackEntityCommand>,
//     mut pick_writer: MessageWriter<PickResourceCommand>,
// ) {
//     for event in events.read() {
//         //----------------------------------
//         // Если выбран не юнит —
//         // просто снимаем выделение
//         //----------------------------------
// 
//         if event.selected_layer != TileLayer::Unit {
//             selected.clear();
//             continue;
//         }
// 
//         //----------------------------------
//         // В воду ходить нельзя
//         //----------------------------------
// 
//         if event.target_tile.is_water {
//             selected.clear();
//             continue;
//         }
// 
//         //----------------------------------
//         // Атака
//         //----------------------------------
// 
//         if event.target_tile.has_unit {
//             attack_writer.write(AttackEntityCommand {
//                 entity_pos: event.selected_pos,
//                 target_entity_pos: event.target_tile.pos,
//             });
// 
//             continue;
//         }
// 
//         //----------------------------------
//         // Поднять ресурс
//         //----------------------------------
// 
//         if event.target_tile.has_resource {
//             pick_writer.write(PickResourceCommand {
//                 entity_pos: event.selected_pos,
//                 target_resource_pos: event.target_tile.pos,
//             });
// 
//             continue;
//         }
// 
//         //----------------------------------
//         // Иначе движение
//         //----------------------------------
// 
//         move_writer.write(MoveEntityCommand {
//             entity_pos: event.selected_pos,
//             target_pos: event.target_tile.pos,
//         });
//     }
// }