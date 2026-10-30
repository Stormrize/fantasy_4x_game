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

    /// Общая видимость для игрока/бота: местность видна, но юниты — нет.
    pub fn is_general_visible_for(&self, id: Id) -> bool {
        self.general_visible_bots
            .iter()
            .any(|(bot_id, visible)| *bot_id == id && *visible)
    }

    /// Полная видимость: видны и юниты.
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

    /// Сбрасывает текущую видимость клетки (вызывается при пересчёте тумана).
    /// Раскрытое свитком (`revealed`) не трогается.
    pub fn clear_visibility(&mut self) {
        self.full_visible_player = false;
        for (_, visible) in self.general_visible_bots.iter_mut() {
            *visible = false;
        }
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

    /// Видно ли клетку локальному игроку (общая видимость + раскрытое свитком).
    pub fn is_visible_player(&self, row: usize, col: usize) -> bool {
        let fog = &self.map[row * self.size + col];
        fog.general_visible_player || fog.revealed_player
    }

    /// Общая видимость для `id` (включает раскрытое свитком).
    /// `local_player_id` — id локального игрока (не всегда `Id(0)`).
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

    pub fn is_full_visible_for(&self, row: usize, col: usize, id: Id, local_player_id: Id) -> bool {
        let fog = &self.map[row * self.size + col];
        if id == local_player_id {
            fog.full_visible_player || fog.revealed_player
        } else {
            fog.is_full_visible_for(id) || fog.is_revealed_for(id)
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
