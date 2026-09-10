use crate::ui::initial_loading::ui_assets::ui_assets::{race_loading_ready, start_loading_races};
use crate::ui::menus::AppState;
use crate::world::ui_and_graphic_resources::player_registry::RacesUiRegistry;
use bevy::prelude::{
    App, AppExtStates, IntoScheduleConfigs, OnEnter, Plugin, States, Update, in_state,
};

mod ui_assets;

pub(super) struct RaceLoadingPlugin;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(super) enum RaceLoadingState {
    #[default]
    Loading,
    Ready,
}

impl Plugin for RaceLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RaceLoadingState>()
            .init_resource::<RacesUiRegistry>()
            .add_systems(OnEnter(AppState::InitialAssetLoading), start_loading_races)
            .add_systems(
                Update,
                race_loading_ready
                    .chain()
                    .run_if(in_state(RaceLoadingState::Loading))
                    .run_if(in_state(AppState::InitialAssetLoading)),
            );
    }
}
