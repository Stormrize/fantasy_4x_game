use bevy::math::ops::atan2;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{
    Commands, Component, Entity, Handle, IVec2, Query, Transform, WorldAsset, WorldAssetRoot,
};

use crate::world::components::PlayerRace;
use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::data::unit::UnitType;
use crate::world::game_resources::unit_map::UnitsMap;

#[derive(Component)]
pub struct UnitMesh {
    pub coord: IVec2,
    pub resource: UnitType,
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

            let handle = race_assets.get_unit(unit_type.index());

            let center_f32 = (config.terrain_size / 2) as f32 * tile_size;
            let center_pos = Vec3::new(center_f32, 0.0, center_f32);
            let current_pos = Vec3::new(row as f32 * tile_size, 0.0, col as f32 * tile_size);

            let mut transform = Transform::from_translation(current_pos)
                .with_scale(Vec3::splat(0.5))
                .looking_at(center_pos, Vec3::Y);

            transform.rotate_local_y(180.0_f32.to_radians());

            let entity = commands
                .spawn((transform, WorldAssetRoot(handle.clone())))
                .id();

            spawned.entities.push(entity);
            spawned.mesh_handles.push(handle);

            commands.entity(unit).insert(UnitVisual(entity));
        }
    }

    spawned
}

pub fn despawn_units_meshes(commands: &mut Commands, spawned: SpawnedUnitsMeshes) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }
}
