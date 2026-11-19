use bevy::prelude::Res;

use crate::game::events::render_selection_messages::RenderMessages;
use crate::game::events::render_selection_messages::{
    RenderBuilding, RenderFog, RenderResource, RenderTerrain, RenderUnit,
};
use crate::game::game_logic_plugin::components::{AlreadySelectedTile, TileLayer};

pub fn manage_selected_entity(
    selected_entity: Res<AlreadySelectedTile>,
    mut render_messages: RenderMessages,
) {
    if let Some(layer) = selected_entity.layer {
        match layer {
            TileLayer::Fog => {
                render_messages.fog.write(RenderFog {
                    pos: selected_entity.pos.unwrap(),
                });
            }
            TileLayer::Unit => {
                render_messages.unit.write(RenderUnit {
                    pos: selected_entity.pos.unwrap(),
                });
            }
            TileLayer::Building => {
                render_messages.building.write(RenderBuilding {
                    pos: selected_entity.pos.unwrap(),
                });
            }
            TileLayer::Resource => {
                render_messages.resource.write(RenderResource {
                    pos: selected_entity.pos.unwrap(),
                });
            }
            TileLayer::Terrain => {
                render_messages.terrain.write(RenderTerrain {
                    pos: selected_entity.pos.unwrap(),
                });
            }
        }
    }
}
