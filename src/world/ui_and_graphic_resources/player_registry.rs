//! Файл, который хранить в себе hashmap нужные для lookup

use crate::world::game_config::PlayerRace;
use crate::world::ui_and_graphic_resources::data::ui_race_data::UiRaceData;
use crate::world::ui_and_graphic_resources::data::visual_race_data::RaceAssets;
use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;

/// PLayerRegistry хеш меп, чтобы достать PlayerRace
/// # Examples
/// ```
/// let player_id = biome.inside().unwrap;
/// let player_race = player_registry.race_of(player_id);
/// let race_assets = races_assets_registry.race_assets_of(player_race);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    LocalPlayerId(u8),
    MultiPlayerId(u64),
    Unoccupied,
}

impl Id {
    pub fn default_local_id() -> Self {
        Self::LocalPlayerId(255)
    }

    pub fn default_multi_id() -> Self {
        Self::MultiPlayerId(u64::MAX)
    }
}

#[derive(Resource)]
pub struct PlayerRegistry(pub HashMap<Id, PlayerRace>);

impl PlayerRegistry {
    pub fn race_of(&self, id: Id) -> &PlayerRace {
        &self.0[&id]
    }
}

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
