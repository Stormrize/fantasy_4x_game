use crate::world::enums::resources::ResourceType;
use crate::world::game_config::GameConfig;
use bevy::math::IVec2;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{default, Assets, Commands, Component, Cone, Cuboid, Entity, Handle, Mesh, Mesh3d, Resource, StandardMaterial, Transform};
use crate::renderer::resource_renderer::ResourceAssets;
use crate::world::enums::units::Unit;
use crate::world::resource_map::ResourceMap;
use crate::world::unit_map::UnitsMap;

#[derive(Component)]
pub struct UnitMesh {
    pub coord: IVec2,
    pub resource: ResourceType,
}

#[derive(Resource)]
pub struct UnitsAssets {
    pub materials: [Handle<StandardMaterial>; 10],
}

pub struct SpawnedUnitsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

impl UnitsAssets {
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
    config: &GameConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    assets: &UnitsAssets,
    units: &UnitsMap,
) -> SpawnedUnitsMeshes {
    let chunk_size = config.chunk_size;
    let tile_size = config.tile_size;

    let mut spawned = SpawnedUnitsMeshes {
        entities: Vec::with_capacity(chunk_size),
        mesh_handles: Vec::with_capacity(chunk_size),
    };

    let min_y = coord.y * chunk_size as i32;
    let min_x = coord.x * chunk_size as i32;
    let max_x = coord.x * chunk_size as i32 + chunk_size as i32;
    let max_y = coord.y * chunk_size as i32 + chunk_size as i32;

    for row in min_x..max_x {
        for col in min_y..max_y {
            let unit = units.get(row as usize, col as usize);
            if unit == Unit::None {
                continue;
            }

            let mesh_handle = meshes.add(Cone::new(0.3, 0.5));

            let entity = commands
                .spawn((
                    Transform::from_xyz(row as f32 * tile_size, 0.3, col as f32 * tile_size),
                    Mesh3d(mesh_handle.clone()),
                   // MeshMaterial3d(assets.material_for(unit)),
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
    spawned: SpawnedUnitsMeshes,
) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }

    for mesh_handle in spawned.mesh_handles {
        meshes.remove(mesh_handle.id());
    }
}
