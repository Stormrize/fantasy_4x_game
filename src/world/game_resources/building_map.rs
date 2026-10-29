use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct BuildingMap {
    pub map: Vec<Option<Entity>>,
    pub size: usize,
}

impl BuildingMap {
    pub fn get(&self, row: usize, col: usize) -> Option<Entity> {
        self.map[row * self.size + col]
    }
    pub fn is_some(&self, row: usize, col: usize) -> bool {
        self.map[row * self.size + col].is_some()
    }
    pub fn set(&mut self, row: usize, col: usize, entity: Option<Entity>) {
        self.map[row * self.size + col] = entity;
    }
}
