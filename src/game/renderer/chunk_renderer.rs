use std::time::Instant;

use bevy::prelude::{
    AssetServer, Assets, Commands, IVec2, Mesh, Projection, Query, Res, ResMut, StandardMaterial,
    Transform, Window, With,
};
use bevy::window::PrimaryWindow;

use crate::game::{
    camera_plugin::camera::MainCamera,
    renderer::{
        Rendered,
        buildings_renderer::{despawn_buildings_meshes, spawn_buildings_meshes},
        camera_math::{
            camera_chunk_bounds, camera_ground_point, chunk_coord_from_world_xz, outside_bounds,
        },
        fog_renderer::{despawn_fog_meshes, spawn_fog_meshes},
        resource_renderer::{despawn_resource_meshes, spawn_resource_meshes},
        terrain_renderer::{despawn_terrain_meshes, spawn_terrain_meshes},
        units_renderer::{despawn_units_meshes, spawn_units_meshes},
    },
};
use crate::world::components::PlayerRace;
use crate::world::game_configs::RenderConfig;
use crate::world::game_resources::WorldMaps;
use crate::world::game_resources::data::building::BuildingType;
use crate::world::game_resources::data::unit::UnitType;

#[allow(clippy::too_many_arguments)]
pub fn stream_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut rendered: ResMut<Rendered>,
    config: Res<RenderConfig>,
    world_params: WorldMaps,
    camera_q: Query<(&Transform, &Projection), With<MainCamera>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    visual_b_q: Query<(&PlayerRace, &BuildingType)>,
    visual_u_q: Query<(&PlayerRace, &UnitType)>,
) {
    let Ok((camera_transform, projection)) = camera_q.single() else {
        return;
    };
    let Ok(window) = window_q.single() else {
        return;
    };

    let center = camera_ground_point(camera_transform);
    let center_chunk = chunk_coord_from_world_xz(center.x, center.z, &config);

    let (load_min, load_max) = calculate_load_area(&config, camera_transform, projection, window);

    let has_load_area = load_min.x <= load_max.x && load_min.y <= load_max.y;

    unload_distant_chunks(
        &mut commands,
        &mut meshes,
        &config,
        &mut rendered,
        load_min,
        load_max,
        has_load_area,
    );

    if !has_load_area {
        return;
    }

    let missing_chunks = get_sorted_missing_chunks(&rendered, load_min, load_max, center_chunk);

    load_missing_chunks(
        &mut commands,
        &config,
        &mut meshes,
        &mut rendered,
        missing_chunks,
        world_params,
        &asset_server,
        &mut materials,
        visual_b_q,
        visual_u_q,
    );
}

fn calculate_load_area(
    config: &RenderConfig,
    camera_transform: &Transform,
    projection: &Projection,
    window: &Window,
) -> (IVec2, IVec2) {
    let preload = config.preload_margin_chunks as i32;

    let (mut min, mut max) = camera_chunk_bounds(camera_transform, projection, window, config);

    min -= preload;
    max += preload;
    let max_chunk_index = (config.terrain_size as i32 / config.chunk_size as i32) - 1;

    min = min.clamp(IVec2::ZERO, IVec2::splat(max_chunk_index));
    max = max.clamp(IVec2::ZERO, IVec2::splat(max_chunk_index));

    (min, max)
}

fn unload_distant_chunks(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    config: &RenderConfig,
    rendered: &mut Rendered,
    load_min: IVec2,
    load_max: IVec2,
    has_load_area: bool,
) {
    let (unload_min, unload_max) = if has_load_area {
        let unload_padding = IVec2::splat(config.unload_margin_chunks as i32);
        let max_chunk_index = (config.terrain_size as i32 / config.chunk_size as i32) - 1;

        let mut min = load_min - unload_padding;
        let mut max = load_max + unload_padding;

        min = min.clamp(IVec2::ZERO, IVec2::splat(max_chunk_index));
        max = max.clamp(IVec2::ZERO, IVec2::splat(max_chunk_index));

        (min, max)
    } else {
        (IVec2::ONE, IVec2::ZERO)
    };

    let to_unload: Vec<IVec2> = rendered
        .terrain
        .keys()
        .copied()
        .filter(|coord| outside_bounds(*coord, unload_min, unload_max))
        .collect();

    for coord in to_unload {
        if let Some(terrain_meshes) = rendered.terrain.remove(&coord) {
            despawn_terrain_meshes(commands, meshes, terrain_meshes);
        }
        if let Some(resource_meshes) = rendered.resources.remove(&coord) {
            despawn_resource_meshes(commands, meshes, resource_meshes);
        }
        if let Some(buildings_meshes) = rendered.buildings.remove(&coord) {
            despawn_buildings_meshes(commands, buildings_meshes);
        }
        if let Some(fog_meshes) = rendered.fog.remove(&coord) {
            despawn_fog_meshes(commands, meshes, fog_meshes);
        }
        if let Some(units_meshes) = rendered.units.remove(&coord) {
            despawn_units_meshes(commands, units_meshes);
        }
    }
}

fn get_sorted_missing_chunks(
    rendered: &Rendered,
    load_min: IVec2,
    load_max: IVec2,
    center_chunk: IVec2,
) -> Vec<IVec2> {
    let mut missing_chunks = Vec::new();

    for x in load_min.x..=load_max.x {
        for y in load_min.y..=load_max.y {
            let coord = IVec2::new(x, y);
            if !rendered.terrain.contains_key(&coord) {
                missing_chunks.push(coord);
            }
        }
    }

    missing_chunks.sort_by_key(|coord| {
        let delta = *coord - center_chunk;
        let chebyshev = delta.x.abs().max(delta.y.abs());
        let manhattan = delta.x.abs() + delta.y.abs();
        (chebyshev, manhattan)
    });

    missing_chunks
}

#[allow(clippy::too_many_arguments)]
fn load_missing_chunks(
    commands: &mut Commands,
    config: &RenderConfig,
    meshes: &mut ResMut<Assets<Mesh>>,
    rendered: &mut Rendered,
    missing_chunks: Vec<IVec2>,
    world_params: WorldMaps,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
    visual_b_q: Query<(&PlayerRace, &BuildingType)>,
    visual_u_q: Query<(&PlayerRace, &UnitType)>,
) {
    let start_time = Instant::now();

    for coord in missing_chunks.into_iter() {
        if start_time.elapsed().as_millis() > 5 {
            break;
        }

        if rendered.terrain.contains_key(&coord) {
            continue;
        }

        let terrain_meshes =
            spawn_terrain_meshes(commands, config, meshes, coord, &world_params.terrain);

        let resource_meshes =
            spawn_resource_meshes(commands, config, meshes, coord, &world_params.resources);

        let building_meshes =
            spawn_buildings_meshes(commands, config, coord, &world_params.buildings, visual_b_q);

        let fog_meshes = spawn_fog_meshes(
            commands,
            config,
            meshes,
            materials,
            coord,
            asset_server,
            &world_params.fog,
        );

        let units_meshes =
            spawn_units_meshes(commands, config, coord, &world_params.units, visual_u_q);

        rendered.terrain.insert(coord, terrain_meshes);
        rendered.resources.insert(coord, resource_meshes);
        rendered.buildings.insert(coord, building_meshes);
        rendered.fog.insert(coord, fog_meshes);
        rendered.units.insert(coord, units_meshes);
    }
}
