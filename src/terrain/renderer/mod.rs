use bevy::prelude::{in_state, IntoScheduleConfigs, OnEnter, Res};
use std::collections::HashMap;
use bevy::app::{App, Startup, Update};
use bevy::asset::Assets;
use bevy::camera::ClearColor;
use bevy::color::Color;
use bevy::math::IVec2;
use bevy::mesh::Mesh;
use bevy::prelude::{Commands, MessageReader, Plugin, ResMut, Resource, StandardMaterial};
use crate::input::events::{handle_input, RegenerateWorldEvent};
use crate::terrain::generation::cities::cities_generation::BuildingMap;
use crate::terrain::generation::resources::resource_generation::ResourceMap;
use crate::terrain::generation::terrain::terrain_generation::{PerlinPlaceholder, TerrainMap};
use crate::terrain::renderer::chunk_renderer::stream_chunks;
use crate::terrain::renderer::buildings_renderer::{despawn_buildings_meshes, BuildingsAssets, SpawnedBuildingsMeshes};
use crate::terrain::renderer::resource_renderer::{despawn_resource_meshes, ResourceAssets, SpawnedResourceMeshes};
use crate::terrain::renderer::terrain_renderer::{despawn_terrain_meshes, SpawnedTerrainMeshes, TerrainAssets};
use crate::terrain::terrain_config::TerrainConfig;
use crate::ui::menus::AppState;

mod terrain_renderer;
mod resource_renderer;
mod buildings_renderer;
mod chunk_renderer;
mod camera_math;
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
            .add_systems(OnEnter(AppState::InGameMenu), create_map_resources)
            .add_systems(
                Update,
                (handle_input, apply_regenerate, stream_chunks).chain().run_if(in_state(AppState::InGameMenu)),
            );
    }
}

fn create_map_resources(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    //tiles
    commands.insert_resource(TerrainAssets::build(&mut materials));
    //resources
    commands.insert_resource(ResourceAssets::build(&mut materials));
    //buildings
    commands.insert_resource(BuildingsAssets::build(&mut materials));

    let config = TerrainConfig::default();
    let perlin = PerlinPlaceholder::default();
    let terrain = TerrainMap::new(&config, &perlin);
    let resources = ResourceMap::new(&terrain, &config);
    let cities = BuildingMap::new(&config, &terrain, &resources);

    commands.insert_resource(Rendered::default());
    commands.insert_resource(config);
    commands.insert_resource(perlin);
    commands.insert_resource(terrain);
    commands.insert_resource(resources);
    commands.insert_resource(cities);
    commands.insert_resource(ClearColor(Color::BLACK))

}
fn apply_regenerate(
    mut events: MessageReader<RegenerateWorldEvent>,
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    mut meshes: ResMut<Assets<Mesh>>,

) {
    if events.read().count() == 0 {
        return;
    }

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