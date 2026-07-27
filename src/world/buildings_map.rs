use bevy::prelude::Resource;
use crate::world::enums::buildings::BuildingType;

#[derive(Resource)]
pub struct BuildingMap {
    pub map: Vec<(BuildingType, u8)>,
    pub size: usize,
}