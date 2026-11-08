use crate::world::components::Id;
use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Fog {
    // General visibility is persistent memory: the tile has been explored at
    // some point. It is never reset by the fog recalculation.
    pub general_visible_player: bool,
    pub general_visible_bots: Vec<(Id, bool)>,
    // Full visibility is the current turn's line of sight: units, buildings,
    // resources and enemy units inside a unit's or building's view radius.
    // It is reset and recomputed on every fog update.
    pub full_visible_player: bool,
    pub full_visible_bots: Vec<(Id, bool)>,
    // Местность, раскрытая свитком навсегда (не сбрасывается пересчётом тумана).
    // буду использовать если нада будет именно full visibility пока unused
    #[serde(default)]
    pub revealed_player: bool,
    #[serde(default)]
    pub revealed_bots: Vec<(Id, bool)>,
}

impl Fog {
    pub fn new(
        general_visible_player: bool,
        general_visible_bots: Vec<(Id, bool)>,
        full_visible_player: bool,
        full_visible_bots: Vec<(Id, bool)>,
    ) -> Self {
        let revealed_bots = general_visible_bots
            .iter()
            .map(|(id, _)| (*id, false))
            .collect();
        Self {
            general_visible_player,
            general_visible_bots,
            full_visible_player,
            full_visible_bots,
            revealed_player: false,
            revealed_bots,
        }
    }

    /// Persistent memory: the tile has been explored for this player/bot.
    pub fn is_general_visible_for(&self, id: Id) -> bool {
        self.general_visible_bots
            .iter()
            .any(|(bot_id, visible)| *bot_id == id && *visible)
    }

    /// Current line of sight: units, buildings and enemy units are visible.
    pub fn is_full_visible_for(&self, id: Id) -> bool {
        self.full_visible_bots
            .iter()
            .any(|(bot_id, visible)| *bot_id == id && *visible)
    }

    #[allow(unused)]
    /// Раскрыто свитком навсегда.
    pub fn is_revealed_for(&self, id: Id) -> bool {
        self.revealed_bots
            .iter()
            .any(|(bot_id, revealed)| *bot_id == id && *revealed)
    }

    pub fn set_general_visible_for(&mut self, id: Id, visible: bool) {
        if let Some((_, value)) = self
            .general_visible_bots
            .iter_mut()
            .find(|(bot_id, _)| *bot_id == id)
        {
            *value |= visible;
        }
    }

    pub fn set_full_visible_for(&mut self, id: Id, visible: bool) {
        if let Some((_, value)) = self
            .full_visible_bots
            .iter_mut()
            .find(|(bot_id, _)| *bot_id == id)
        {
            *value |= visible;
        }
    }

    pub fn set_revealed_for(&mut self, id: Id) {
        if let Some((_, revealed)) = self
            .revealed_bots
            .iter_mut()
            .find(|(bot_id, _)| *bot_id == id)
        {
            *revealed = true;
        }
    }

    /// Resets only the current line of sight before a fog recalculation.
    /// General visibility is persistent explored memory and is intentionally
    /// left untouched, so `full_visible` can never "un-explore" a tile.
    pub fn clear_visibility(&mut self) {
        self.full_visible_player = false;
        for (_, visible) in self.full_visible_bots.iter_mut() {
            *visible = false;
        }
    }
}

#[derive(Resource, Deserialize, Debug)]
pub struct FogMap {
    pub map: Vec<Fog>,
    pub size: usize,
    /// Отладка: весь туман раскрыт для локального игрока.
    /// При пересчёте тумана это состояние сохраняется.
    #[serde(default)]
    pub debug_full_visibility: bool,
}

impl FogMap {
    #[inline]
    #[allow(dead_code)]
    pub fn get(&self, row: usize, col: usize) -> &Fog {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        &self.map[row * self.size + col]
    }

    #[inline]
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut Fog {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        &mut self.map[row * self.size + col]
    }

