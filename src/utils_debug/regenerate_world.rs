use crate::core_rust::generation::map_generation_plugin::MapGenerationLoadingState;
use crate::world::game_configs::{ConfigType, GameModeConfig, GenerationConfig};
use bevy::prelude::*;

#[derive(Message)]
pub struct RegenerateWorldEvent;

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: MessageWriter<RegenerateWorldEvent>,
    mut config: ResMut<ConfigType>,
    mut next_state: ResMut<NextState<MapGenerationLoadingState>>,
) {
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        match &mut config.game_mode {
            GameModeConfig::Local(config) => {
                config.seed = config
                    .seed
                    .wrapping_mul(1_664_525)
                    .wrapping_add(1_013_904_223);
                events.write(RegenerateWorldEvent);
                next_state.set(MapGenerationLoadingState::Loading);
                info!("random seed: {}", config.seed);
            }
            _ => unreachable!(),
        }
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) {
        match &mut config.game_mode {
            GameModeConfig::Local(config) => {
                config.seed = config.seed.wrapping_add(1);
                events.write(RegenerateWorldEvent);
                next_state.set(MapGenerationLoadingState::Loading);
                info!("new seed: {}", config.seed);
            }
            _ => unreachable!(),
        }
    }

    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        match &mut config.game_mode {
            GameModeConfig::Local(config) => {
                config.seed = config.seed.wrapping_sub(1);
                events.write(RegenerateWorldEvent);
                next_state.set(MapGenerationLoadingState::Loading);
                info!("new seed: {}", config.seed);
            }
            _ => unreachable!(),
        }
    }
    if keyboard.just_pressed(KeyCode::KeyF) {}
}
