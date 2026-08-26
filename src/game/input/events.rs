use crate::world::game_config::GameConfig;
use bevy::prelude::*;

#[derive(Message)]
pub struct RegenerateWorldEvent;

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: MessageWriter<RegenerateWorldEvent>,
    mut config: ResMut<GameConfig>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        config.seed = config
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        events.write(RegenerateWorldEvent);
        info!("Случайный seed: {}", config.seed);
    }

    if keyboard.just_pressed(KeyCode::Space) {
        config.seed = config.seed.wrapping_add(1);
        events.write(RegenerateWorldEvent);
        info!("Новый seed: {}", config.seed);
    }

    if keyboard.just_pressed(KeyCode::KeyZ) {
        config.seed = config.seed.wrapping_sub(1);
        events.write(RegenerateWorldEvent);
        info!("Прошлый seed: {}", config.seed);
    }
}
