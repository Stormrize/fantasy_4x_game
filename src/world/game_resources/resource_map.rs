use crate::world::game_resources::data::resource::ResourceType;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ResourceMap {
    pub map: Vec<ResourceType>,
    pub size: usize,
}
