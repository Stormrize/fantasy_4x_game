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

/// Bot's builder unit.
#[derive(Debug, Clone)]
pub struct OwnHelper {
    pub pos: IVec2,
    pub movement_points: u16,
    pub charges: u8,
}

/// Bot's combat unit.
#[derive(Debug, Clone, Copy)]
pub struct OwnCombatUnit {
    pub pos: IVec2,
    pub unit_type: UnitType,
    /// Level and movement points will be used by the combat AI.
    #[allow(dead_code)]
    pub level: UnitLevel,
    #[allow(dead_code)]
    pub movement_points: u16,
}

/// Bot's building.
#[derive(Debug, Clone, Copy)]
pub struct OwnBuilding {
    pub pos: IVec2,
    pub building_type: BuildingType,
    pub level: BuildingLevel,
}

/// World as seen by a bot: only what it sees right now (fog of war) or
/// remembers. The planner works exclusively with this snapshot, so the bot
/// never knows anything it has not observed.
#[derive(Debug)]
pub struct WorldSnapshot {
    pub id: Id,
    pub size: usize,
    /// Movement cost over known terrain (0 means water/unknown).
    pub cost: Vec<u8>,
    /// Biome of known tiles (used for building placement).
    pub biome: Vec<BiomeType>,
    /// Tile has been explored at some point (now or in the past).
    pub explored: Vec<bool>,
    /// Tile is currently fully visible (units and buildings are visible).
    pub visible_now: Vec<bool>,
    /// Will be used by the combat AI (only currently visible targets can be
    /// attacked).
    pub known_resources: Vec<Option<CommonKind>>,
    pub known_rare: Vec<Option<RareKind>>,
    /// Known buildings (including own and neutral ones).
    pub known_buildings: Vec<Option<SeenBuilding>>,
    /// Enemy units: currently visible or recently remembered.
    pub known_enemy_units: Vec<Option<(Id, UnitType)>>,
    pub own_helpers: Vec<OwnHelper>,
    pub own_buildings: Vec<OwnBuilding>,
    /// Own combat units.
    pub own_combat_units: Vec<OwnCombatUnit>,
    /// Cost of hiring own race's combat units by `UnitType::index()`.
    pub combat_unit_costs: [Gold; 3],
    /// Map reveal scrolls in the inventory.
    pub reveal_scrolls: u32,
    pub gold: Gold,
    pub food: Food,
    pub science: Science,
}

/// Updates the bot's memory with what it sees and builds the world snapshot.
///
/// `bot_id` is the bot whose eyes we are looking through. Visibility comes
/// from the fog of war (that bot only), everything else comes from memory.
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

    // 1. Refresh the memory with the current observation.
    memory.observe(
        bot_id,
        local_player_id,
        world_maps,
        buildings_query,
        units_query,
    );

    // 2. Build snapshot arrays: currently visible data comes from the live
    //    maps, everything else from memory.
    let mut cost = vec![0u8; n];
    let mut biome = vec![BiomeType::Water; n];
    let mut explored = vec![false; n];
    let mut visible_now = vec![false; n];
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

            visible_now[index] = full_visible;
            explored[index] = general_visible || memory.explored.contains(&index);

            if full_visible {
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

                known_enemy_units[index] = world_maps
                    .units
                    .get(row, col)
                    .and_then(|entity| units_query.get(entity).ok())
                    .filter(|(owner, _)| **owner != bot_id)
                    .map(|(owner, unit_type)| (*owner, *unit_type));
            } else if let Some(remembered_biome) = memory.biome.get(&index) {
                // The tile was explored before: remember terrain and static
                // contents. Terrain, resources and buildings do not expire.
                biome[index] = *remembered_biome;
                cost[index] = biome_cost(*remembered_biome);

                known_resources[index] = memory.resources.get(&index).copied();
                known_rare[index] = memory.rare_resources.get(&index).copied();
                known_buildings[index] = memory.buildings.get(&index).copied();

                // Enemy units are remembered only while the sighting is fresh.
                if let Some((owner, unit_type, seen_turn)) =
                    memory.unit_sightings.get(&index).copied()
                    && memory.turn.saturating_sub(seen_turn) <= profile.forget_memory_turns
                {
                    known_enemy_units[index] = Some((owner, unit_type));
                }
            }
        }
    }

    // 3. Own builders and buildings.
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
            continue;
        };
        own_helpers.push(OwnHelper {
            pos: IVec2::new((index / size) as i32, (index % size) as i32),
            movement_points: movement.0,
            charges: stats.build_gather_points,
        });
    }
    own_helpers.sort_by_key(|helper| (helper.pos.x, helper.pos.y));

    // Own combat units.
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

    // 4. Own economy and inventory.
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

    // Cost of hiring own race's combat units (by `UnitType::index()`).
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
