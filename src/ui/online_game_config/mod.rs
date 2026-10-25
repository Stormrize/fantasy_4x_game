use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::menus::AppState;
use crate::world::components::{GameMode, Id, MapShape, MapSize, PlayerType};
use crate::world::game_configs::{ConfigType, GameModeConfig, OnlinePlayerConfig, PlayerConfig};
use crate::world::turn_manager::TurnManager;
use bevy::app::App;
use bevy::prelude::{Commands, NextState, OnEnter, Plugin, Res, ResMut};

pub struct OnlineGameConfig;

impl Plugin for OnlineGameConfig {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::OnlineGameSetupMenu), build_online_config);
    }
}

pub fn build_online_config(
    mut commands: Commands,
    races_ui_registry: Res<RacesUiRegistry>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // todo change that player can choose what races are ok

    let races = races_ui_registry
        .0
        .keys()
        .enumerate()
        .map(|(index, race)| (Id(index as u16), race.clone()))
        .collect();

    commands.insert_resource(TurnManager::new(
        Id(0),
        PlayerType::Human,
        3,
        GameMode::Online,
    ));

    let online_config = OnlinePlayerConfig { races };
    let game_mode = GameModeConfig::Multiplayer(online_config);
    let player_config = PlayerConfig::new(MapSize::Medium, MapShape::Jagged, Id(0));
    let config_type = ConfigType {
        player_config,
        game_mode,
    };

    commands.insert_resource(config_type);

    next_state.set(AppState::InGameAssetLoading);
}
