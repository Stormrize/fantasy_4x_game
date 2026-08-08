use bevy::prelude::Resource;
use crate::world::game_resources::data::resources::ResourceType;

#[derive(Resource)]
pub struct ResourceMap {
    pub map: Vec<ResourceType>,
    pub size: usize,
}