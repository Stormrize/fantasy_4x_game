use bevy::math::IVec2;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{default, Assets, Commands, Component, Entity, Handle, Mesh, Mesh3d, Resource, Sphere, StandardMaterial, Transform, UVec2, Vec3};
use crate::terrain::enums::buildings::BuildingType;
use crate::terrain::generation::cities::cities_generation::BuildingMap;
use crate::terrain::terrain_config::TerrainConfig;

#[derive(Component)]
pub struct BuildingsMesh {
    pub coord: IVec2,
    pub building: BuildingType,
}


#[derive(Resource)]
pub struct BuildingsAssets {
    pub materials: [Handle<StandardMaterial>; 8]
}

pub struct SpawnedBuildingsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

impl BuildingsAssets {
    pub fn build(materials: &mut Assets<StandardMaterial>) -> Self {
        let mat_handles: Vec<Handle<StandardMaterial>> = BuildingType::ALL
            .iter()
            .map(|&building | {
                materials.add(StandardMaterial {
                    base_color: building.color(),
                    ..default()
                })
            })
            .collect();

        Self {
            materials: mat_handles
                .try_into()
                .expect("Check ResourceType::ALL")
        }
    }

    #[inline]
    pub fn material_for(&self, resource: BuildingType) -> Handle<StandardMaterial> {
        self.materials[resource.index()].clone()
    }
}

pub fn spawn_buildings_meshes(
    commands: &mut Commands,
    config: &TerrainConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    assets: &BuildingsAssets,
    buildings: &BuildingMap,
) -> SpawnedBuildingsMeshes {

    let chunk_size = config.chunk_size;
    let tile = config.tile_size;
    let mut spawned = SpawnedBuildingsMeshes {
        entities: Vec::with_capacity(chunk_size),
        mesh_handles: Vec::with_capacity(chunk_size)
    };


    let min_y = coord.y * chunk_size as i32;
    let min_x = coord.x * chunk_size as i32;
    let max_x = coord.x * chunk_size as i32 + chunk_size as i32;
    let max_y = coord.y * chunk_size as i32 + chunk_size as i32;

    for col in min_y..max_y {
        for row in min_x..max_x {
            let building = buildings.get(col as usize, row as usize);
            if building == BuildingType::None {
                continue;
            }

            let mesh_handle = meshes.add(Sphere::new(0.5));

            let entity = commands.spawn((
                Transform::from_xyz(
                    col as f32 * tile,
                    0.5,
                    row as f32 * tile,
                ),
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(assets.material_for(building)),
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