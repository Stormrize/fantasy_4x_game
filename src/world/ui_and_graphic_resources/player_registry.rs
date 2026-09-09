//! File that contains hashmap for lookups

use crate::world::components::PlayerRace;
use crate::world::ui_and_graphic_resources::data::ui_race_data::UiRaceData;
use crate::world::ui_and_graphic_resources::data::visual_race_data::RaceAssets;
use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;

/// # Examples
/// ```
/// let player_race = player_registry.race_of(player_id);
/// let race_assets = races_assets_registry.race_assets_of(player_race);
/// ```

#[derive(Resource, Default, Debug)]
pub struct RacesUiRegistry(pub HashMap<PlayerRace, UiRaceData>);

impl RacesUiRegistry {
    pub fn ui_race_data_of(&self, player: &PlayerRace) -> &UiRaceData {
        &self.0[player]
    }
}

#[derive(Resource, Default, Debug)]
pub struct RacesAssetsRegistry(pub HashMap<PlayerRace, RaceAssets>);

impl RacesAssetsRegistry {
    pub fn race_assets_of(&self, player: &PlayerRace) -> &RaceAssets {
        &self.0[player]
    }
}
