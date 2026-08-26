use crate::ui::in_game_asset_loading::load_game_assets::RawVisualRaceHandles;
use crate::ui::in_game_asset_loading::load_game_map::MapGenerationTask;
use crate::ui::menus::AppState;
use crate::world::game_config::{GameConfig, GameMode};
use crate::world::ui_and_graphic_resources::data::visual_race_data::VisualRaceData;
use crate::world::ui_and_graphic_resources::visual_race_dataset::VisualRacesData;
use bevy::asset::{AssetServer, LoadState};
use bevy::prelude::{Commands, NextState, Res, ResMut};
use bevy::tasks::futures_lite::future;

fn poll_map_generation(mut commands: Commands, task: &mut MapGenerationTask) -> bool {
    if let Some(generated) = future::block_on(future::poll_once(&mut task.task)) {
        commands.insert_resource(generated.terrain);
        commands.insert_resource(generated.resources);
        commands.insert_resource(generated.cities);
        commands.insert_resource(generated.units);
        commands.insert_resource(generated.fog);

        commands.remove_resource::<MapGenerationTask>();
        true
    } else {
        false
    }
}

pub(super) fn set_new_state_if_all_loaded(
    visual_races_data: Res<VisualRacesData>,
    raw_handles: Res<RawVisualRaceHandles>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    commands: Commands,
    task: Option<ResMut<MapGenerationTask>>,
    config: Res<GameConfig>,
) {
    let Some(mut task) = task else {
        return;
    };

    let expected_current_races = raw_handles.0.len();

    if visual_races_data.0.len() < expected_current_races || visual_races_data.0.len() == 0 {
        return;
    }

    let mut counter = 0;

    for (_, visual_race_data) in &visual_races_data.0 {
        let loaded = all_fields_loaded(visual_race_data, &asset_server);

        if loaded {
            counter += 1;
        }
    }

    if config.game_mode == GameMode::Local {
        if counter == expected_current_races && poll_map_generation(commands, &mut *task) {
            println!("Loaded all handles in_game_asset_loading");
            next_state.set(AppState::InGameRenderer);
        }
    } else {
        if counter == expected_current_races
        /* && get_map_from_serveer*/
        {
            println!("Loaded all handles in_game_asset_loading");
            next_state.set(AppState::InGameRenderer);
        }
    }
}

fn all_fields_loaded(data: &VisualRaceData, asset_server: &AssetServer) -> bool {
    let mut all_loaded = true;

    for handle in data.handles() {
        match asset_server.get_load_state(handle.id()) {
            Some(LoadState::Loaded) => {}

            Some(LoadState::Failed(err)) => {
                eprintln!("Failed to load asset {:?}: {:?}", handle.id(), err);
                all_loaded = false; //false
            }

            Some(state) => {
                eprintln!("Asset {:?} is not loaded yet: {:?}", handle.id(), state);
                all_loaded = false; //false
            }

            None => {
                eprintln!("Asset {:?} has no load state", handle.id());
                all_loaded = true; //false
            }
        }
    }

    all_loaded
}
