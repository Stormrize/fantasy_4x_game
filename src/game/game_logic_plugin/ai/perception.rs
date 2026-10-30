use crate::game::game_logic_plugin::action::rules::movement::biome_cost;
use crate::game::game_logic_plugin::ai::memory::{BotMemory, SeenBuilding};
use crate::game::game_logic_plugin::ai::profile::AiProfile;
use crate::world::components::{Food, Gold, Id, Science};
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::WorldMaps;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType};
use crate::world::game_resources::data::resource::{CommonKind, RareKind, ResourceType};
use crate::world::game_resources::data::unit::{
    HelperBasicStats, MovementPoints, UnitLevel, UnitType,
};

use bevy::math::IVec2;
use bevy::prelude::Query;

/// Строитель бота.
#[derive(Debug, Clone)]
pub struct OwnHelper {
    pub pos: IVec2,
    pub movement_points: u16,
    pub charges: u8,
}

/// Боевой юнит бота.
#[derive(Debug, Clone, Copy)]
pub struct OwnCombatUnit {
    pub pos: IVec2,
    pub unit_type: UnitType,
    /// Уровень и очки движения понадобятся боевому ИИ.
    #[allow(dead_code)]
    pub level: UnitLevel,
    #[allow(dead_code)]
    pub movement_points: u16,
}

/// Здание бота.
#[derive(Debug, Clone, Copy)]
pub struct OwnBuilding {
    pub pos: IVec2,
    pub building_type: BuildingType,
    pub level: BuildingLevel,
}

/// Снимок мира с точки зрения бота: только то, что он видит сейчас
/// (туман войны) или помнит. Планировщик работает исключительно с ним,
/// поэтому бот не знает о том, чего не видел.
#[derive(Debug)]
pub struct WorldSnapshot {
    pub id: Id,
    pub size: usize,
    /// Стоимость передвижения по известной местности (0 — вода/неизвестно).
    pub cost: Vec<u8>,
    /// Тип биома известных клеток (для выбора зданий).
    pub biome: Vec<BiomeType>,
    /// Клетка когда-либо была видна (сейчас или в прошлом).
    pub explored: Vec<bool>,
    /// Общая видимость в текущий ход.
    pub visible_now: Vec<bool>,
    /// Полная видимость (видны юниты) в текущий ход.
    /// Понадобится боевому ИИ (атаковать можно только то, что реально видно).
    #[allow(dead_code)]
    pub full_visible_now: Vec<bool>,
    pub known_resources: Vec<Option<CommonKind>>,
    pub known_rare: Vec<Option<RareKind>>,
    /// Известные здания (включая свои и нейтральные).
    pub known_buildings: Vec<Option<SeenBuilding>>,
    /// Чужие юниты: видимые сейчас или свежие воспоминания.
    pub known_enemy_units: Vec<Option<(Id, UnitType)>>,
    pub own_helpers: Vec<OwnHelper>,
    pub own_buildings: Vec<OwnBuilding>,
    /// Свои боевые юниты.
    pub own_combat_units: Vec<OwnCombatUnit>,
    /// Стоимость найма боевых юнитов своей расы по индексу `UnitType::index()`.
    pub combat_unit_costs: [Gold; 3],
    /// Свитки открытия местности в инвентаре.
    pub reveal_scrolls: u32,
    pub gold: Gold,
    pub food: Food,
    pub science: Science,
}

