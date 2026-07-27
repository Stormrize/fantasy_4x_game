use bevy::prelude::Resource;
use crate::world::enums::resources::ResourceType;

#[derive(Resource)]
pub struct ResourceMap {
    pub map: Vec<ResourceType>,
    pub size: usize,
}