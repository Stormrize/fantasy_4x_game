use bevy::prelude::*;
use noise::Perlin;
use crate::terrain::generation::cities::cities_generation::BuildingMap;
use crate::terrain::generation::resources::resource_generation::ResourceMap;
use crate::terrain::generation::terrain::terrain_generation::{PerlinPlaceholder, TerrainMap};
use crate::terrain::terrain_config::TerrainConfig;

#[derive(Message)] 
pub struct RegenerateWorldEvent;

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<TerrainConfig>,
    mut events: MessageWriter<RegenerateWorldEvent>,
    mut perlin: ResMut<PerlinPlaceholder>,
    mut terrain: ResMut<TerrainMap>,
    mut resources: ResMut<ResourceMap>,
    mut buildings: ResMut<BuildingMap>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) { config.seed = config
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);

        perlin.perlin_main = Perlin::new(config.seed);
        perlin.perlin_detail = Perlin::new(config.seed.wrapping_add(1337));
        events.write(RegenerateWorldEvent);
        info!("Новый seed: {}", config.seed);
    }

    if keyboard.just_pressed(KeyCode::Space) {
        config.seed = config.seed.wrapping_add(1);
        events.write(RegenerateWorldEvent);
        perlin.perlin_main = Perlin::new(config.seed);
        perlin.perlin_detail = Perlin::new(config.seed.wrapping_add(1337));
        *terrain = TerrainMap::new(&config, &perlin);
        *resources = ResourceMap::new(&terrain, &config);
        *buildings = BuildingMap::new(&config, &terrain, &resources);
        info!("Случайный seed: {}", config.seed);
    }

    if keyboard.just_pressed(KeyCode::KeyZ) {
        config.seed = config.seed.wrapping_sub(1);
        events.write(RegenerateWorldEvent);
        perlin.perlin_main = Perlin::new(config.seed);
        perlin.perlin_detail = Perlin::new(config.seed.wrapping_sub(1337));
        *terrain = TerrainMap::new(&config, &perlin);
        *resources = ResourceMap::new(&terrain, &config);
        *buildings = BuildingMap::new(&config, &terrain, &resources);
        info!("Случайный seed: {}", config.seed);
    }
}