//! Reads the ECS and the world maps into plain view structs.
//!
//! Everything here runs once per rebuild, never per frame.

use bevy::prelude::*;

use crate::game::game_logic_plugin::components::TileContent;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::resource::{CommonKind, RareKind, ResourceType};
use crate::world::game_resources::data::unit::{LevelAbilities, UnitLevel, UnitType};
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;

use super::data::{
    AbilityView, BiomeEntry, BuildingIndex, BuildingView, Discovery, ResourceEntry, UnitIndex,
    UnitView,
};
use super::grid::{cell_pos, pos_in_bounds};
use super::params::{BuildingsQuery, CombatUnitsQuery, HelpersQuery};

// Units

pub(crate) fn gather_units(
    units_map: &UnitsMap,
    fog: &FogMap,
    buildings_map: &BuildingMap,
    resources_map: &ResourceMap,
    combat_units: &CombatUnitsQuery,
    helpers: &HelpersQuery,
) -> UnitIndex {
    let size = units_map.size;
    let mut index = UnitIndex::new(size);

    for (cell, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let pos = cell_pos(cell, size);
        let tile_content = tile_content_at(pos, units_map, buildings_map, resources_map, fog);
        let visible = is_fully_visible(pos, fog);

        if let Some(view) = read_unit(entity, pos, tile_content, visible, combat_units, helpers) {
            index.insert(cell, view);
        }
    }

    index
}

/// Reads a unit entity as either a combat unit or a builder (helper).
fn read_unit(
    entity: Entity,
    pos: IVec2,
    tile_content: TileContent,
    visible: bool,
    combat_units: &CombatUnitsQuery,
    helpers: &HelpersQuery,
) -> Option<UnitView> {
    if let Ok((id, race, unit_type, level, basic, hp, movement, movement_max, abilities)) =
        combat_units.get(entity)
    {
        return Some(UnitView {
            pos,
            owner: *id,
            race: race.clone(),
            kind: *unit_type,
            level: Some(*level),
            hp: hp.0.min(basic.health),
            max_hp: basic.health,
            attack: basic.attack,
            defense: basic.defense,
            range: basic.range,
            moves: movement.0,
            moves_max: movement_max.0,
            build_points: None,
            abilities: unlocked_abilities(*level, abilities),
            tile_content,
            visible,
        });
    }

    let (id, race, basic, movement, movement_max) = helpers.get(entity).ok()?;
    Some(UnitView {
        pos,
        owner: *id,
        race: race.clone(),
        kind: UnitType::Helper,
        level: None,
        hp: basic.health,
        max_hp: basic.health,
        attack: 0,
        defense: basic.defense,
        range: 0,
        moves: movement.0,
        moves_max: movement_max.0,
        build_points: Some(basic.build_gather_points),
        abilities: Vec::new(),
        tile_content,
        visible,
    })
}

/// A unit knows one more ability per level; level `n` shows abilities `0..=n`.
fn unlocked_abilities(level: UnitLevel, abilities: &LevelAbilities) -> Vec<AbilityView> {
    let last = (level as usize).min(abilities.0.len().saturating_sub(1));

    abilities
        .0
        .iter()
        .take(last + 1)
        .map(|ability| AbilityView {
            name: ability.name.to_string(),
            damage: ability.damage,
            mana: ability.mana_amount_needs,
            cooldown: ability.cool_down_turns,
            range: ability.range,
            area: ability.area,
        })
        .collect()
}

fn tile_content_at(
    pos: IVec2,
    units_map: &UnitsMap,
    buildings_map: &BuildingMap,
    resources_map: &ResourceMap,
    fog: &FogMap,
) -> TileContent {
    if !pos_in_bounds(pos, units_map.size) {
        return TileContent::default();
    }
    let (x, y) = (pos.x as usize, pos.y as usize);
    TileContent::from_world(
        true,
        units_map.is_some(x, y),
        buildings_map.is_some(x, y),
        resources_map.is_some(x, y),
        !fog.is_general_visible_player(x, y),
    )
}

