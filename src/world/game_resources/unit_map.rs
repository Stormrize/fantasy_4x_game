use crate::world::game_resources::data::unit::Unit;
use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct UnitsMap {
    pub units: Vec<Entity>,
    pub size: usize,
}
