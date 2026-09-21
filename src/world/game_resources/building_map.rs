use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct BuildingMap {
    pub map: Vec<Option<Entity>>,
    pub size: usize,
}
