use crate::world::components::Id;
use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Fog {
    // general visibility no units visible
    pub general_visible_player: bool,
    pub general_visible_bots: Vec<(Id, bool)>,
    // full visibility units visible
    pub full_visible_player: bool,
    pub full_visible_bots: Vec<(Id, bool)>,
}

impl Fog {
    pub fn new(
        general_visible_player: bool,
        general_visible_bots: Vec<(Id, bool)>,
        full_visible_player: bool,
        full_visible_bots: Vec<(Id, bool)>,
    ) -> Self {
        Self {
            general_visible_player,
            general_visible_bots,
            full_visible_player,
            full_visible_bots,
        }
    }
}

#[derive(Resource, Deserialize, Debug)]
pub struct FogMap {
    pub map: Vec<Fog>,
    pub size: usize,
}

impl FogMap {
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> &Fog {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        &self.map[row * self.size + col]
    }

    pub fn is_visible_player(&self, row: usize, col: usize) -> bool {
        self.map[row * self.size + col].general_visible_player
    }
}
