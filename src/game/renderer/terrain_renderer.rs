use bevy::prelude::{
    Assets, Commands, Component, Entity, Handle, IVec2, Mesh, Mesh3d, MeshMaterial3d, Plane3d, Res,
    Transform,
};

use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::{data::biome::BiomeType, terrain_map::TerrainMap};

#[derive(Component)]
pub struct TerrainMesh {
    pub coord: IVec2,
    pub biome: BiomeType,
}

pub struct SpawnedTerrainMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

pub fn spawn_terrain_meshes(
    commands: &mut Commands,
    config: &RenderConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    terrain: &TerrainMap,
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
            let biome = terrain.get(row as usize, col as usize);

            let race_inside = biome.race_inside();

            let Some(race) = race_inside else {
                continue;
            };

            let handle = config.assets_registry.get(race).unwrap_or_else(|| {
                panic!("assets for this race do not exist, if it is moded race, dont forget to configure assets");
            }).get_biome(biome.index());

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
