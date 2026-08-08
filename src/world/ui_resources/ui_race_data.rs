use std::collections::HashMap;
use bevy::prelude::Resource;
use super::data::race_data::RaceData;
#[derive(Resource)]
pub struct UiRacesData {
    pub races_data: HashMap<String, RaceData>,
}
