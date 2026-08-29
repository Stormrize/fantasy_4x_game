use crate::world::game_configs::ConfigType;
use crate::world::ui_and_graphic_resources::player_registry::{Id, PlayerRegistry};
use crate::world::{
    game_config::PlayerRace,
    ui_and_graphic_resources::{
        data::visual_race_data::{RaceAssets, RawRaceAssets},
        player_registry::{RacesAssetsRegistry, RacesUiRegistry},
    },
};
use bevy::color::Color;
use bevy::prelude::{
    AssetServer, Assets, Commands, Handle, Res, ResMut, Resource, StandardMaterial,
};
use bevy::utils::default;

#[derive(Resource, Default, Debug)]
pub(super) struct RawVisualRaceHandles(pub Vec<(PlayerRace, Handle<RawRaceAssets>)>);

pub(super) fn load_assets(
    mut commands: Commands,
    config: Res<ConfigType>,
    races_data: Res<RacesUiRegistry>,
    asset_server: Res<AssetServer>,
) {
    let mut vec;

    match &*config {
        ConfigType::Local(config) => {
            let races = &config.races;
            vec = Vec::with_capacity(races.len());

            for race in races {
                let visual_path = &races_data.ui_race_data_of(&race.1).visual_path;
                let raw_visual_data: Handle<RawRaceAssets> = asset_server.load(visual_path);
                vec.push((race.1.clone(), raw_visual_data));
            }
        }
        ConfigType::Multiplayer(config) => {
            let allowed_races = &config.allowed_races;
            vec = Vec::with_capacity(allowed_races.len());

            for race in allowed_races {
                let visual_path = &races_data.ui_race_data_of(&race).visual_path;
                let raw_visual_data: Handle<RawRaceAssets> = asset_server.load(visual_path);
                vec.push((race.clone(), raw_visual_data));
            }
        }
    }

    commands.insert_resource(RawVisualRaceHandles(vec));
}

pub(super) fn check_raw_handles(
    raw_handles: Res<RawVisualRaceHandles>,
    mut raw_assets: ResMut<Assets<RawRaceAssets>>,
    mut assets_registry: ResMut<RacesAssetsRegistry>,
    mut player_registry: ResMut<PlayerRegistry>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut id_counter = 0;
    for handle in &raw_handles.0 {
        if let Some(asset) = raw_assets.remove(&handle.1) {
            let RawRaceAssets {
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

            let fallback_visual_race_data = RaceAssets {
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

            assets_registry
                .0
                .insert(handle.0.clone(), fallback_visual_race_data);

            //note под вопросиком
            player_registry
                .0
                .insert(Id::LocalPlayerId(id_counter), handle.0.clone());
            id_counter += 1;
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
