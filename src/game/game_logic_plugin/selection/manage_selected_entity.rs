use bevy::prelude::{MessageWriter, Res};
use crate::game::game_logic_plugin::selection::events_resources_structs::{
    components::{SelectedEntity, TileLayer},
    messages::{RenderBuilding, RenderFog, RenderResource, RenderUnit, RenderTerrain}
};

pub fn manage_selected_entity(
    selected_entity: Res<SelectedEntity>,
    mut render_terrain: MessageWriter<RenderTerrain>,
    mut render_resource: MessageWriter<RenderResource>,
    mut render_building: MessageWriter<RenderBuilding>,
    mut render_unit: MessageWriter<RenderUnit>,
    mut render_fog: MessageWriter<RenderFog>,
) {
    match selected_entity.layer {
        //TODO идет на рендер так как не нужно проверять на сервере
        Some(_) => {
            let layer = selected_entity.layer.unwrap();
            match layer {
                TileLayer::Fog => {
                    render_fog.write(RenderFog {
                        pos: selected_entity.position.unwrap(),
                    });
                }
                TileLayer::Unit => {
                    render_unit.write(RenderUnit {
                        pos: selected_entity.position.unwrap(),
                    });
                }
                TileLayer::Building => {
                    render_building.write(RenderBuilding {
                        pos: selected_entity.position.unwrap(),
                    });
                }
                TileLayer::Resource => {
                    render_resource.write(RenderResource {
                        pos: selected_entity.position.unwrap(),
                    });
                }
                TileLayer::Terrain => {
                    render_terrain.write(RenderTerrain {
                        pos: selected_entity.position.unwrap(),
                    });
                }
            }
        }
        None => {}
    }
}
