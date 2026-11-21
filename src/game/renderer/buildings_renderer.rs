use bevy::math::Vec3;
use bevy::prelude::{Commands, Entity, Handle, IVec2, Transform, WorldAsset, WorldAssetRoot};

use crate::game::renderer::fog_memory::FogMemory;
use crate::world::game_configs::RenderConfig;

pub struct SpawnedBuildingsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<WorldAsset>>,
}

/// Spawns buildings for every explored tile of a chunk, using remembered data
/// for explored-only tiles.
pub fn spawn_buildings_meshes(
    commands: &mut Commands,
    config: &RenderConfig,
    coord: IVec2,
    memory: &FogMemory,
) -> SpawnedBuildingsMeshes {
    let chunk_size = config.chunk_size;
    let tile_size = config.tile_size;
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
            let Some(tile) = memory.get(row as usize, col as usize) else {
                continue;
            };
            let Some(building) = &tile.building else {
                continue;
            };

            let Some(race_assets) = config.assets_registry.get(&building.race) else {
                continue;
            };

            let handle = race_assets.get_building(building.index);

            let entity = commands
                .spawn((
                    Transform::from_xyz((row as f32) * tile_size, 0.0, (col as f32) * tile_size)
                        .with_scale(Vec3::splat(0.07)),
                    WorldAssetRoot(handle.clone()),
                ))
                .id();

            spawned.entities.push(entity);
            spawned.mesh_handles.push(handle);
        }
    }

    spawned
}

pub fn despawn_buildings_meshes(commands: &mut Commands, spawned: SpawnedBuildingsMeshes) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }
}
