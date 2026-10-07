use bevy::prelude::{
    AssetServer, Assets, Commands, Component, Entity, Handle, IVec2, Image, Mesh, Mesh3d,
    MeshMaterial3d, Meshable, Plane3d, StandardMaterial, Transform, default,
};

use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::fog_map::FogMap;

#[derive(Component)]
pub struct FogMesh {
    pub coord: IVec2,
    pub fog: bool,
}

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
            let fog = fog_map.get(row as usize, col as usize);

            if !fog {
                continue;
            }

            let handle: Handle<Image> = asset_server.load("fog.png");

            let mesh_handle = meshes.add(Plane3d::default().mesh().size(tile_size, tile_size));

            let entity = commands
                .spawn((
                    // 1. Создаем плоский квадрат (размером 1.0 на 1.0 метр или под ваш tile_size)
                    Mesh3d(mesh_handle.clone()),
                    // 2. Делаем материал с вашей текстурой тумана и поддержкой прозрачности
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(handle.clone()),
                        unlit: true, // Туман будет светиться сам, игнорируя 3D-источники света
                        ..default()
                    })),
                    // 3. Теперь 3D координаты (X, Y, Z) отработают идеально
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
