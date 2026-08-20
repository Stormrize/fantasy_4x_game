use crate::world::game_resources::data::units::Unit;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct UnitsMap {
    pub units: Vec<Unit>,
    pub size: usize,
}
