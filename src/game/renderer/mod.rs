use crate::game::renderer::render_actions::render_move;
use crate::game::renderer::{
    buildings_renderer::{
        SpawnedBuildingsMeshes, despawn_buildings_meshes, spawn_buildings_meshes,
    },
    chunk_renderer::stream_chunks,
    fog_renderer::{SpawnedFogMeshes, despawn_fog_meshes, spawn_fog_meshes},
    resource_renderer::{SpawnedResourceMeshes, despawn_resource_meshes, spawn_resource_meshes},
    terrain_renderer::{SpawnedTerrainMeshes, despawn_terrain_meshes},
    units_renderer::{SpawnedUnitsMeshes, despawn_units_meshes, spawn_units_meshes},
};
use crate::ui::menus::AppState;
use crate::utils_debug::regenerate_world::{RegenerateWorldEvent, handle_input};
use crate::world::components::PlayerRace;
use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::building::BuildingType;
use crate::world::game_resources::data::unit::UnitType;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::unit_map::UnitsMap;
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    App, AssetServer, Assets, ClearColor, Color, Commands, DetectChanges, IVec2,
    IntoScheduleConfigs, Mesh, MessageReader, OnEnter, Plugin, Query, Res, ResMut, Resource,
    StandardMaterial, Update, in_state,
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
            )
            .add_systems(
                Update,
                (
                    refresh_fog_chunks,
                    refresh_units_chunks,
                    refresh_buildings_chunks,
                    refresh_resources_chunks,
                )
                    .after(stream_chunks)
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

// todo сделать эффективней не все рефрешить точечно плюс эффекты, пока норм заповнить
/// Пересоздаёт туман в загруженных чанках, когда карта тумана изменилась.
fn refresh_fog_chunks(
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    config: Res<RenderConfig>,
    fog_map: Res<FogMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if !fog_map.is_changed() {
        return;
    }
    let coords: Vec<IVec2> = rendered.fog.keys().copied().collect();
    for coord in coords {
        if let Some(old) = rendered.fog.remove(&coord) {
            despawn_fog_meshes(&mut commands, &mut meshes, old);
        }
        let spawned = spawn_fog_meshes(
            &mut commands,
            &config,
            &mut meshes,
            &mut materials,
            coord,
            &asset_server,
            &fog_map,
        );
        rendered.fog.insert(coord, spawned);
    }
}

/// Пересоздаёт юнитов в загруженных чанках, когда карта юнитов изменилась.
fn refresh_units_chunks(
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    config: Res<RenderConfig>,
    units_map: Res<UnitsMap>,
    visual_q: Query<(&PlayerRace, &UnitType)>,
) {
    if !units_map.is_changed() {
        return;
    }
    let coords: Vec<IVec2> = rendered.units.keys().copied().collect();
    for coord in coords {
        if let Some(old) = rendered.units.remove(&coord) {
            despawn_units_meshes(&mut commands, old);
        }
        let spawned = spawn_units_meshes(&mut commands, &config, coord, &units_map, visual_q);
        rendered.units.insert(coord, spawned);
    }
}

/// Пересоздаёт здания в загруженных чанках, когда карта зданий изменилась.
fn refresh_buildings_chunks(
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    config: Res<RenderConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    buildings_map: Res<BuildingMap>,
    visual_q: Query<(&PlayerRace, &BuildingType)>,
) {
    if !buildings_map.is_changed() {
        return;
    }
    let coords: Vec<IVec2> = rendered.buildings.keys().copied().collect();
    for coord in coords {
        if let Some(old) = rendered.buildings.remove(&coord) {
            despawn_buildings_meshes(&mut commands, old);
        }
        let spawned = spawn_buildings_meshes(
            &mut commands,
            &config,
            &mut meshes,
            coord,
            &buildings_map,
            visual_q,
        );
        rendered.buildings.insert(coord, spawned);
    }
}

/// Пересоздаёт ресурсы в загруженных чанках, когда карта ресурсов изменилась.
fn refresh_resources_chunks(
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    config: Res<RenderConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    resources: Res<ResourceMap>,
) {
    if !resources.is_changed() {
        return;
    }
    let coords: Vec<IVec2> = rendered.resources.keys().copied().collect();
    for coord in coords {
        if let Some(old) = rendered.resources.remove(&coord) {
            despawn_resource_meshes(&mut commands, &mut meshes, old);
        }
        let spawned = spawn_resource_meshes(&mut commands, &config, &mut meshes, coord, &resources);
        rendered.resources.insert(coord, spawned);
    }
}
