use std::time::Instant;
use bevy::asset::Assets;
use bevy::camera::Projection;
use bevy::math::IVec2;
use bevy::mesh::Mesh;
use bevy::prelude::{Commands, Query, Res, ResMut, Transform, Window, With};
use bevy::window::PrimaryWindow;
use crate::camera::camera::MainCamera;
use crate::terrain::generation::cities::cities_generation::BuildingMap;
use crate::terrain::generation::resources::resource_generation::ResourceMap;
use crate::terrain::generation::terrain::terrain_generation::TerrainMap;
use crate::terrain::renderer::buildings_renderer::{despawn_buildings_meshes, spawn_buildings_meshes, BuildingsAssets};
use crate::terrain::renderer::camera_math::{camera_chunk_bounds, camera_ground_point, chunk_coord_from_world_xz, outside_bounds};
use crate::terrain::renderer::Rendered;
use crate::terrain::renderer::resource_renderer::{despawn_resource_meshes, spawn_resource_meshes, ResourceAssets};
use crate::terrain::renderer::terrain_renderer::{despawn_terrain_meshes, spawn_terrain_meshes, TerrainAssets};
use crate::terrain::terrain_config::TerrainConfig;

pub fn stream_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rendered: ResMut<Rendered>,
    config: Res<TerrainConfig>,
    terrain_assets: Res<TerrainAssets>,
    resource_assets: Res<ResourceAssets>,
    buildings_assets: Res<BuildingsAssets>,
    terrain: Res<TerrainMap>,
    resources: Res<ResourceMap>,
    buildings: Res<BuildingMap>,
    camera_q: Query<(&Transform, &Projection), With<MainCamera>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok((camera_transform, projection)) = camera_q.single() else { return; };
    let Ok(window) = window_q.single() else { return; };

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
        &terrain_assets,
        &resource_assets, 
        &buildings_assets,
        &terrain,
        &resources,
        &buildings
    );
}


fn calculate_load_area(
    config: &TerrainConfig,
    camera_transform: &Transform,
    projection: &Projection,
    window: &Window,
) -> (IVec2, IVec2) {
    let preload = config.preload_margin_chunks as i32;

    let (mut min, mut max) = camera_chunk_bounds(camera_transform, projection, window, &config);

    min -= preload;
    max += preload;
    let max_chunk_index = (config.terrain_size as i32 / config.chunk_size as i32) - 1;

    min = min.clamp(IVec2::ZERO, IVec2::splat(max_chunk_index));
    max = max.clamp(IVec2::ZERO, IVec2::splat(max_chunk_index));

    (min, max)

}


fn unload_distant_chunks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    config: &TerrainConfig,
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
            despawn_buildings_meshes(commands, meshes, buildings_meshes);
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

    for y in load_min.y..=load_max.y {
        for x in load_min.x..=load_max.x {
            let coord = IVec2::new(y, x);
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

fn load_missing_chunks(
    commands: &mut Commands,
    config: &TerrainConfig,
    meshes: &mut ResMut<Assets<Mesh>>,
    rendered: &mut Rendered,
    missing_chunks: Vec<IVec2>,
    terrain_assets: &TerrainAssets,
    resource_assets: &ResourceAssets,
    buildings_assets: &BuildingsAssets,
    terrain: &TerrainMap,
    resources: &ResourceMap,
    buildings: &BuildingMap,
) {
    let start_time = Instant::now();

    for coord in missing_chunks.into_iter() {

        println!("{:?}", coord);

        if start_time.elapsed().as_millis() > 5 {
            break;
        }

        if rendered.terrain.contains_key(&coord) {
            continue;
        }

        let terrain_meshes = spawn_terrain_meshes(commands, &config, meshes, coord, terrain_assets, terrain);

        let resource_meshes = spawn_resource_meshes(commands, &config, meshes, coord, resource_assets, resources);

        let building_meshes = spawn_buildings_meshes(commands, config, meshes, coord, buildings_assets, buildings);
        
        rendered.terrain.insert(coord, terrain_meshes);
        rendered.resources.insert(coord, resource_meshes);
        rendered.buildings.insert(coord, building_meshes);
    }
}