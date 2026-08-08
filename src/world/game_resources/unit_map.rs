use bevy::prelude::Resource;
use crate::world::game_resources::data::units::Unit;

#[derive(Resource)]
pub struct UnitsMap {
    pub units: Vec<Unit>,
    pub size: usize,
}