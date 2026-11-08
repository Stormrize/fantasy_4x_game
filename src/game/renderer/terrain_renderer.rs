use bevy::prelude::{
    Assets, Commands, Entity, Handle, IVec2, Mesh, Mesh3d, MeshMaterial3d, Plane3d, Transform,
};

use crate::game::renderer::fog_memory::FogMemory;
use crate::world::game_configs::RenderConfig;

pub struct SpawnedTerrainMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

/// Spawns terrain for every explored tile of a chunk.
///
/// Full visibility always shows the current biome. Tiles that are only explored
/// use the last state the local player saw (`FogMemory`), so they never leak
/// live information through the fog.
pub fn spawn_terrain_meshes(
    commands: &mut Commands,
    config: &RenderConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    memory: &FogMemory,
) -> SpawnedTerrainMeshes {
    // todo можно поменять тайлы на гексагоны изза боевки

    let chunk_size = config.chunk_size;
    let tile_size = config.tile_size;

    let min_y = coord.y * chunk_size as i32;
    let min_x = coord.x * chunk_size as i32;
    let max_x = coord.x * chunk_size as i32 + chunk_size as i32;
    let max_y = coord.y * chunk_size as i32 + chunk_size as i32;

    let mut spawned = SpawnedTerrainMeshes {
        entities: Vec::with_capacity(chunk_size),
        mesh_handles: Vec::with_capacity(chunk_size),
    };

    for row in min_x..max_x {
        for col in min_y..max_y {
            let Some(tile) = memory.get(row as usize, col as usize) else {
                continue;
            };
            let Some(terrain) = &tile.terrain else {
                continue;
            };

            let handle = config
                .assets_registry
                .get(&terrain.race)
                .unwrap_or_else(|| {
                    panic!("assets for this race do not exist, if it is moded race, dont forget to configure assets");
                })
                .get_biome(terrain.index);

            let mesh_handle = meshes.add(Plane3d::default());

            let entity = commands
                .spawn((
                    Transform::from_xyz(row as f32 * tile_size, 0.0, col as f32 * tile_size),
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(handle.clone()),
                ))
                .id();

            spawned.entities.push(entity);
            spawned.mesh_handles.push(mesh_handle);
        }
    }
    spawned
}

pub fn despawn_terrain_meshes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    spawned: SpawnedTerrainMeshes,
) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }

    for mesh_handle in spawned.mesh_handles {
        meshes.remove(mesh_handle.id());
    }
}
