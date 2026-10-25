use crate::world::components::PlayerRace;
use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::building::BuildingType;
use bevy::math::Vec3;
use bevy::prelude::{
    Assets, Commands, Component, Entity, Handle, IVec2, Mesh, Mesh3d, MeshMaterial3d, Query,
    Sphere, Transform, WorldAsset, WorldAssetRoot,
};

#[derive(Component)]
pub struct BuildingsMesh {
    pub coord: IVec2,
    pub building: BuildingType,
}

pub struct SpawnedBuildingsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<WorldAsset>>,
}

pub fn spawn_buildings_meshes(
    commands: &mut Commands,
    config: &RenderConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    buildings: &BuildingMap,
    visual_q: Query<(&PlayerRace, &BuildingType)>,
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
            let building = buildings.get(row as usize, col as usize);
            let Some(building) = building else {
                continue;
            };

            let (player_race, building_type) = visual_q.get(building).unwrap();

            let Some(race_assets) = config.assets_registry.get(player_race) else {
                continue;
            };

            let handle = race_assets.get_building(building_type.index());

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
