use bevy::math::Quat;
use bevy::prelude::{
    Assets, Commands, Component, Entity, Handle, IVec2, Mesh, Query, Transform, WorldAsset,
    WorldAssetRoot,
};

use crate::world::components::PlayerRace;
use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::data::unit::UnitType;
use crate::world::game_resources::{data::resource::ResourceType, unit_map::UnitsMap};

#[derive(Component)]
pub struct UnitMesh {
    pub coord: IVec2,
    pub resource: ResourceType,
}

pub struct SpawnedUnitsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<WorldAsset>>,
}
#[derive(Component)]
pub struct UnitVisual(pub Entity);

pub fn spawn_units_meshes(
    commands: &mut Commands,
    config: &RenderConfig,
    meshes: &mut Assets<Mesh>,
    coord: IVec2,
    units: &UnitsMap,
    visual_q: Query<(&PlayerRace, &UnitType)>,
) -> SpawnedUnitsMeshes {
    let chunk_size = config.chunk_size;
    let tile_size = config.tile_size;

    let mut spawned = SpawnedUnitsMeshes {
        entities: Vec::new(),
        mesh_handles: Vec::new(),
    };

    let min_y = coord.y * chunk_size as i32;
    let min_x = coord.x * chunk_size as i32;
    let max_x = coord.x * chunk_size as i32 + chunk_size as i32;
    let max_y = coord.y * chunk_size as i32 + chunk_size as i32;

    for row in min_x..max_x {
        for col in min_y..max_y {
            let Some(unit) = units.get(row as usize, col as usize) else {
                continue;
            };

            let Ok((player_race, unit_type)) = visual_q.get(unit) else {
                continue;
            };

            let Some(race_assets) = config.assets_registry.get(player_race) else {
                continue;
            };

            if unit_type.index() != 4 {
                continue;
            }

            let handle = race_assets.get_unit(2);

            //let mesh_handle = meshes.add(Cone::new(0.3, 0.5));

            let entity = commands
                .spawn((
                    Transform::from_xyz(row as f32 * tile_size, 0.0, col as f32 * tile_size)
                        .with_rotation(Quat::from_rotation_y(180.0f32.to_radians())),
                    WorldAssetRoot(handle.clone()),
                ))
                .id();
            spawned.entities.push(entity);
            spawned.mesh_handles.push(handle);

            commands.entity(unit).insert(UnitVisual(entity));
        }
    }

    spawned
}

// pub fn despawn_units_meshes(
//     commands: &mut Commands,
//     meshes: &mut Assets<Mesh>,
//     spawned: SpawnedUnitsMeshes,
// ) {
//     for entity in spawned.entities {
//         commands.entity(entity).despawn();
//     }
//
//     for mesh_handle in spawned.mesh_handles {
//         meshes.remove(mesh_handle.id());
//     }
// }
