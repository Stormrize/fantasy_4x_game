use crate::input::events::{handle_input, RegenerateWorldEvent};
use crate::world::terrain_config::TerrainConfig;
use crate::ui::menus::AppState;
use bevy::app::{App, FixedUpdate, Startup, Update};
use bevy::asset::Assets;
use bevy::camera::ClearColor;
use bevy::color::Color;
use bevy::math::IVec2;
use bevy::mesh::Mesh;
use bevy::prelude::{in_state, Fixed, IntoScheduleConfigs, OnEnter, Res};
use bevy::prelude::{Commands, MessageReader, Plugin, ResMut, Resource, StandardMaterial};
use std::collections::HashMap;
use crate::generation::{regenerate_map};
use crate::renderer::buildings_renderer::{despawn_buildings_meshes, BuildingsAssets, SpawnedBuildingsMeshes};
use crate::renderer::chunk_renderer::stream_chunks;
use crate::renderer::resource_renderer::{despawn_resource_meshes, ResourceAssets, SpawnedResourceMeshes};
use crate::renderer::terrain_renderer::{despawn_terrain_meshes, SpawnedTerrainMeshes, TerrainAssets};
use crate::world::buildings_map::BuildingMap;
use crate::world::resource_map::ResourceMap;
use crate::world::terrain_map::{TerrainMap};

pub mod buildings_renderer;
pub mod camera_math;
pub mod chunk_renderer;
pub mod resource_renderer;
pub mod terrain_renderer;
#[derive(Resource, Default)]
pub struct Rendered {
    pub terrain: HashMap<IVec2, SpawnedTerrainMeshes>,
    pub resources: HashMap<IVec2, SpawnedResourceMeshes>,
    pub buildings: HashMap<IVec2, SpawnedBuildingsMeshes>,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RegenerateWorldEvent>()
            .add_systems(OnEnter(AppState::InGameMenu), insert_assets)
            .add_systems(
                Update,
                (handle_input, apply_regenerate, stream_chunks)
                    .chain()
                    .run_if(in_state(AppState::InGameMenu)),
            );
    }
}
fn insert_assets(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    //rendered
    commands.insert_resource(Rendered::default());
    //tiles
    commands.insert_resource(TerrainAssets::build(&mut materials));
    //resources
    commands.insert_resource(ResourceAssets::build(&mut materials));
    //buildings
    commands.insert_resource(BuildingsAssets::build(&mut materials));
    //visual
    commands.insert_resource(ClearColor(Color::BLACK))
}
fn apply_regenerate(
    mut events: MessageReader<RegenerateWorldEvent>,
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<TerrainConfig>,
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
