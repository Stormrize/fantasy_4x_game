use bevy::app::{App, Plugin, Update};
use bevy::asset::{Asset, AssetServer, Handle, LoadState};
use bevy::platform::collections::HashMap;
use bevy::prelude::{AppExtStates, Commands, IntoScheduleConfigs, NextState, OnEnter, Res, ResMut, Resource, States, TypePath, in_state, Assets};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::world::components::PlayerRace;
use crate::world::game_configs::ConfigType;
use crate::world::game_resources::data::unit::CombatUnitData;

#[derive(Resource, Debug, Default)]
pub struct RawRacesUnitsData(pub Vec<(PlayerRace, Handle<UnitsData>)>);
#[derive(Resource, Debug, Default)]
pub struct RacesUnitsData(pub HashMap<PlayerRace, UnitsData>);

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct UnitsData(pub [CombatUnitData; 4]);

fn start_loading_units_data(
    mut commands: Commands,
    config: Res<ConfigType>,
    races_ui_registry: Res<RacesUiRegistry>,
    asset_server: Res<AssetServer>,
) {
    let mut vec;

    match &*config {
        ConfigType::Local(config) => {
            let races = &config.races;
            vec = Vec::with_capacity(races.len());

            for race in races {
                let units_data_path = &races_ui_registry.ui_race_data_of(&race.1).unit_data_path;
                let units_data: Handle<UnitsData> = asset_server.load(units_data_path);
                vec.push((race.1.clone(), units_data));
            }
        }
        ConfigType::Multiplayer(config) => {
            let allowed_races = &config.allowed_races;
            vec = Vec::with_capacity(allowed_races.len());

            for race in allowed_races {
                let units_data_path = &races_ui_registry.ui_race_data_of(&race).unit_data_path;
                let units_data: Handle<UnitsData> = asset_server.load(units_data_path);
                vec.push((race.clone(), units_data));
            }
        }
    }
    commands.insert_resource(RawRacesUnitsData(vec));
}

fn all_units_data_loaded(
    raw_races_units_data: &RawRacesUnitsData,
    asset_server: &AssetServer,
) -> bool {
    raw_races_units_data.0.iter().all(|(_, handle)| {
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
    mut raw_races_units_data: ResMut<RawRacesUnitsData>,
    mut raw_units_assets: ResMut<Assets<UnitsData>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<UnitsDataLoadingState>>,
    mut commands: Commands,
) {
    if all_units_data_loaded(&raw_races_units_data, &asset_server) {
        let mut hashmap = HashMap::with_capacity(raw_races_units_data.0.len());
        let vec = std::mem::take(&mut raw_races_units_data.0);
        
        for (player, handle) in vec {
            let Some(handle) = raw_units_assets.remove(&handle) else {
                return;
            };
            hashmap.insert(player, handle);
        }
        commands.insert_resource(RacesUnitsData(hashmap));
        commands.remove_resource::<RawRacesUnitsData>();
        next_state.set(UnitsDataLoadingState::Ready);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UnitsDataLoadingState {
    Loading,
    #[default]
    Ready,
}

pub struct UnitsDataLoadingPlugin;

impl Plugin for UnitsDataLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UnitsDataLoadingState>()
            .init_resource::<RawRacesUnitsData>()
            .init_resource::<RacesUnitsData>()
            .add_plugins(RonAssetPlugin::<UnitsData>::new(&["ron"]))
            .add_systems(
                OnEnter(UnitsDataLoadingState::Loading),
                start_loading_units_data,
            )
            .add_systems(
                Update,
                units_data_ready.run_if(in_state(UnitsDataLoadingState::Loading)),
            );
    }
}
