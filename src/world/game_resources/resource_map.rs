use crate::world::game_resources::data::resource::ResourceType;
use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Resource, Deserialize, Debug)]
pub struct ResourceMap {
    pub map: Vec<Option<ResourceType>>,
    pub size: usize,
}

impl ResourceMap {
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<&ResourceType> {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        self.map[row * self.size + col].as_ref()
    }

    pub fn is_some(&self, row: usize, col: usize) -> bool {
        self.map[row * self.size + col].is_some()
    }
}
