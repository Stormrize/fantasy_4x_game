use crate::world::ui_and_graphic_resources::player_registry::PlayerRegistry;
use crate::world::{
    game_config::GameConfig,
    game_resources::{data::biome::BiomeType, terrain_map::TerrainMap},
    ui_and_graphic_resources::player_registry::RacesAssetsRegistry,
};
use bevy::prelude::{
    Assets, Commands, Component, Entity, Handle, IVec2, Mesh, Mesh3d, MeshMaterial3d, Plane3d, Res,
    Transform,
};

#[derive(Component)]
pub struct TerrainMesh {
    pub coord: IVec2,
    pub biome: BiomeType,
}

pub struct SpawnedTerrainMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

#[allow(dead_code)]
#[cfg(false)]
pub fn spawn_terrain_meshes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    assets: &TerrainAssets,
    terrain: &TerrainMap,
) -> SpawnedTerrainMeshes {
    let chunk_world = chunk_size as f32 * tile_size;
    let chunk_origin = Vec3::new(
        coord.x as f32 / chunk_world,
        0.0,
        coord.y as f32 / chunk_world,
    );

    let mut spawned = SpawnedTerrainMeshes {
        entities: Vec::with_capacity(BiomeType::ALL.len()),
        mesh_handles: Vec::with_capacity(BiomeType::ALL.len()),
    };

    for biome in BiomeType::ALL {
        let Some(mesh) = build_biome_surface_mesh(biome, coord, terrain) else {
            continue;
        };

        let mesh_handle = meshes.add(mesh);

        let entity = commands
            .spawn((
                TerrainMesh { coord, biome },
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(assets.material_for(biome)),
                Transform::from_translation(chunk_origin),
            ))
            .id();
        spawned.entities.push(entity);
        spawned.mesh_handles.push(mesh_handle);
    }
    spawned
}

pub fn spawn_terrain_meshes(
    commands: &mut Commands,
    config: &GameConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    assets_registry: &RacesAssetsRegistry,
    terrain: &TerrainMap,
    player_registry: &PlayerRegistry,
) -> SpawnedTerrainMeshes {
    // TODO можно поменять тайлы на гексагоны изза боевки

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

            let player_id = biome.inside();

            if player_id.is_none() {
                continue;
            }

            let player_race = player_registry.race_of(player_id.unwrap());

            let handle = assets_registry
                .race_assets_of(player_race)
                .get_biome(biome.index());

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

#[allow(dead_code)]
#[cfg(false)]
fn build_biome_surface_mesh(
    biome: BiomeType,
    coord: IVec2,
    terrain: &GlobalTerrainMap,
    config: &GameConfig,
) -> Option<Mesh> {
    let chunk_size = config.terrain_size;
    let tile_size = config.tile_size;

    let tile_size = tile_size;
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    const INV_128: f32 = 1.0 / 128.0;

    let min_y = coord.y * chunk_size as i32;
    let min_x = coord.x * chunk_size as i32;
    let max_x = coord.x * chunk_size as i32 + chunk_size as i32;
    let max_y = coord.y * chunk_size as i32 + chunk_size as i32;

    for col in min_y..max_y {
        for row in min_x..max_x {
            let tile_biome = terrain.get(col as usize, row as usize);
            if tile_biome != biome {
                continue;
            }

            let base = positions.len() as u32;
            let x0 = col as f32 * tile_size;
            let x1 = x0 + tile_size;
            let z0 = row as f32 * tile_size;
            let z1 = z0 + tile_size;

            let y = f32::from(tile_biome.visual_height()) * INV_128 * tile_size;

            positions.extend_from_slice(&[[x0, y, z0], [x0, y, z1], [x1, y, z1], [x1, y, z0]]);
            normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
            uvs.extend_from_slice(&[[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]]);
            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }
    }

    if positions.is_empty() {
        return None;
    }

    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}
