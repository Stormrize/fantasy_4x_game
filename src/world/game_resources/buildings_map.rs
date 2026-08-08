use bevy::prelude::Resource;
use crate::world::game_resources::data::buildings::BuildingType;

#[derive(Resource)]
pub struct BuildingMap {
    pub map: Vec<(BuildingType, u8)>,
    pub size: usize,
}