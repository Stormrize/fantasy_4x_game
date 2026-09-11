use crate::ui::menus::AppState;
use crate::world::components::PlayerRace;
use crate::world::game_configs::ConfigType;
use crate::world::game_resources::data::unit::UnitData;
use crate::world::ui_and_graphic_resources::player_registry::RacesUiRegistry;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{Asset, AssetServer, Handle, LoadState};
use bevy::prelude::{
    AppExtStates, Commands, IntoScheduleConfigs, NextState, OnEnter, Res, ResMut, Resource, States,
    TypePath, in_state,
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

#[derive(Resource, Debug, Default)]
pub(super) struct RawRacesUnitsData(Vec<(PlayerRace, Handle<UnitsData>)>);

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct UnitsData(pub [UnitData; 4]);

pub(super) fn start_loading_units_data(
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

pub(super) fn all_units_data_loaded(
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

pub fn units_data_ready(
    raw_races_units_data: Res<RawRacesUnitsData>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<UnitsDataLoadingState>>,
) {
    if all_units_data_loaded(&raw_races_units_data, &asset_server) {
        commands.remove_resource::<RawRacesUnitsData>();
        next_state.set(UnitsDataLoadingState::Ready);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(super) enum UnitsDataLoadingState {
    #[default]
    Loading,
    Ready,
}

pub struct UnitsDataLoadingPlugin;

impl Plugin for UnitsDataLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UnitsDataLoadingState>()
            .init_resource::<RawRacesUnitsData>()
            .add_plugins(RonAssetPlugin::<UnitsData>::new(&["ron"]))
            .add_systems(
                OnEnter(AppState::InGameAssetLoading),
                start_loading_units_data,
            )
            .add_systems(
                Update,
                units_data_ready
                    .run_if(in_state(UnitsDataLoadingState::Loading))
                    .run_if(in_state(AppState::InGameAssetLoading)),
            );
    }
}
