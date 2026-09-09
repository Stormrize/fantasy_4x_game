use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct BuildingMap {
    pub map: Vec<Entity>,
    pub size: usize,
}
