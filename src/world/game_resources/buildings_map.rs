use crate::world::game_resources::data::buildings::BuildingType;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BuildingMap {
    pub map: Vec<BuildingType>,
    pub size: usize,
}
