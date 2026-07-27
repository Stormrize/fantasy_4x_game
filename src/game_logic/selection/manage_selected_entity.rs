use bevy::prelude::{MessageWriter, Res};
use crate::game_logic::selection::events_resources_structs::components::{SelectedEntity, TileLayer::*};
use crate::game_logic::selection::events_resources_structs::messages::{RenderBuilding, RenderFog, RenderResource, RenderTerrain, RenderUnit};

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
                Fog => {
                    render_fog.write(RenderFog {
                        pos: selected_entity.position.unwrap()
                    });
                },
                Unit => {
                    render_unit.write(RenderUnit {
                        pos: selected_entity.position.unwrap()
                    });
                },
                Building => {
                    render_building.write(RenderBuilding {
                        pos: selected_entity.position.unwrap()
                    });
                },
                Resource => {
                    render_resource.write(RenderResource {
                        pos: selected_entity.position.unwrap()
                    });
                },
                Terrain => {
                    render_terrain.write(RenderTerrain {
                        pos: selected_entity.position.unwrap()
                    });
                },
            }
        }
        None => {}
    }
}