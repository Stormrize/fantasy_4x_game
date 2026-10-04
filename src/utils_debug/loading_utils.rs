use bevy::asset::{AssetServer, Handle, LoadState};
use bevy::platform::collections::HashMap;
use crate::io::in_game_loading::buildings_data_loader_plugin::BuildingsData;
use crate::io::in_game_loading::game_assets_loader_plugin::RawRaceAssets;
use crate::io::in_game_loading::units_data_loader_plugin::UnitsData;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::world::components::{Id, PlayerRace};

pub fn load_raw_game_assets(
    races: &Vec<(Id, PlayerRace)>,
    races_ui_registry: &RacesUiRegistry,
    asset_server: &AssetServer,
    vec: &mut Vec<(PlayerRace, Handle<RawRaceAssets>)>,
) {
    for race in races {
        let visual_path = &races_ui_registry.ui_race_data_of(&race.1).visual_path;
        let raw_visual_data: Handle<RawRaceAssets> = asset_server.load(visual_path);
        vec.push((race.1.clone(), raw_visual_data));
    }
}

pub fn load_buildings_data(
    races: &Vec<(Id, PlayerRace)>,
    races_ui_registry: &RacesUiRegistry,
    asset_server: &AssetServer,
    vec: &mut Vec<(PlayerRace, Handle<BuildingsData>)>,
) {
    for race in races {
        let building_data_path = &races_ui_registry.ui_race_data_of(&race.1).building_data_path;
        let buildings_data: Handle<BuildingsData> = asset_server.load(building_data_path);
        vec.push((race.1.clone(), buildings_data));
    }
}

pub fn load_units_data(
    races: &Vec<(Id, PlayerRace)>,
    races_ui_registry: &RacesUiRegistry,
    asset_server: &AssetServer,
    vec: &mut Vec<(PlayerRace, Handle<UnitsData>)>,
) {
    for race in races {
        let unit_data_path = &races_ui_registry.ui_race_data_of(&race.1).unit_data_path;
        let units_data: Handle<UnitsData> = asset_server.load(unit_data_path);
        vec.push((race.1.clone(), units_data));
    }
}

pub fn all_data_loaded<T: bevy::asset::Asset>(
    data: &Vec<(PlayerRace, Handle<T>)>,
    asset_server: &AssetServer,
) -> bool {
    data.iter().all(|(_, handle)| {
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

pub fn all_values_loaded<K, T: bevy::asset::Asset>(
    handles: &HashMap<K, Handle<T>>,
    asset_server: &AssetServer,
) -> bool {
    handles
        .values()
        .all(|handle| match asset_server.get_load_state(handle.id()) {
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
        })
}