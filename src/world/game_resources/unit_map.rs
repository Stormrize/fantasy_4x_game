use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct UnitsMap {
    pub map: Vec<Option<Entity>>,
    pub size: usize,
}
