use crate::world::enums::resources::ResourceType;
use crate::world::terrain_config::TerrainConfig;
use bevy::math::IVec2;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{
    default, Assets, Commands, Component, Cuboid, Entity, Handle, Mesh, Mesh3d, Resource,
    StandardMaterial, Transform,
};
use crate::world::resource_map::ResourceMap;

#[derive(Component)]
pub struct ResourceMesh {
    pub coord: IVec2,
    pub resource: ResourceType,
}

#[derive(Resource)]
pub struct ResourceAssets {
    pub materials: [Handle<StandardMaterial>; 10],
}

pub struct SpawnedResourceMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

impl ResourceAssets {
    pub fn build(materials: &mut Assets<StandardMaterial>) -> Self {
        let mat_handles: Vec<Handle<StandardMaterial>> = ResourceType::ALL
            .iter()
            .map(|&resource| {
                materials.add(StandardMaterial {
                    base_color: resource.color(),
                    ..default()
                })
            })
            .collect();

        Self {
            materials: mat_handles.try_into().expect("Check ResourceType::ALL"),
        }
    }

    #[inline]
    pub fn material_for(&self, resource: ResourceType) -> Handle<StandardMaterial> {
        self.materials[resource.index()].clone()
    }
}

pub fn spawn_resource_meshes(
    commands: &mut Commands,
    config: &TerrainConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    assets: &ResourceAssets,
    resources: &ResourceMap,
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
            let resource = resources.get(row as usize, col as usize);
            if resource == ResourceType::None {
                continue;
            }

            let mesh_handle = meshes.add(Cuboid::new(0.3, 0.3, 0.3));

            let entity = commands
                .spawn((
                    Transform::from_xyz(row as f32 * tile_size, 0.3, col as f32 * tile_size),
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(assets.material_for(resource)),
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
