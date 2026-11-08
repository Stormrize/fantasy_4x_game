use std::collections::{HashMap, HashSet};

use bevy::math::Vec3;
use bevy::prelude::{
    Commands, Component, Entity, Handle, IVec2, Query, Res, ResMut, Transform, WorldAsset,
    WorldAssetRoot,
};

use crate::game::renderer::Rendered;
use crate::game::renderer::fog_memory::{FogMemory, LayerVisual};
use crate::world::components::PlayerRace;
use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::data::unit::UnitType;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::unit_map::UnitsMap;

#[derive(Default)]
pub struct SpawnedUnitsMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<WorldAsset>>,
}

/// Link from a logical unit entity to its visual mesh entity.
#[derive(Component)]
pub struct UnitVisual(pub Entity);

/// Visual-only "last seen" unit. It is rendered from `FogMemory` and has no
/// live logical entity behind it.
#[derive(Component)]
pub struct GhostUnit {
    pub tile: IVec2,
    pub visual: LayerVisual,
}

/// Synchronizes unit meshes with what the local player is allowed to see.
///
/// * fully visible units get a live visual linked through `UnitVisual`;
/// * tiles that are only explored show the unit the player last saw there;
/// * units that leave the line of sight become remembered ghosts instead of
///   disappearing without a trace.
///
/// Visuals of units that stay visible are never recreated, so smooth movement
/// keeps working.
#[allow(clippy::too_many_arguments)]
pub fn sync_unit_visuals(
    mut commands: Commands,
    mut rendered: ResMut<Rendered>,
    config: Res<RenderConfig>,
    units_map: Res<UnitsMap>,
    memory: Res<FogMemory>,
    fog: Res<FogMap>,
    visual_q: Query<(&PlayerRace, &UnitType)>,
    logical_q: Query<(Entity, &UnitVisual)>,
    ghost_q: Query<(Entity, &GhostUnit)>,
) {
    let loaded_chunks: HashSet<IVec2> = rendered.terrain.keys().copied().collect();
    let rendered_visuals: HashSet<Entity> = rendered
        .units
        .values()
        .flat_map(|group| group.entities.iter().copied())
        .collect();

    let live_tiles = collect_live_unit_tiles(&units_map);
    let desired_live = collect_desired_live(&live_tiles, &loaded_chunks, &config, &fog);
    let desired_ghosts = collect_desired_ghosts(&loaded_chunks, &config, &memory, &fog);
    let ghost_entities: HashSet<Entity> = ghost_q.iter().map(|(entity, _)| entity).collect();

    // Visuals that must stay alive: live units that are still visible and
    // ghosts whose remembered state did not change.
    let mut kept: Vec<(Entity, IVec2)> = Vec::new();
    let mut kept_logicals: HashSet<Entity> = HashSet::new();
    let mut kept_ghost_tiles: HashSet<IVec2> = HashSet::new();
    let mut handled_visuals: HashSet<Entity> = HashSet::new();

    for (logical, unit_visual) in &logical_q {
        let visual = unit_visual.0;
        if !rendered_visuals.contains(&visual) {
            // The chunk holding the visual was unloaded; drop the stale link.
            if let Ok(mut logical_entity) = commands.get_entity(logical) {
                logical_entity.remove::<UnitVisual>();
            }
            continue;
        }
        handled_visuals.insert(visual);

        if let Some(tile) = desired_live.get(&logical) {
            kept.push((visual, *tile));
            kept_logicals.insert(logical);
        } else {
            commands.entity(visual).despawn();
            if let Ok(mut logical_entity) = commands.get_entity(logical) {
                logical_entity.remove::<UnitVisual>();
            }
        }
    }

    for (entity, ghost) in &ghost_q {
        if !rendered_visuals.contains(&entity) {
            continue;
        }
        handled_visuals.insert(entity);

        if desired_ghosts.get(&ghost.tile) == Some(&ghost.visual) {
            kept.push((entity, ghost.tile));
            kept_ghost_tiles.insert(ghost.tile);
        } else {
            commands.entity(entity).despawn();
        }
    }

    // Visuals that are neither linked to a live unit nor marked as ghosts are
    // leftovers of units that were removed from the world.
    for visual in &rendered_visuals {
        if handled_visuals.contains(visual) || ghost_entities.contains(visual) {
            continue;
        }
        commands.entity(*visual).despawn();
    }

    for (logical, tile) in &desired_live {
        if kept_logicals.contains(logical) {
            continue;
        }
        let Ok((race, unit_type)) = visual_q.get(*logical) else {
            continue;
        };
        let visual = LayerVisual::new(race, unit_type.index());
        let Some(entity) = spawn_unit_mesh(&mut commands, &config, *tile, &visual) else {
            continue;
        };
        if let Ok(mut logical_entity) = commands.get_entity(*logical) {
            logical_entity.insert(UnitVisual(entity));
        }
        kept.push((entity, *tile));
    }

    for (tile, visual) in &desired_ghosts {
        if kept_ghost_tiles.contains(tile) {
            continue;
        }
        let Some(entity) = spawn_unit_mesh(&mut commands, &config, *tile, visual) else {
            continue;
        };
        commands.entity(entity).insert(GhostUnit {
            tile: *tile,
            visual: visual.clone(),
        });
        kept.push((entity, *tile));
    }

    // Rebuild the per-chunk registry from the visuals that survived. Entities
    // themselves are reused, so movement animations are not interrupted.
    rendered
        .units
        .retain(|coord, _| loaded_chunks.contains(coord));
    for group in rendered.units.values_mut() {
        group.entities.clear();
        group.mesh_handles.clear();
    }
    for coord in &loaded_chunks {
        rendered.units.entry(*coord).or_default();
    }

    for (entity, tile) in kept {
        let coord = chunk_coord(tile, config.chunk_size);
        if !loaded_chunks.contains(&coord) {
            commands.entity(entity).despawn();
            continue;
        }
        rendered
            .units
            .entry(coord)
            .or_default()
            .entities
            .push(entity);
    }
}

