use super::data::ui_race_data::RaceData;
use crate::world::game_config::PlayerRace;
use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;

#[derive(Resource, Default, Debug)]
pub struct InitialRacesData(pub HashMap<PlayerRace, RaceData>);
// todo отдаем game_config
