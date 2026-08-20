use crate::world::game_config::PlayerRace;
use crate::world::ui_and_graphic_resources::data::visual_race_data::VisualRaceData;
use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;

#[derive(Resource, Default, Debug)]
pub struct VisualRacesData(pub HashMap<PlayerRace, VisualRaceData>);
