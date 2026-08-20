use crate::world::game_config::GameConfig;
use crate::world::game_resources::data::resources::ResourceType;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::ui_and_graphic_resources::visual_race_dataset::VisualRacesData;
use bevy::prelude::{
    Assets, Commands, Component, Cone, Entity, Handle, IVec2, Mesh, Mesh3d, MeshMaterial3d,
    Transform,
};

#[derive(Component)]
pub struct UnitMesh {
    pub coord: IVec2,
    pub resource: ResourceType,
}

pub struct SpawnedUnitsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

pub fn spawn_units_meshes(
    commands: &mut Commands,
    config: &GameConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    assets: &VisualRacesData,
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

            let player_race = unit.inside();

            if player_race.is_none() {
                continue;
            }

            //todo пока нет асетов для юнитов
            let handle = assets.0[player_race.unwrap()].get_resource(2);

            let mesh_handle = meshes.add(Cone::new(0.3, 0.5));

            let entity = commands
                .spawn((
                    Transform::from_xyz(row as f32 * tile_size, 0.3, col as f32 * tile_size),
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

pub fn despawn_units_meshes(
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
