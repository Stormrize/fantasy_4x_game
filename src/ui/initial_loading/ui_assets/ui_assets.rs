use crate::ui::initial_loading::ui_assets::RaceLoadingState;
use crate::world::components::PlayerRace;
use crate::world::ui_and_graphic_resources::data::ui_race_data::UiRaceData;
use crate::world::ui_and_graphic_resources::player_registry::RacesUiRegistry;
use bevy::asset::LoadState;
use bevy::prelude::{AssetServer, NextState, Res, ResMut};
use serde::Deserialize;
use smol_str::SmolStr;

// race paths
macro_rules! define_manifests {
    ($($path:literal),* $(,)?) => {
        // Генерируем массив путей
        const RACE_MANIFESTS: [&str; count_exprs!($($path)*)] = [
            $($path),*
        ];

        // Генерируем массив содержимого самих файлов
        const RACE_MANIFEST_CONTENTS: [&str; count_exprs!($($path)*)] = [
            $(include_str!($path)),*
        ];
    };
}

macro_rules! count_exprs {
    () => { 0 };
    ($head:literal $($tail:literal)*) => { 1 + count_exprs!($($tail)*) };
}

// Использование:
define_manifests!(
    "../../../../assets/core/manifests/fire_mages_identifier.ron",
    "../../../../assets/core/manifests/water_mages_identifier.ron",
    "../../../../assets/core/manifests/stone_mages_identifier.ron",
);

// raw data
#[derive(Deserialize, Debug, Clone)]
pub struct RawRaceData {
    pub id: String,
    pub description: String,
    pub ui_icon_path: String,
    pub visual_path: String,
    pub unit_data: String,
}

// loading systems
pub(super) fn start_loading_races(
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
            unit_data: data.unit_data,
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

pub(super) fn race_loading_ready(
    mut next_state: ResMut<NextState<RaceLoadingState>>,
    races_ui_registry: Res<RacesUiRegistry>,
    asset_server: Res<AssetServer>,
) {
    if all_races_loaded(&races_ui_registry, &asset_server) {
        println!("race loaded");
        next_state.set(RaceLoadingState::Ready);
    }
}
