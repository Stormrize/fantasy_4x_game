use crate::world::ui_and_graphic_resources::player_registry::PlayerRegistry;
use crate::world::{
    game_config::GameConfig,
    game_resources::{buildings_map::BuildingMap, data::building::BuildingType},
    ui_and_graphic_resources::player_registry::RacesAssetsRegistry,
};
use bevy::prelude::{Assets, Commands, Component, Entity, Handle, IVec2, Mesh, Mesh3d, MeshMaterial3d, Res, Sphere, Transform, Visibility};

#[derive(Component)]
pub struct BuildingsMesh {
    pub coord: IVec2,
    pub building: BuildingType,
}

pub struct SpawnedBuildingsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

pub fn spawn_buildings_meshes(
    commands: &mut Commands,
    config: &GameConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    assets_registry: &RacesAssetsRegistry,
    buildings: &BuildingMap,
    player_registry: &PlayerRegistry,
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

            let player_id = building.inside();

            if player_id.is_none() {
                continue;
            }
            let player_race = player_registry.race_of(player_id.unwrap());
            let handle = assets_registry
                .race_assets_of(player_race)
                .get_building(building.index());
            let mesh_handle = meshes.add(Sphere::new(0.5));

            let entity = commands
                .spawn((
                    Transform::from_xyz(
                        (row as f32 + 0.5) * tile_size,
                        0.5,
                        (col as f32 + 0.5) * tile_size,
                    ),
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
