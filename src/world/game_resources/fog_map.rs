use crate::world::components::Id;
use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Fog {
    pub visible_for_player: bool,
    pub visible_for_bots: Box<[(Id, bool)]>,
}

impl Fog {
    pub fn new(player: bool, visible_for_bots: Box<[(Id, bool)]>) -> Self {
        Self {
            visible_for_player: player,
            visible_for_bots,
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
        self.map[row * self.size + col].visible_for_player
    }
}
