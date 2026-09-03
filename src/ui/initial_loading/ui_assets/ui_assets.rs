use crate::ui::initial_loading::ui_assets::RaceLoadingState;
use crate::world::game_config::PlayerRace;
use crate::world::ui_and_graphic_resources::data::ui_race_data::UiRaceData;
use crate::world::ui_and_graphic_resources::player_registry::RacesUiRegistry;
use bevy::asset::{Asset, LoadState};
use bevy::prelude::{
    AssetServer, Assets, Commands, Handle, NextState, OnEnter, Plugin, Res, ResMut, Resource,
    States, TypePath, Update, in_state,
};
use serde::Deserialize;
use smol_str::SmolStr;

// race paths
const RACE_MANIFESTS: [&str; 3] = [
    "core/manifests/fire_mages_identifier.ron",
    "core/manifests/water_mages_identifier.ron",
    "core/manifests/stone_mages_identifier.ron",
];

// raw data
#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct RawRaceData {
    pub id: String,
    pub description: String,
    pub ui_icon_path: String,
    pub visual_path: String,
}

#[derive(Resource, Default, Debug)]
pub(super) struct RawRaceDataHandles(Vec<Handle<RawRaceData>>);

// loading systems
pub(super) fn start_loading_races(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handles: Vec<Handle<RawRaceData>> = RACE_MANIFESTS
        .into_iter()
        .map(|path| asset_server.load(path))
        .collect();
    commands.insert_resource(RawRaceDataHandles(handles));
}

pub(super) fn check_raw_handles(
    raw_handles: Res<RawRaceDataHandles>,
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
fn all_races_loaded(
    registry: &RacesUiRegistry,
    asset_server: &AssetServer,
    handles: &RawRaceDataHandles,
) -> bool {
    let expected = handles.0.len();

    if expected == 0 || registry.0.len() < expected {
        return false;
    }

    registry.0.values().all(
        |race| match asset_server.get_load_state(race.ui_icon.id()) {
            Some(LoadState::Loaded) => true,
            Some(LoadState::Failed(err)) => {
                eprintln!("Failed to load race icon {:?}: {err:?}", race.ui_icon.id());
                false
            }
            _ => false,
        },
    )
}

pub(super) fn race_loading_ready(
    mut commands: Commands,
    mut next_state: ResMut<NextState<RaceLoadingState>>,
    registry: Res<RacesUiRegistry>,
    asset_server: Res<AssetServer>,
    handles: Res<RawRaceDataHandles>,
) {
    if all_races_loaded(&registry, &asset_server, &handles) {
        println!("race loaded");
        commands.remove_resource::<RawRaceDataHandles>();
        next_state.set(RaceLoadingState::Ready);
    }
}
