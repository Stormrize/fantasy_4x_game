use crate::world::components::Id;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::building::BuildingType;
use crate::world::game_resources::data::resource::{CommonKind, RareKind};
use crate::world::game_resources::data::unit::UnitType;
use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;
use std::collections::HashSet;

/// Здание, которое бот когда-то видел.
#[derive(Debug, Clone, Copy)]
pub struct SeenBuilding {
    pub owner: Id,
    /// Понадобится боевому ИИ (оценка угроз городов-варваров, вражеских столиц).
    #[allow(dead_code)]
    pub building_type: BuildingType,
}

/// Память всех ботов (ключ — id бота).
#[derive(Resource, Default)]
pub struct BotMemories(pub HashMap<Id, BotMemory>);

/// Постоянная память бота: только то, что он реально видел через туман войны.
///
/// Местность, ресурсы и здания помнятся (они почти статичны), чужие юниты
/// забываются по `AiProfile::unit_memory_turns` — как и у человека,
/// который помнит, что «где-то здесь был враг», но не вечно.
#[derive(Default)]
pub struct BotMemory {
    /// Счётчик ходов бота (для устаревания воспоминаний о юнитах).
    pub turn: u64,
    /// Клетки, которые бот когда-либо видел (общая видимость).
    pub explored: HashSet<usize>,
    /// Запомненная местность (только для explored).
    pub biome: HashMap<usize, BiomeType>,
    /// Запомненные обычные ресурсы (могли быть собраны другими — проверяется при виде).
    pub resources: HashMap<usize, CommonKind>,
    /// Запомненные редкие ресурсы.
    pub rare_resources: HashMap<usize, RareKind>,
    /// Запомненные здания.
    pub buildings: HashMap<usize, SeenBuilding>,
    /// Замеченные чужие юниты: клетка -> (владелец, тип, ход, когда видели).
    pub unit_sightings: HashMap<usize, (Id, UnitType, u64)>,
}
