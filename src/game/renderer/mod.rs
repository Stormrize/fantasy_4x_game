use crate::ui::menus::AppState;
use crate::world::{
    game_config::GameConfig,
    game_resources::{
        buildings_map::BuildingMap, resource_map::ResourceMap, terrain_map::TerrainMap,
    },
};
use bevy::platform::collections::HashMap;
use bevy::prelude::{App, Assets, Commands, IVec2, IntoScheduleConfigs, Mesh, MessageReader, OnEnter, Plugin, Res, ResMut, Resource, Update, in_state, ClearColor, Color, StandardMaterial};

use crate::game::{
    generation::regenerate_map,
    renderer::{
        chunk_renderer::stream_chunks,
        buildings_renderer::{SpawnedBuildingsMeshes, despawn_buildings_meshes},
        resource_renderer::{SpawnedResourceMeshes, despawn_resource_meshes},
        terrain_renderer::{SpawnedTerrainMeshes, despawn_terrain_meshes},
        units_renderer::{SpawnedUnitsMeshes, despawn_units_meshes},
    },
};
use crate::game::generation::insert_map_resources;
use crate::game::input::events::{handle_input, RegenerateWorldEvent};

pub mod buildings_renderer;
pub mod camera_math;
pub mod chunk_renderer;
pub mod resource_renderer;
pub mod terrain_renderer;
mod units_renderer;

#[derive(Resource, Default)]
pub struct Rendered {
    pub terrain: HashMap<IVec2, SpawnedTerrainMeshes>,
    pub resources: HashMap<IVec2, SpawnedResourceMeshes>,
    pub buildings: HashMap<IVec2, SpawnedBuildingsMeshes>,
    //pub units: HashMap<IVec2, SpawnedUnitsMeshes>,
}

pub struct MapRenderPlugin;

impl Plugin for MapRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RegenerateWorldEvent>()
            .add_systems(OnEnter(AppState::InGameRenderer), insert_assets)
            .add_systems(
                Update,
                (handle_input, apply_regenerate, stream_chunks)
                    .chain()
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}
fn insert_assets(mut commands: Commands) {
    //rendered
    commands.insert_resource(Rendered::default());
    //map_generation
    insert_map_resources(&mut commands);
    //visual
    commands.insert_resource(ClearColor(Color::BLACK))
}
fn apply_regenerate(
    mut events: MessageReader<RegenerateWorldEvent>,
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<GameConfig>,
    terrains: ResMut<TerrainMap>,
    resources: ResMut<ResourceMap>,
    buildings: ResMut<BuildingMap>,
) {
    if events.read().count() == 0 {
        return;
    }

    regenerate_map(config, terrains, resources, buildings);

    for (_, terrain_meshes) in &mut rendered.terrain.drain() {
        despawn_terrain_meshes(&mut commands, &mut meshes, terrain_meshes);
    }
    for (_, resource_meshes) in &mut rendered.resources.drain() {
        despawn_resource_meshes(&mut commands, &mut meshes, resource_meshes);
    }
    for (_, buildings) in &mut rendered.buildings.drain() {
        despawn_buildings_meshes(&mut commands, &mut meshes, buildings);
    }
}
