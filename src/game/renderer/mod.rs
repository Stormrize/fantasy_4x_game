use crate::game::renderer::render_actions::render_move;
use crate::game::renderer::{
    buildings_renderer::{SpawnedBuildingsMeshes, despawn_buildings_meshes},
    chunk_renderer::stream_chunks,
    fog_renderer::SpawnedFogMeshes,
    resource_renderer::{SpawnedResourceMeshes, despawn_resource_meshes},
    terrain_renderer::{SpawnedTerrainMeshes, despawn_terrain_meshes},
    units_renderer::{SpawnedUnitsMeshes, despawn_units_meshes},
};
use crate::ui::menus::AppState;
use crate::utils_debug::regenerate_world::{RegenerateWorldEvent, handle_input};
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    App, Assets, ClearColor, Color, Commands, IVec2, IntoScheduleConfigs, Mesh, MessageReader,
    OnEnter, Plugin, ResMut, Resource, Update, in_state,
};

pub mod buildings_renderer;
pub mod camera_math;
pub mod chunk_renderer;
mod fog_renderer;
mod render_actions;
pub mod resource_renderer;
pub mod terrain_renderer;
mod units_renderer;

#[derive(Resource, Default)]
pub struct Rendered {
    pub terrain: HashMap<IVec2, SpawnedTerrainMeshes>,
    pub resources: HashMap<IVec2, SpawnedResourceMeshes>,
    pub buildings: HashMap<IVec2, SpawnedBuildingsMeshes>,
    pub units: HashMap<IVec2, SpawnedUnitsMeshes>,
    pub fog: HashMap<IVec2, SpawnedFogMeshes>,
}

pub struct MapRenderPlugin;

impl Plugin for MapRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RegenerateWorldEvent>()
            .add_systems(OnEnter(AppState::InGameRenderer), insert_assets)
            .add_systems(
                Update,
                (handle_input, despawn, stream_chunks, render_move)
                    .chain()
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}
fn insert_assets(mut commands: Commands) {
    //rendered
    commands.insert_resource(Rendered::default());
    //visual note переделается
    commands.insert_resource(ClearColor(Color::BLACK))
}
fn despawn(
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: MessageReader<RegenerateWorldEvent>,
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
        despawn_buildings_meshes(&mut commands, buildings);
    }
    for (_, units_meshes) in &mut rendered.units.drain() {
        despawn_units_meshes(&mut commands, units_meshes);
    }
}