/// Обновляет память бота увиденным и собирает снимок мира.
///
/// `bot_id` — id бота, чьими глазами смотрим. Видимость берётся из тумана
/// войны (только своя), всё остальное — из памяти.
#[allow(clippy::too_many_arguments)]
pub fn build_snapshot<'w, 's>(
    bot_id: Id,
    local_player_id: Id,
    world_maps: &WorldMaps,
    buildings_query: &Query<'w, 's, (&Id, &BuildingType, &BuildingLevel)>,
    units_query: &Query<'w, 's, (&Id, &UnitType)>,
    helpers_query: &Query<'w, 's, (&MovementPoints, &HelperBasicStats)>,
    combat_query: &Query<'w, 's, (&Id, &UnitType, &UnitLevel, &MovementPoints)>,
    players: &Players,
    game_logic_config: &GameLogicConfig,
    memory: &mut BotMemory,
    profile: &AiProfile,
) -> WorldSnapshot {
    let size = world_maps.terrain.size;
    let n = size * size;
    memory.turn += 1;

    // 1. Обновляем память тем, что бот видит в этот ход.
    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            let general_visible =
                world_maps
                    .fog
                    .is_general_visible_for(row, col, bot_id, local_player_id);
            let full_visible =
                world_maps
                    .fog
                    .is_full_visible_for(row, col, bot_id, local_player_id);

            if general_visible {
                memory.explored.insert(index);
                memory
                    .biome
                    .insert(index, world_maps.terrain.get_biome_type(row, col));

                match world_maps.resources.get(row, col) {
                    Some(ResourceType::Common(common)) => {
                        memory.resources.insert(index, common.kind);
                        memory.rare_resources.remove(&index);
                    }
                    Some(ResourceType::Rare(rare)) => {
                        memory.rare_resources.insert(index, rare.kind);
                        memory.resources.remove(&index);
                    }
                    None => {
                        memory.resources.remove(&index);
                        memory.rare_resources.remove(&index);
                    }
                }

                match world_maps
                    .buildings
                    .get(row, col)
                    .and_then(|entity| buildings_query.get(entity).ok())
                {
                    Some((owner, building_type, _)) => {
                        memory.buildings.insert(
                            index,
                            SeenBuilding {
                                owner: *owner,
                                building_type: *building_type,
                            },
                        );
                    }
                    None => {
                        memory.buildings.remove(&index);
                    }
                }
            }

            if full_visible {
                match world_maps
                    .units
                    .get(row, col)
                    .and_then(|entity| units_query.get(entity).ok())
                {
                    Some((owner, unit_type)) if *owner != bot_id => {
                        memory
                            .unit_sightings
                            .insert(index, (*owner, *unit_type, memory.turn));
                    }
                    _ => {
                        memory.unit_sightings.remove(&index);
                    }
                }
            }
        }
    }

    // 2. Собираем массивы снимка: сейчас видимое — из карт, иначе — из памяти.
    let mut cost = vec![0u8; n];
    let mut biome = vec![BiomeType::Water; n];
    let mut explored = vec![false; n];
    let mut visible_now = vec![false; n];
    let mut full_visible_now = vec![false; n];
    let mut known_resources = vec![None; n];
    let mut known_rare = vec![None; n];
    let mut known_buildings = vec![None; n];
    let mut known_enemy_units = vec![None; n];

    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            let general_visible =
                world_maps
                    .fog
                    .is_general_visible_for(row, col, bot_id, local_player_id);
            let full_visible =
                world_maps
                    .fog
                    .is_full_visible_for(row, col, bot_id, local_player_id);

            visible_now[index] = general_visible;
            full_visible_now[index] = full_visible;
            explored[index] = general_visible || memory.explored.contains(&index);

            if general_visible {
                let terrain_biome = world_maps.terrain.get_biome_type(row, col);
                biome[index] = terrain_biome;
                cost[index] = biome_cost(terrain_biome);

                match world_maps.resources.get(row, col) {
                    Some(ResourceType::Common(common)) => {
                        known_resources[index] = Some(common.kind);
                    }
                    Some(ResourceType::Rare(rare)) => {
                        known_rare[index] = Some(rare.kind);
                    }
                    None => {}
                }

                known_buildings[index] = world_maps
                    .buildings
                    .get(row, col)
                    .and_then(|entity| buildings_query.get(entity).ok())
                    .map(|(owner, building_type, _)| SeenBuilding {
                        owner: *owner,
                        building_type: *building_type,
                    });

                if full_visible {
                    known_enemy_units[index] = world_maps
                        .units
                        .get(row, col)
                        .and_then(|entity| units_query.get(entity).ok())
                        .filter(|(owner, _)| **owner != bot_id)
                        .map(|(owner, unit_type)| (*owner, *unit_type));
                }
            } else if let Some(remembered_biome) = memory.biome.get(&index) {
                // Клетку видели раньше — помним местность и статику.
                biome[index] = *remembered_biome;
                cost[index] = biome_cost(*remembered_biome);
                known_resources[index] = memory.resources.get(&index).copied();
                known_rare[index] = memory.rare_resources.get(&index).copied();
                known_buildings[index] = memory.buildings.get(&index).copied();

                // Юниты — только если воспоминание ещё свежее.
                if let Some((owner, unit_type, seen_turn)) =
                    memory.unit_sightings.get(&index).copied()
                    && memory.turn.saturating_sub(seen_turn) <= profile.unit_memory_turns
                {
                    known_enemy_units[index] = Some((owner, unit_type));
                }
            }
        }
    }

    // 3. Свои строители и здания.
    let mut own_helpers = Vec::new();
    for (index, slot) in world_maps.units.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok((owner, _)) = units_query.get(entity) else {
            continue;
        };
        if *owner != bot_id {
            continue;
        }
        let Ok((movement, stats)) = helpers_query.get(entity) else {
            // Боевые юниты — ими займётся боевой ИИ.
            continue;
        };
        own_helpers.push(OwnHelper {
            pos: IVec2::new((index / size) as i32, (index % size) as i32),
            movement_points: movement.0,
            charges: stats.build_gather_points,
        });
    }
    own_helpers.sort_by_key(|helper| (helper.pos.x, helper.pos.y));

    // Свои боевые юниты.
    let mut own_combat_units = Vec::new();
    for (index, slot) in world_maps.units.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok((owner, unit_type, level, movement)) = combat_query.get(entity) else {
            continue;
        };
        if *owner != bot_id {
            continue;
        }
        own_combat_units.push(OwnCombatUnit {
            pos: IVec2::new((index / size) as i32, (index % size) as i32),
            unit_type: *unit_type,
            level: *level,
            movement_points: movement.0,
        });
    }
    own_combat_units.sort_by_key(|unit| (unit.pos.x, unit.pos.y));

    let mut own_buildings = Vec::new();
    for (index, slot) in world_maps.buildings.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok((owner, building_type, level)) = buildings_query.get(entity) else {
            continue;
        };
        if *owner != bot_id {
            continue;
        }
        own_buildings.push(OwnBuilding {
            pos: IVec2::new((index / size) as i32, (index % size) as i32),
            building_type: *building_type,
            level: *level,
        });
    }
    own_buildings.sort_by_key(|building| (building.pos.x, building.pos.y));

    // 4. Своя экономика и инвентарь.
    let player = players.0.iter().find(|p| p.id == bot_id);
    let (gold, food, science) = player
        .map(|p| (p.gold, p.food, p.science))
        .unwrap_or((0, 0, 0));
    let reveal_scrolls = player
        .map(|p| {
            p.inventory
                .iter()
                .filter(|rare| rare.kind == RareKind::RevealScroll)
                .count() as u32
        })
        .unwrap_or(0);

    // Стоимость найма боевых юнитов своей расы (по индексам UnitType::index()).
    let combat_unit_costs = player
        .and_then(|p| game_logic_config.units_data.get(&p.player_race))
        .map(|units_data| {
            let mut costs = [0u16; 3];
            for (index, data) in units_data.0.iter().enumerate() {
                if index < 3 {
                    costs[index] = data.level_economy.0[0].new_level_cost;
                }
            }
            costs
        })
        .unwrap_or([0u16; 3]);

    WorldSnapshot {
        id: bot_id,
        size,
        cost,
        biome,
        explored,
        visible_now,
        full_visible_now,
        known_resources,
        known_rare,
        known_buildings,
        known_enemy_units,
        own_helpers,
        own_buildings,
        own_combat_units,
        combat_unit_costs,
        reveal_scrolls,
        gold,
        food,
        science,
    }
}
