use bevy::app::{App, Plugin, Update};
use bevy::asset::{Asset, AssetServer, Handle, LoadState};
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    AppExtStates, Commands, IntoScheduleConfigs, NextState, OnEnter, Res, ResMut, Resource, States,
    TypePath, in_state,
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::world::components::PlayerRace;
use crate::world::game_configs::{ConfigType, GameMode};
use crate::world::game_resources::data::building::BuildingData;

#[derive(Resource, Debug, Default)]
pub struct RawRacesBuildingsData(pub Vec<(PlayerRace, Handle<BuildingsData>)>);

pub struct RacesBuildingsData(pub HashMap<PlayerRace, BuildingsData>);
#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct BuildingsData(pub Vec<BuildingData>);

fn start_loading_buildings_data(
    mut commands: Commands,
    config: Res<ConfigType>,
    races_ui_registry: Res<RacesUiRegistry>,
    asset_server: Res<AssetServer>,
) {
    let mut vec;

    match &config.game_mode {
        GameMode::Local(config) => {
            let races = &config.races;
            vec = Vec::with_capacity(races.len());

            for race in races {
                let building_data_path = &races_ui_registry
                    .ui_race_data_of(&race.1)
                    .building_data_path;
                let units_data: Handle<BuildingsData> = asset_server.load(building_data_path);
                vec.push((race.1.clone(), units_data));
            }
        }
        GameMode::Multiplayer(config) => {
            let allowed_races = &config.allowed_races;
            vec = Vec::with_capacity(allowed_races.len());

            for race in allowed_races {
                let building_data_path =
                    &races_ui_registry.ui_race_data_of(&race).building_data_path;
                let units_data: Handle<BuildingsData> = asset_server.load(building_data_path);
                vec.push((race.clone(), units_data));
            }
        }
    }
    commands.insert_resource(RawRacesBuildingsData(vec));
}

fn all_units_data_loaded(
    raw_races_buildings_data: &RawRacesBuildingsData,
    asset_server: &AssetServer,
) -> bool {
    raw_races_buildings_data.0.iter().all(|(_, handle)| {
        match asset_server.get_load_state(handle.id()) {
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
        }
    })
}

fn units_data_ready(
    raw_races_buildings_data: Res<RawRacesBuildingsData>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<BuildingsDataLoadingState>>,
) {
    if all_units_data_loaded(&raw_races_buildings_data, &asset_server) {
        next_state.set(BuildingsDataLoadingState::Ready);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BuildingsDataLoadingState {
    Loading,
    #[default]
    Ready,
}

pub struct BuildingsDataLoadingPlugin;

impl Plugin for BuildingsDataLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BuildingsDataLoadingState>()
            .init_resource::<RawRacesBuildingsData>()
            .add_plugins(RonAssetPlugin::<BuildingsData>::new(&["ron"]))
            .add_systems(
                OnEnter(BuildingsDataLoadingState::Loading),
                start_loading_buildings_data,
            )
            .add_systems(
                Update,
                units_data_ready.run_if(in_state(BuildingsDataLoadingState::Loading)),
            );
    }
}
