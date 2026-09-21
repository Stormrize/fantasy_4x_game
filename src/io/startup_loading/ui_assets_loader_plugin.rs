use bevy::asset::LoadState;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    App, AppExtStates, AssetServer, NextState, OnEnter, Plugin, Res, ResMut, Resource, States,
    Update, in_state,
};
use serde::Deserialize;
use smol_str::SmolStr;

use crate::world::components::PlayerRace;
use crate::world::ui_and_graphic_resources::data::ui_race_data::UiRaceData;

// race paths
macro_rules! define_manifests {
    ($($path:literal),* $(,)?) => {
        const RACE_MANIFEST_CONTENTS: [&str; count_exprs!($($path)*)] = [
            $(include_str!($path)),*
        ];
    };
}

macro_rules! count_exprs {
    () => { 0 };
    ($head:literal $($tail:literal)*) => { 1 + count_exprs!($($tail)*) };
}

define_manifests!(
    "../../../assets/core/manifests/fire_mages_identifier.ron",
    "../../../assets/core/manifests/water_mages_identifier.ron",
    "../../../assets/core/manifests/stone_mages_identifier.ron",
);

// raw data
#[derive(Deserialize, Debug, Clone)]
pub struct RawRaceData {
    pub id: String,
    pub description: String,
    pub ui_icon_path: String,
    pub visual_path: String,
    pub unit_data: String,
    pub buildings_data: String,
}

// registry
#[derive(Resource, Default, Debug)]
pub struct RacesUiRegistry(pub HashMap<PlayerRace, UiRaceData>);

impl RacesUiRegistry {
    pub fn ui_race_data_of(&self, player: &PlayerRace) -> &UiRaceData {
        &self.0[player]
    }
}

// loading systems
fn start_loading_races(
    asset_server: Res<AssetServer>,
    mut races_ui_registry: ResMut<RacesUiRegistry>,
) {
    for content in RACE_MANIFEST_CONTENTS {
        let data = ron::from_str::<RawRaceData>(content).unwrap();

        let race_data = UiRaceData {
            id: SmolStr::from(data.id),
            description: data.description,
            ui_icon: asset_server.load(data.ui_icon_path),
            visual_path: data.visual_path,
            unit_data_path: data.unit_data,
            building_data_path: data.buildings_data,
        };

        races_ui_registry
            .0
            .insert(PlayerRace::new(race_data.id.clone()), race_data);
    }
}

fn all_races_loaded(registry: &RacesUiRegistry, asset_server: &AssetServer) -> bool {
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

fn race_loading_ready(
    mut next_state: ResMut<NextState<RaceLoadingState>>,
    races_ui_registry: Res<RacesUiRegistry>,
    asset_server: Res<AssetServer>,
) {
    if all_races_loaded(&races_ui_registry, &asset_server) {
        println!("race loaded");
        next_state.set(RaceLoadingState::Ready);
    }
}

// states
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RaceLoadingState {
    Loading,
    #[default]
    Ready,
}

// plugin
pub struct RaceLoadingPlugin;

impl Plugin for RaceLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RaceLoadingState>()
            .init_resource::<RacesUiRegistry>()
            .add_systems(OnEnter(RaceLoadingState::Ready), start_loading_races)
            .add_systems(
                Update,
                race_loading_ready
                    .chain()
                    .run_if(in_state(RaceLoadingState::Loading)),
            );
    }
}
