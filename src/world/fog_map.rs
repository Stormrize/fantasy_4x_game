use bevy::prelude::Resource;
#[derive(Resource)]
pub struct FogMap {
    pub map: Vec<bool>,
    pub size: usize,
}