// use crate::game::game_logic_plugin::events_resources_structs::render_messages::RenderMessages;
// use crate::game::game_logic_plugin::events_resources_structs::{
//     components::{AlreadySelectedTileContent, TileLayer},
//     render_messages::{RenderBuilding, RenderFog, RenderResource, RenderTerrain, RenderUnit},
// };
// use bevy::prelude::Res;
// 
// pub fn manage_selected_entity(
//     selected_entity: Res<AlreadySelectedTileContent>,
//     mut render_messages: RenderMessages,
// ) {
//     // полностю переписать это говно так как это работает каждый кадр тоесть сообзение отправляются каждый кадр
//     match selected_entity.layer {
//         // note <идет на рендер, рендерится картинка, юзер нажимает например построить здание
//         // note и это отправляется на сервер или на твой же комп сервер проверяет дает ответ>
//         Some(_) => {
//             let layer = selected_entity.layer.unwrap();
//             match layer {
//                 TileLayer::Fog => {
//                     render_messages.fog.write(RenderFog {
//                         pos: selected_entity.pos.unwrap(),
//                     });
//                 }
//                 TileLayer::Unit => {
//                     render_messages.unit.write(RenderUnit {
//                         pos: selected_entity.pos.unwrap(),
//                     });
//                 }
//                 TileLayer::Building => {
//                     render_messages.building.write(RenderBuilding {
//                         pos: selected_entity.pos.unwrap(),
//                     });
//                 }
//                 TileLayer::Resource => {
//                     render_messages.resource.write(RenderResource {
//                         pos: selected_entity.pos.unwrap(),
//                     });
//                 }
//                 TileLayer::Terrain => {
//                     render_messages.terrain.write(RenderTerrain {
//                         pos: selected_entity.pos.unwrap(),
//                     });
//                 }
//             }
//         }
//         None => {}
//     }
// }
