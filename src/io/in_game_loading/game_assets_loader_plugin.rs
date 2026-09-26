use bevy::asset::LoadState;
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    App, AppExtStates, Asset, AssetServer, Assets, Color, Commands, Handle, IntoScheduleConfigs,
    NextState, OnEnter, Plugin, Res, ResMut, Resource, StandardMaterial, States, TypePath, Update,
    default, in_state,
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::world::game_configs::ConfigType;
use crate::world::ui_and_graphic_resources::data::visual_race_data::RaceAssets;
use crate::world::{components::PlayerRace, game_configs::GameMode};

// raw race assets
#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct RawRaceAssets {
    //biome
    pub water: String,
    pub small_biome: String,
    pub medium_biome: String,
    pub large_biome: String,

    //resource
    pub small_resource: String,
    pub medium_resource: String,
    pub large_resource: String,

    //building
    pub capital: String,
    pub captured_city: String,
    pub barbarian_city: String,
    pub medium_building: String,
    pub large_building: String,
    pub spawning_building: String,
    pub house: String,
    pub research_center: String,
    pub unique_race_building: String,
}
#[derive(Resource, Default, Debug)]
struct RawRacesAssetsData(pub Vec<(PlayerRace, Handle<RawRaceAssets>)>);

// registry
#[derive(Resource, Default, Debug)]
pub struct RacesAssetsRegistry(pub HashMap<PlayerRace, RaceAssets>);

// loading systems
fn start_loading_game_assets(
    mut commands: Commands,
    config: Res<ConfigType>,
    races_data: Res<RacesUiRegistry>,
    asset_server: Res<AssetServer>,
) {
    let mut vec;

    match &config.game_mode {
        GameMode::Local(config) => {
            let races = &config.races;
            vec = Vec::with_capacity(races.len());

            for race in races {
                let visual_path = &races_data.ui_race_data_of(&race.1).visual_path;
                let raw_visual_data: Handle<RawRaceAssets> = asset_server.load(visual_path);
                vec.push((race.1.clone(), raw_visual_data));
            }
        }
        GameMode::Multiplayer(config) => {
            let allowed_races = &config.allowed_races;
            vec = Vec::with_capacity(allowed_races.len());

            for race in allowed_races {
                let visual_path = &races_data.ui_race_data_of(&race).visual_path;
                let raw_visual_data: Handle<RawRaceAssets> = asset_server.load(visual_path);
                vec.push((race.clone(), raw_visual_data));
            }
        }
    }
    println!("started loading game assets");
    commands.insert_resource(RawRacesAssetsData(vec));
}

//---------------------------------------------------------------------------------------------------------------------------//

fn all_raw_assets_loaded(raw_handles: &RawRacesAssetsData, asset_server: &AssetServer) -> bool {
    raw_handles.0.iter().all(
        |(_, handle)| match asset_server.get_load_state(handle.id()) {
            Some(LoadState::Loaded) => true,

            Some(LoadState::Failed(err)) => {
                eprintln!("Failed to load asset {:?}: {:?}", handle.id(), err);
                false
            }

            Some(state) => {
                eprintln!("Asset {:?} is not loaded yet: {:?}", handle.id(), state);
                false
            }

            None => {
                eprintln!("Asset {:?} has no load state", handle.id());
                false
            }
        },
    )
}

fn load_assets(
    raw_handles: &RawRacesAssetsData,
    raw_race_assets: &mut Assets<RawRaceAssets>,
    assets_registry: &mut RacesAssetsRegistry,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) {
    for handle in raw_handles.0.iter() {
        let race_asset = raw_race_assets.remove(&handle.1).unwrap();
        let fallback_visual_race_data = RaceAssets {
            biomes: [
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.water)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.small_biome)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.medium_biome)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.large_biome)),
                    ..default()
                }),
            ],

            resources: [
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.small_resource)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.medium_resource)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.large_resource)),
                    ..default()
                }),
            ],

            buildings: [
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.capital)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.captured_city)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.barbarian_city)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.medium_building)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.large_building)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.spawning_building)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.house)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.research_center)),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color: parse_color(parse_string(&race_asset.unique_race_building)),
                    ..default()
                }),
            ],
        };
        assets_registry
            .0
            .insert(handle.0.clone(), fallback_visual_race_data);
    }
}

fn raw_assets_ready(
    raw_handles: Res<RawRacesAssetsData>,
    mut raw_race_assets: ResMut<Assets<RawRaceAssets>>,
    mut assets_registry: ResMut<RacesAssetsRegistry>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameAssetsLoadingState>>,
    mut commands: Commands,
) {
    if all_raw_assets_loaded(&raw_handles, &asset_server) {
        load_assets(
            &raw_handles,
            &mut raw_race_assets,
            &mut assets_registry,
            &asset_server,
            &mut materials,
        );
        commands.remove_resource::<RawRacesAssetsData>();
        next_state.set(GameAssetsLoadingState::AssetsLoading);
    }
}

//---------------------------------------------------------------------------------------------------------------------------//
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
//---------------------------------------------------------------------------------------------------------------------------//

fn all_fields_loaded(data: &RaceAssets, asset_server: &AssetServer) -> bool {
    data.handles().all(|handle| {
        match asset_server.get_load_state(handle.id()) {
            Some(LoadState::Loaded) => true,

            Some(LoadState::Failed(err)) => {
                eprintln!("Failed to load asset {:?}: {:?}", handle.id(), err);
                false //false
            }

            Some(state) => {
                eprintln!("Asset {:?} is not loaded yet: {:?}", handle.id(), state);
                false //false
            }

            None => {
                eprintln!("Asset {:?} has no load state", handle.id());
                true //false
            }
        }
    })
}

fn all_game_assets_loaded(
    races_assets_registry: &RacesAssetsRegistry,
    asset_server: &AssetServer,
) -> bool {
    races_assets_registry
        .0
        .values()
        .all(|race_assets| all_fields_loaded(race_assets, &asset_server))
}

fn game_assets_ready(
    assets_registry: Res<RacesAssetsRegistry>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameAssetsLoadingState>>,
) {
    if all_game_assets_loaded(&assets_registry, &asset_server) {
        println!("Game assets loaded");
        next_state.set(GameAssetsLoadingState::Ready);
    }
}

// states
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameAssetsLoadingState {
    #[default]
    Disabled,
    Loading,
    AssetsLoading,
    Ready,
}

pub struct GameAssetsLoadingPlugin;

impl Plugin for GameAssetsLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameAssetsLoadingState>()
            .init_resource::<RawRacesAssetsData>()
            .init_resource::<RacesAssetsRegistry>()
            .add_plugins(RonAssetPlugin::<RawRaceAssets>::new(&["ron"]))
            .add_systems(
                OnEnter(GameAssetsLoadingState::Loading),
                start_loading_game_assets,
            )
            .add_systems(
                Update,
                (
                    raw_assets_ready.run_if(in_state(GameAssetsLoadingState::Loading)),
                    game_assets_ready.run_if(in_state(GameAssetsLoadingState::AssetsLoading)),
                ),
            );
    }
}
