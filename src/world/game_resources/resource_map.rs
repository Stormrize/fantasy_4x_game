use bevy::prelude::Resource;

use crate::world::game_resources::data::resource::ResourceType;

#[derive(Resource)]
pub struct ResourceMap {
    pub map: Vec<ResourceType>,
    pub size: usize,
}
