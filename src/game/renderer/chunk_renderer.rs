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

use crate::world::ui_and_graphic_resources::player_registry::PlayerRegistry;
use crate::world::{
    game_config::GameConfig,
    game_resources::{
        buildings_map::BuildingMap, fog_map::FogMap, resource_map::ResourceMap,
        terrain_map::TerrainMap, unit_map::UnitsMap,
    },
    ui_and_graphic_resources::player_registry::RacesAssetsRegistry,
};
use bevy::prelude::{
    AssetServer, Assets, Commands, IVec2, Mesh, Projection, Query, Res, ResMut, StandardMaterial,
    Transform, Window, With,
};
use bevy::window::PrimaryWindow;
use std::time::Instant;

pub fn stream_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut rendered: ResMut<Rendered>,
    assets_registry: Res<RacesAssetsRegistry>,
    player_registry: Res<PlayerRegistry>,
    config: Res<GameConfig>,
    terrain: Res<TerrainMap>,
    resources: Res<ResourceMap>,
    buildings: Res<BuildingMap>,
    fog_map: Res<FogMap>,
    units: Res<UnitsMap>,
    camera_q: Query<(&Transform, &Projection), With<MainCamera>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
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
        &assets_registry,
        &player_registry,
        &terrain,
        &resources,
        &buildings,
        &fog_map,
        &units,
        &asset_server,
        &mut materials,
    );
}

fn calculate_load_area(
    config: &GameConfig,
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
    config: &GameConfig,
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
        if let Some(fog_meshes) = rendered.fog.remove(&coord) {
            despawn_fog_meshes(commands, meshes, fog_meshes);
        }
        if let Some(units_meshes) = rendered.units.remove(&coord) {
            despawn_units_meshes(commands, meshes, units_meshes);
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

fn load_missing_chunks(
    commands: &mut Commands,
    config: &GameConfig,
    meshes: &mut ResMut<Assets<Mesh>>,
    rendered: &mut Rendered,
    missing_chunks: Vec<IVec2>,
    assets_registry: &RacesAssetsRegistry,
    player_registry: &PlayerRegistry,
    terrain: &TerrainMap,
    resources: &ResourceMap,
    buildings: &BuildingMap,
    fog: &FogMap,
    units: &UnitsMap,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) {
    let start_time = Instant::now();

    for coord in missing_chunks.into_iter() {
        if start_time.elapsed().as_millis() > 5 {
            break;
        }

        if rendered.terrain.contains_key(&coord) {
            continue;
        }

        let terrain_meshes = spawn_terrain_meshes(
            commands,
            &config,
            meshes,
            coord,
            assets_registry,
            terrain,
            player_registry,
        );

        let resource_meshes = spawn_resource_meshes(
            commands,
            &config,
            meshes,
            coord,
            assets_registry,
            resources,
            player_registry,
        );

        let building_meshes = spawn_buildings_meshes(
            commands,
            config,
            meshes,
            coord,
            assets_registry,
            buildings,
            player_registry,
        );

        let fog_meshes = spawn_fog_meshes(
            commands,
            config,
            meshes,
            materials,
            coord,
            asset_server,
            fog,
        );

        let units_meshes = spawn_units_meshes(
            commands,
            &config,
            meshes,
            coord,
            assets_registry,
            units,
            player_registry,
        );

        rendered.terrain.insert(coord, terrain_meshes);
        rendered.resources.insert(coord, resource_meshes);
        rendered.buildings.insert(coord, building_meshes);
        rendered.fog.insert(coord, fog_meshes);
        rendered.units.insert(coord, units_meshes);
    }
}
