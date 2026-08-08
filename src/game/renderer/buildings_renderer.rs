use crate::world::enums::buildings::BuildingType;
use crate::world::game_config::GameConfig;
use bevy::math::IVec2;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{
    default, Assets, Commands, Component, Entity, Handle, Mesh, Mesh3d, Resource, Sphere,
    StandardMaterial, Transform, UVec2, Vec3,
};
use crate::world::buildings_map::BuildingMap;

#[derive(Component)]
pub struct BuildingsMesh {
    pub coord: IVec2,
    pub building: BuildingType,
}

#[derive(Resource)]
pub struct BuildingsAssets {
    pub materials: [Handle<StandardMaterial>; 8],
}

pub struct SpawnedBuildingsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

impl BuildingsAssets {
    pub fn build(materials: &mut Assets<StandardMaterial>) -> Self {
        let mat_handles: Vec<Handle<StandardMaterial>> = BuildingType::ALL
            .iter()
            .map(|&building| {
                materials.add(StandardMaterial {
                    base_color: building.color(),
                    ..default()
                })
            })
            .collect();

        Self {
            materials: mat_handles.try_into().expect("Check BuildingType::ALL"),
        }
    }

    #[inline]
    pub fn material_for(&self, resource: BuildingType) -> Handle<StandardMaterial> {
        self.materials[resource.index()].clone()
    }
}

pub fn spawn_buildings_meshes(
    commands: &mut Commands,
    config: &GameConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    assets: &BuildingsAssets,
    buildings: &BuildingMap,
) -> SpawnedBuildingsMeshes {
    let chunk_size = config.chunk_size;
    let tile = config.tile_size;
    let mut spawned = SpawnedBuildingsMeshes {
        entities: Vec::with_capacity(chunk_size),
        mesh_handles: Vec::with_capacity(chunk_size),
    };

    let min_y = coord.y * chunk_size as i32;
    let min_x = coord.x * chunk_size as i32;
    let max_x = coord.x * chunk_size as i32 + chunk_size as i32;
    let max_y = coord.y * chunk_size as i32 + chunk_size as i32;

    for row in min_x..max_x {
        for col in min_y..max_y {
            let building = buildings.get(row as usize, col as usize);
            if building.0 != BuildingType::Capital && building.0 != BuildingType::EmptyCities {
                continue;
            }

            let mesh_handle = meshes.add(Sphere::new(0.5));

            let entity = commands
                .spawn((
                    Transform::from_xyz(row as f32 * tile, 0.5, col as f32 * tile),
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(assets.material_for(building.0)),
                ))
                .id();

            spawned.entities.push(entity);
            spawned.mesh_handles.push(mesh_handle);
        }
    }

    spawned
}

pub fn despawn_buildings_meshes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    spawned: SpawnedBuildingsMeshes,
) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }

    for mesh_handle in spawned.mesh_handles {
        meshes.remove(mesh_handle.id());
    }
}