fn collect_live_unit_tiles(units_map: &UnitsMap) -> HashMap<Entity, IVec2> {
    let size = units_map.size;
    let mut tiles = HashMap::new();
    for (index, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else {
            continue;
        };
        tiles.insert(
            entity,
            IVec2::new((index / size) as i32, (index % size) as i32),
        );
    }
    tiles
}

fn collect_desired_live(
    live_tiles: &HashMap<Entity, IVec2>,
    loaded_chunks: &HashSet<IVec2>,
    config: &RenderConfig,
    fog: &FogMap,
) -> HashMap<Entity, IVec2> {
    live_tiles
        .iter()
        .filter(|(_, tile)| loaded_chunks.contains(&chunk_coord(**tile, config.chunk_size)))
        .filter(|(_, tile)| fog.is_full_visible_player(tile.x as usize, tile.y as usize))
        .map(|(entity, tile)| (*entity, *tile))
        .collect()
}

fn collect_desired_ghosts(
    loaded_chunks: &HashSet<IVec2>,
    config: &RenderConfig,
    memory: &FogMemory,
    fog: &FogMap,
) -> HashMap<IVec2, LayerVisual> {
    let mut ghosts = HashMap::new();
    let chunk_size = config.chunk_size as i32;

    for coord in loaded_chunks {
        let min_x = coord.x * chunk_size;
        let min_y = coord.y * chunk_size;
        for row in min_x..min_x + chunk_size {
            for col in min_y..min_y + chunk_size {
                let (r, c) = (row as usize, col as usize);
                if fog.is_full_visible_player(r, c) || !fog.is_general_visible_player(r, c) {
                    continue;
                }
                let Some(tile) = memory.get(r, c) else {
                    continue;
                };
                if let Some(unit) = &tile.unit {
                    ghosts.insert(IVec2::new(row, col), unit.clone());
                }
            }
        }
    }

    ghosts
}

fn chunk_coord(tile: IVec2, chunk_size: usize) -> IVec2 {
    let chunk_size = chunk_size as i32;
    IVec2::new(tile.x.div_euclid(chunk_size), tile.y.div_euclid(chunk_size))
}

fn spawn_unit_mesh(
    commands: &mut Commands,
    config: &RenderConfig,
    tile: IVec2,
    visual: &LayerVisual,
) -> Option<Entity> {
    let race_assets = config.assets_registry.get(&visual.race)?;
    let handle = race_assets.get_unit(visual.index);
    let tile_size = config.tile_size;

    let center_f32 = (config.terrain_size / 2) as f32 * tile_size;
    let center_pos = Vec3::new(center_f32, 0.0, center_f32);
    let current_pos = Vec3::new(tile.x as f32 * tile_size, 0.0, tile.y as f32 * tile_size);

    let mut transform = Transform::from_translation(current_pos)
        .with_scale(Vec3::splat(0.5))
        .looking_at(center_pos, Vec3::Y);
    transform.rotate_local_y(180.0_f32.to_radians());

    Some(commands.spawn((transform, WorldAssetRoot(handle))).id())
}

pub fn despawn_units_meshes(commands: &mut Commands, spawned: SpawnedUnitsMeshes) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }
}