    /// Whether the local player has ever explored the tile
    /// (persistent memory + permanently revealed tiles).
    pub fn is_general_visible_player(&self, row: usize, col: usize) -> bool {
        let fog = &self.map[row * self.size + col];
        fog.general_visible_player || fog.revealed_player
    }

    pub fn is_full_visible_player(&self, row: usize, col: usize) -> bool {
        let fog = &self.map[row * self.size + col];
        fog.full_visible_player
    }

    /// Whether `id` has ever explored the tile (includes permanently revealed tiles).
    /// `local_player_id` is the local player's id (not necessarily `Id(0)`).
    pub fn is_general_visible_for(
        &self,
        row: usize,
        col: usize,
        id: Id,
        local_player_id: Id,
    ) -> bool {
        let fog = &self.map[row * self.size + col];
        if id == local_player_id {
            fog.general_visible_player || fog.revealed_player
        } else {
            fog.is_general_visible_for(id) || fog.is_revealed_for(id)
        }
    }

    /// Whether `id` currently sees the tile with full visibility.
    /// Permanently revealed tiles are explored memory only, so they do not
    /// grant full visibility on later turns.
    pub fn is_full_visible_for(&self, row: usize, col: usize, id: Id, local_player_id: Id) -> bool {
        let fog = &self.map[row * self.size + col];
        if id == local_player_id {
            fog.full_visible_player
        } else {
            fog.is_full_visible_for(id)
        }
    }

    pub fn set_visibility(
        &mut self,
        row: usize,
        col: usize,
        id: Id,
        local_player_id: Id,
        general: bool,
        full: bool,
    ) {
        let fog = self.get_mut(row, col);
        if id == local_player_id {
            fog.general_visible_player |= general;
            fog.full_visible_player |= full;
        } else {
            fog.set_general_visible_for(id, general);
            fog.set_full_visible_for(id, full);
        }
    }

    pub fn reveal_area(
        &mut self,
        row: usize,
        col: usize,
        radius: usize,
        local_player_id: Id,
        id: Id,
        general: bool,
        full: bool,
    ) {
        let size = self.size;
        let radius = radius as isize;
        for dr in -radius..=radius {
            for dc in -radius..=radius {
                let r = row as isize + dr;
                let c = col as isize + dc;
                if r < 0 || c < 0 || r as usize >= size || c as usize >= size {
                    continue;
                }
                self.set_visibility(r as usize, c as usize, id, local_player_id, general, full);
            }
        }
    }

    pub fn clear_visibility(&mut self) {
        for fog in self.map.iter_mut() {
            fog.clear_visibility();
        }
    }

    #[allow(unused)]
    /// Раскрывает местность для игрока навсегда (свиток открытия местности).
    pub fn set_revealed(&mut self, row: usize, col: usize, id: Id, local_player_id: Id) {
        let fog = self.get_mut(row, col);
        if id == local_player_id {
            fog.revealed_player = true;
        } else {
            fog.set_revealed_for(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clearing_line_of_sight_keeps_explored_memory() {
        let mut fog = Fog::new(true, vec![(Id(1), true)], true, vec![(Id(1), true)]);

        fog.clear_visibility();

        assert!(fog.general_visible_player);
        assert!(fog.is_general_visible_for(Id(1)));
        assert!(!fog.full_visible_player);
        assert!(!fog.is_full_visible_for(Id(1)));
    }

    #[test]
    fn revealed_tiles_are_explored_but_not_full_visible() {
        let mut map = FogMap {
            map: vec![Fog::new(
                false,
                vec![(Id(1), false)],
                false,
                vec![(Id(1), false)],
            )],
            size: 1,
            debug_full_visibility: false,
        };

        map.set_revealed(0, 0, Id(1), Id(0));

        assert!(map.is_general_visible_for(0, 0, Id(1), Id(0)));
        assert!(!map.is_full_visible_for(0, 0, Id(1), Id(0)));
    }
}
