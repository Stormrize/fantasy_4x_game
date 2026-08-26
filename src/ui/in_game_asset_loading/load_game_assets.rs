use crate::ui::menus::AppState;
use crate::world::{
    game_config::{GameConfig, PlayerRace},
    ui_and_graphic_resources::{
        data::visual_race_data::{RawVisualRaceData, VisualRaceData},
        ui_race_dataset::InitialRacesData,
        visual_race_dataset::VisualRacesData,
    },
};
use bevy::asset::LoadState;
use bevy::color::Color;
use bevy::prelude::{
    AssetServer, Assets, Commands, Handle, NextState, Res, ResMut, Resource, StandardMaterial,
};
use bevy::utils::default;

#[derive(Resource, Default, Debug)]
pub(super) struct RawVisualRaceHandles(pub Vec<(PlayerRace, Handle<RawVisualRaceData>)>);

pub(super) fn load_assets(
    mut commands: Commands,
    config: Res<GameConfig>,
    races_data: Res<InitialRacesData>,
    asset_server: Res<AssetServer>,
) {
    let players = &config.current_players;
    let visual_paths = &races_data.0;

    let mut vec = Vec::new();

    for player in players {
        let visual_path = &visual_paths.get(&player.1).unwrap().visual_path;

        let raw_visual_data: Handle<RawVisualRaceData> = asset_server.load(visual_path);

        vec.push((player.1.clone(), raw_visual_data));
    }
    commands.insert_resource(RawVisualRaceHandles(vec));
}

pub(super) fn check_raw_handles(
    raw_handles: Res<RawVisualRaceHandles>,
    mut raw_assets: ResMut<Assets<RawVisualRaceData>>,
    asset_server: Res<AssetServer>,
    mut visual_races_data: ResMut<VisualRacesData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for handle in &raw_handles.0 {
        if let Some(asset) = raw_assets.remove(&handle.1) {
            let RawVisualRaceData {
                water,
                small_biome,
                medium_biome,
                large_biome,

                //resource
                small_resource,
                medium_resource,
                large_resource,

                //building
                capital,
                empty_cities,
                empty_cities_colonized,
                medium_building,
                large_building,
                spawning_building,
                house,
                research_center,
            } = asset;

            // let visual_race_data = VisualRaceData {
            //     small_biome: asset_server.load(small_biome),
            //     medium_biome: asset_server.load(medium_biome),
            //     large_biome: asset_server.load(large_biome),
            //
            //     //resource
            //     small_resource: asset_server.load(small_resource),
            //     medium_resource: asset_server.load(medium_resource),
            //     large_resource: asset_server.load(large_resource),
            //
            //     //building
            //     capital: asset_server.load(capital),
            //     empty_cities: asset_server.load(empty_cities),
            //     empty_cities_colonized: asset_server.load(empty_cities_colonized),
            //     medium_building: asset_server.load(medium_building),
            //     large_building: asset_server.load(large_building),
            //     spawning_building: asset_server.load(spawning_building),
            //     house: asset_server.load(house),
            //     research_center: asset_server.load(research_center),
            // };

            let fallback_visual_race_data = VisualRaceData {
                biomes: [
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&water)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&small_biome)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&medium_biome)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&large_biome)),
                        ..default()
                    }),
                ],

                resources: [
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&small_resource)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&medium_resource)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&large_resource)),
                        ..default()
                    }),
                ],

                buildings: [
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&capital)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&empty_cities)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&empty_cities_colonized)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&medium_building)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&large_building)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&spawning_building)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&house)),
                        ..default()
                    }),
                    materials.add(StandardMaterial {
                        base_color: parse_color(parse_string(&research_center)),
                        ..default()
                    }),
                ],
            };

            visual_races_data
                .0
                .insert(handle.0.clone(), fallback_visual_race_data);
        }
    }
}

fn parse_string(color: &str) -> Option<Color> {
    let mut color = color.split(", ");

    let r = color.next()?.trim().parse::<f32>().ok()?;
    let g = color.next()?.trim().parse::<f32>().ok()?;
    let b = color.next()?.trim().parse::<f32>().ok()?;

    Some(Color::srgb(r, g, b))
}

fn parse_color(color: Option<Color>) -> Color {
    if color.is_none() {
        Color::srgb(0.0, 0.0, 0.0)
    } else {
        color.unwrap()
    }
}
pub(super) fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<RawVisualRaceHandles>();
}