fn is_fully_visible(pos: IVec2, fog: &FogMap) -> bool {
    pos_in_bounds(pos, fog.size)
        && (fog.debug_full_visibility
            || fog.is_full_visible_player(pos.x as usize, pos.y as usize))
}

// Buildings

pub(crate) fn gather_buildings(
    buildings_map: &BuildingMap,
    buildings: &BuildingsQuery,
) -> BuildingIndex {
    let size = buildings_map.size;
    let mut index = BuildingIndex::new(size);

    for (cell, slot) in buildings_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok((id, kind, level, levels)) = buildings.get(entity) else {
            continue;
        };
        let stats = levels.stats(*level);

        index.insert(
            cell,
            BuildingView {
                pos: cell_pos(cell, size),
                owner: *id,
                kind: *kind,
                level: *level,
                gold: stats.gold(),
                food: stats.food(),
                science: stats.science(),
                housing: stats.housing(),
                spawn_capacity: stats.spawn_capacity(),
                maintenance: stats.maintenance(),
            },
        );
    }

    index
}

// Discovery

/// Walks the explored part of the map once and aggregates unique biomes and
/// resource kinds. `O(size * size)` per rebuild, not per lookup.
pub(crate) fn gather_discovery(
    terrain: &TerrainMap,
    fog: &FogMap,
    resources_map: &ResourceMap,
) -> Discovery {
    const BIOME_ORDER: [BiomeType; 5] = [
        BiomeType::SmallBiome,
        BiomeType::MediumBiome,
        BiomeType::LargeBiome,
        BiomeType::Ground,
        BiomeType::Water,
    ];

    let size = terrain.size;
    let mut biome_counts = [0usize; 5];
    let mut resources = ResourceTotals::default();

    for (index, biome) in terrain.map.iter().enumerate() {
        if !fog.is_general_visible_player(index / size, index % size) {
            continue;
        }

        biome_counts[biome.biome_type as usize] += 1;

        if let Some(Some(resource)) = resources_map.map.get(index) {
            resources.add(resource);
        }
    }

    let biomes = BIOME_ORDER
        .iter()
        .filter_map(|kind| {
            let tiles = biome_counts[*kind as usize];
            (tiles > 0).then_some(BiomeEntry { kind: *kind, tiles })
        })
        .collect();

    Discovery {
        biomes,
        resource_tiles: resources.tiles,
        resources: resources.into_entries(),
    }
}

/// Per-kind counters filled while walking the explored map.
#[derive(Default)]
struct ResourceTotals {
    /// Explored tiles that hold any resource.
    tiles: usize,
    common_tiles: [usize; 3],
    common_amount: [u32; 3],
    rare_tiles: [usize; 4],
    rare_power: [u32; 4],
}

impl ResourceTotals {
    fn add(&mut self, resource: &ResourceType) {
        self.tiles += 1;
        match resource {
            ResourceType::Common(common) => {
                let slot = common.kind.index();
                self.common_tiles[slot] += 1;
                self.common_amount[slot] = self.common_amount[slot].saturating_add(common.amount);
            }
            ResourceType::Rare(rare) => {
                let slot = rare.kind as usize;
                self.rare_tiles[slot] += 1;
                self.rare_power[slot] = self.rare_power[slot].max(rare.power);
            }
        }
    }

    /// Unique kinds only: common ones first, then rare ones.
    fn into_entries(self) -> Vec<ResourceEntry> {
        let mut entries = Vec::new();

        for kind in CommonKind::ALL {
            let slot = kind.index();
            if self.common_tiles[slot] > 0 {
                entries.push(ResourceEntry::Common {
                    kind,
                    tiles: self.common_tiles[slot],
                    amount: self.common_amount[slot],
                });
            }
        }
        for kind in RareKind::ALL {
            let slot = kind as usize;
            if self.rare_tiles[slot] > 0 {
                entries.push(ResourceEntry::Rare {
                    kind,
                    tiles: self.rare_tiles[slot],
                    power: self.rare_power[slot],
                });
            }
        }

        entries
    }
}
