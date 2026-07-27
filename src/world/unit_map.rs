use bevy::prelude::Resource;
use crate::world::enums::units::{Level, Unit};

#[derive(Resource)]
pub struct UnitsMap {
    pub units: Vec<(Unit, Level)>,
    pub size: usize,
}