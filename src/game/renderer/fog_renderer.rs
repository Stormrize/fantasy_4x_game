use bevy::prelude::{
    AssetServer, Assets, Commands, Entity, Handle, IVec2, Image, Mesh, Mesh3d, MeshMaterial3d,
    Meshable, Plane3d, StandardMaterial, Transform, default,
};

use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::fog_map::FogMap;

pub struct SpawnedFogMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

pub fn spawn_fog_meshes(
    commands: &mut Commands,
    config: &RenderConfig,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    coord: IVec2,
    asset_server: &AssetServer,
    fog_map: &FogMap,
) -> SpawnedFogMeshes {
    let chunk_size = config.chunk_size;
    let tile_size = config.tile_size;

    let mut spawned = SpawnedFogMeshes {
        entities: Vec::with_capacity(chunk_size),
        mesh_handles: Vec::with_capacity(chunk_size),
    };

    let min_y = coord.y * chunk_size as i32;
    let min_x = coord.x * chunk_size as i32;
    let max_x = coord.x * chunk_size as i32 + chunk_size as i32;
    let max_y = coord.y * chunk_size as i32 + chunk_size as i32;

    for row in min_x..max_x {
        for col in min_y..max_y {
            if fog_map.is_general_visible_player(row as usize, col as usize) {
                continue;
            }

            let handle: Handle<Image> = asset_server.load("fog.png");

            let mesh_handle = meshes.add(Plane3d::default().mesh().size(tile_size, tile_size));

            let entity = commands
                .spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(handle.clone()),
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_xyz(
                        (row + 1) as f32 * tile_size,
                        1.0,
                        (col + 1) as f32 * tile_size,
                    ),
                ))
                .id();

            spawned.entities.push(entity);
            spawned.mesh_handles.push(mesh_handle.clone());
        }
    }

    spawned
}

pub fn despawn_fog_meshes(
    commands: &mut Commands,
    images: &mut Assets<Mesh>,
    spawned: SpawnedFogMeshes,
) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }

    for mesh_handle in spawned.mesh_handles {
        images.remove(mesh_handle.id());
    }
}
