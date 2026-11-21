use crate::game::game_logic_plugin::action::action_systems::action_router;
use crate::game::renderer::fog_memory::{ChunkSnapshot, FogMemory, update_fog_memory};
use crate::game::renderer::render_actions::{animate_smooth_move, render_move};
use crate::game::renderer::{
    buildings_renderer::{
        SpawnedBuildingsMeshes, despawn_buildings_meshes, spawn_buildings_meshes,
    },
    chunk_renderer::stream_chunks,
    fog_renderer::{SpawnedFogMeshes, despawn_fog_meshes, spawn_fog_meshes},
    resource_renderer::{SpawnedResourceMeshes, despawn_resource_meshes, spawn_resource_meshes},
    terrain_renderer::{SpawnedTerrainMeshes, despawn_terrain_meshes, spawn_terrain_meshes},
    units_renderer::{SpawnedUnitsMeshes, despawn_units_meshes, sync_unit_visuals},
};
use crate::ui::menus::AppState;
use crate::utils_debug::regenerate_world::{RegenerateWorldEvent, handle_input};
use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::fog_map::FogMap;
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    App, AssetServer, Assets, ClearColor, Color, Commands, DetectChanges, IVec2,
    IntoScheduleConfigs, Mesh, MessageReader, OnEnter, Plugin, Res, ResMut, Resource,
    StandardMaterial, Update, in_state,
};

pub mod buildings_renderer;
pub mod camera_math;
pub mod chunk_renderer;
pub mod fog_memory;
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
    /// What each loaded chunk currently renders. Used to rebuild only changed
    /// layers instead of every loaded chunk.
    pub snapshots: HashMap<IVec2, ChunkSnapshot>,
}

pub struct MapRenderPlugin;

impl Plugin for MapRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FogMemory>()
            .add_message::<RegenerateWorldEvent>()
            .add_systems(OnEnter(AppState::InGameRenderer), insert_assets)
            .add_systems(
                Update,
                update_fog_memory
                    .after(action_router)
                    .run_if(in_state(AppState::InGameRenderer)),
            )
            .add_systems(
                Update,
                (
                    handle_input,
                    despawn,
                    stream_chunks,
                    render_move,
                    sync_unit_visuals,
                    animate_smooth_move,
                )
                    .chain()
                    .after(update_fog_memory)
                    .run_if(in_state(AppState::InGameRenderer)),
            )
            .add_systems(
                Update,
                refresh_visible_chunks
                    .after(stream_chunks)
                    .after(update_fog_memory)
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}

fn insert_assets(mut commands: Commands) {
    // rendered
    commands.insert_resource(Rendered::default());
    commands.insert_resource(FogMemory::default());
    // visual note переделается
    commands.insert_resource(ClearColor(Color::BLACK))
}

fn despawn(
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    mut memory: ResMut<FogMemory>,
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
    for (_, fog_meshes) in &mut rendered.fog.drain() {
        despawn_fog_meshes(&mut commands, &mut meshes, fog_meshes);
    }
    rendered.snapshots.clear();
    memory.clear();
}

/// Rebuilds only the chunk layers whose visible content actually changed.
///
/// `FogMemory` already resolves "live now vs. last seen", so a layer only needs
/// a rebuild when the resolved per-tile value differs from what is currently
/// rendered. Static layers are independent from each other: a unit move no
/// longer re-renders the whole chunk.
fn refresh_visible_chunks(
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    config: Res<RenderConfig>,
    memory: Res<FogMemory>,
    fog_map: Res<FogMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if !memory.is_changed() && !fog_map.is_changed() {
        return;
    }

    let coords: Vec<IVec2> = rendered.snapshots.keys().copied().collect();

    for coord in coords {
        let Some(old) = rendered.snapshots.get(&coord) else {
            continue;
        };
        let new = ChunkSnapshot::capture(&config, &memory, &fog_map, coord);

        let fog_changed = new.fog_differs(old);
        let terrain_changed = new.terrain_differs(old);
        let resources_changed = new.resources_differ(old);
        let buildings_changed = new.buildings_differ(old);

        if fog_changed {
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

        if terrain_changed {
            if let Some(old) = rendered.terrain.remove(&coord) {
                despawn_terrain_meshes(&mut commands, &mut meshes, old);
            }
            let spawned = spawn_terrain_meshes(&mut commands, &config, &mut meshes, coord, &memory);
            rendered.terrain.insert(coord, spawned);
        }

        if resources_changed {
            if let Some(old) = rendered.resources.remove(&coord) {
                despawn_resource_meshes(&mut commands, &mut meshes, old);
            }
            let spawned =
                spawn_resource_meshes(&mut commands, &config, &mut meshes, coord, &memory);
            rendered.resources.insert(coord, spawned);
        }

        if buildings_changed {
            if let Some(old) = rendered.buildings.remove(&coord) {
                despawn_buildings_meshes(&mut commands, old);
            }
            let spawned = spawn_buildings_meshes(&mut commands, &config, coord, &memory);
            rendered.buildings.insert(coord, spawned);
        }

        rendered.snapshots.insert(coord, new);
    }
}
