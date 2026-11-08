use bevy::prelude::{
    Assets, Commands, Cuboid, Entity, Handle, IVec2, Mesh, Mesh3d, MeshMaterial3d, Transform,
};

use crate::game::renderer::fog_memory::FogMemory;
use crate::world::game_configs::RenderConfig;

pub struct SpawnedResourceMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

/// Spawns resources for every explored tile of a chunk, using remembered data
/// for explored-only tiles.
pub fn spawn_resource_meshes(
    commands: &mut Commands,
    config: &RenderConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    memory: &FogMemory,
) -> SpawnedResourceMeshes {
    let chunk_size = config.chunk_size;
    let tile_size = config.tile_size;

    let mut spawned = SpawnedResourceMeshes {
        entities: Vec::with_capacity(chunk_size),
        mesh_handles: Vec::with_capacity(chunk_size),
    };

    let min_y = coord.y * chunk_size as i32;
    let min_x = coord.x * chunk_size as i32;
    let max_x = coord.x * chunk_size as i32 + chunk_size as i32;
    let max_y = coord.y * chunk_size as i32 + chunk_size as i32;

    for row in min_x..max_x {
        for col in min_y..max_y {
            let Some(tile) = memory.get(row as usize, col as usize) else {
                continue;
            };
            let Some(resource) = &tile.resource else {
                continue;
            };

            let Some(race_assets) = config.assets_registry.get(&resource.race) else {
                continue;
            };

            let handle = race_assets.get_resource(resource.index);

            let mesh_handle = meshes.add(Cuboid::new(0.3, 0.3, 0.3));

            let entity = commands
                .spawn((
                    Transform::from_xyz(
                        (row as f32 + 0.2) * tile_size,
                        0.2,
                        (col as f32 + 0.2) * tile_size,
                    ),
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

pub fn despawn_resource_meshes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    spawned: SpawnedResourceMeshes,
) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }

    for mesh_handle in spawned.mesh_handles {
        meshes.remove(mesh_handle.id());
    }
}
