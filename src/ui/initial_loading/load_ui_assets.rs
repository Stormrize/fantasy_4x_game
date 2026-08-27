use crate::ui::menus::AppState;
use crate::world::{
    game_config::PlayerRace,
    ui_and_graphic_resources::{
        data::ui_race_data::{UiRaceData, RawRaceData},
        player_registry::RacesUiRegistry
    },
};
use bevy::asset::LoadState;
use bevy::prelude::{AssetServer, Assets, Commands, Handle, NextState, Res, ResMut, Resource};
use smol_str::SmolStr;

#[derive(Resource, Default, Debug)]
pub(super) struct InitialRawRaceDataHandles(Vec<Handle<RawRaceData>>);

pub(super) fn load_races(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path1 = "core/manifests/";
    let path2 = "identifier.ron";
    let fire_mages: Handle<RawRaceData> =
        asset_server.load(format!("{}fire_mages_{}", path1, path2));
    let water_mages: Handle<RawRaceData> =
        asset_server.load(format!("{}water_mages_{}", path1, path2));
    let stone_mages: Handle<RawRaceData> =
        asset_server.load(format!("{}stone_mages_{}", path1, path2));

    commands.insert_resource(InitialRawRaceDataHandles(vec![
        water_mages,
        fire_mages,
        stone_mages,
    ]));
}

pub(super) fn check_raw_handles(
    raw_handles: Res<InitialRawRaceDataHandles>,
    mut raw_assets: ResMut<Assets<RawRaceData>>,
    asset_server: Res<AssetServer>,
    mut initial_races_data: ResMut<RacesUiRegistry>,
) {
    for handle in &raw_handles.0 {
        if let Some(asset) = raw_assets.remove(handle) {
            let RawRaceData {
                id,
                description,
                ui_icon_path,
                visual_path,
            } = asset;

            let race_data = UiRaceData {
                id: SmolStr::from(id),
                description,
                ui_icon: asset_server.load(ui_icon_path),
                visual_path,
            };

            initial_races_data
                .0
                .insert(PlayerRace::new(race_data.id.clone()), race_data);
        }
    }
}

pub(super) fn set_new_state_if_all_loaded(
    races_ui_registry: Res<RacesUiRegistry>,
    asset_server: Res<AssetServer>,
    raw_handles: Res<InitialRawRaceDataHandles>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let expected_total_races = raw_handles.0.len();

    if races_ui_registry.0.len() < expected_total_races || expected_total_races == 0 {
        return;
    }

    let mut counter = 0;
    for (_, visual_race_data) in &races_ui_registry.0 {
        let handle = &visual_race_data.ui_icon;

        let ui_icon_loaded = match asset_server.get_load_state(handle.id()) {
            Some(LoadState::Loaded) => true,

            Some(LoadState::Failed(err)) => {
                println!("Failed to load asset {:?}: {:?}", handle.id(), err);
                false
            }

            Some(state) => {
                println!("Asset {:?} is not loaded yet: {:?}", handle.id(), state);
                false
            }

            None => {
                println!("Asset {:?} has no load state", handle.id());
                false
            }
        };

        if ui_icon_loaded {
            counter += 1;
        }
    }

    if counter == expected_total_races {
        println!("Loaded all handles initial_loading");
        next_state.set(AppState::MainMenu);
    }
}

pub(super) fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<InitialRawRaceDataHandles>();
}
