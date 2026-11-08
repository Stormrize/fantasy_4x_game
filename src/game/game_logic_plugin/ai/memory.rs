use crate::world::components::Id;
use crate::world::game_resources::WorldMaps;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType};
use crate::world::game_resources::data::resource::{CommonKind, RareKind, ResourceType};
use crate::world::game_resources::data::unit::UnitType;
use bevy::ecs::system::Query;
use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;
use std::collections::HashSet;

/// Building remembered by a bot.
#[derive(Debug, Clone, Copy)]
pub struct SeenBuilding {
    pub owner: Id,
    /// Will be used by the combat AI (threat assessment for barbarian cities
    /// and enemy capitals).
    #[allow(dead_code)]
    pub building_type: BuildingType,
}

/// Memory of every bot, keyed by bot id.
#[derive(Resource, Default)]
pub struct BotMemories(pub HashMap<Id, BotMemory>);

/// Persistent memory of a bot: only what it has actually observed.
///
/// Explored tiles come from general visibility (it never decreases). Contents
/// of a tile are updated only while the bot has full visibility there, so the
/// bot remembers the last state it really saw. Terrain, resources and buildings
/// are kept indefinitely; enemy unit sightings expire after
/// `AiProfile::forget_memory_turns`.
#[derive(Default)]
pub struct BotMemory {
    /// Bot turn counter (used to age enemy unit sightings).
    pub turn: u64,
    /// Tiles the bot has ever explored.
    pub explored: HashSet<usize>,
    /// Remembered terrain (valid for explored tiles).
    pub biome: HashMap<usize, BiomeType>,
    /// Remembered common resources (may have been gathered by someone else;
    /// checked again on sight).
    pub resources: HashMap<usize, CommonKind>,
    /// Remembered rare resources.
    pub rare_resources: HashMap<usize, RareKind>,
    /// Remembered buildings.
    pub buildings: HashMap<usize, SeenBuilding>,
    /// Enemy units: tile -> (owner, type, turn when last seen).
    pub unit_sightings: HashMap<usize, (Id, UnitType, u64)>,
}

impl BotMemory {
    pub fn default_with_id(
        bot_id: Id,
        local_player_id: Id,
        world_maps: &WorldMaps,
        buildings_query: &Query<(&Id, &BuildingType, &BuildingLevel)>,
        units_query: &Query<(&Id, &UnitType)>,
    ) -> Self {
        let mut memory = Self::default();
        memory.observe(
            bot_id,
            local_player_id,
            world_maps,
            buildings_query,
            units_query,
        );
        memory
    }

    /// Updates the memory with what the bot currently sees.
    ///
    /// General visibility is persistent memory, so it only adds explored tiles
    /// and terrain. Dynamic contents are refreshed from full visibility only.
    pub fn observe(
        &mut self,
        bot_id: Id,
        local_player_id: Id,
        world_maps: &WorldMaps,
        buildings_query: &Query<(&Id, &BuildingType, &BuildingLevel)>,
        units_query: &Query<(&Id, &UnitType)>,
    ) {
        let size = world_maps.terrain.size;

        for row in 0..size {
            for col in 0..size {
                let index = row * size + col;
                let fog = &world_maps.fog;

                if fog.is_general_visible_for(row, col, bot_id, local_player_id) {
                    self.explored.insert(index);
                    self.biome
                        .insert(index, world_maps.terrain.get_biome_type(row, col));
                }

                if !fog.is_full_visible_for(row, col, bot_id, local_player_id) {
                    continue;
                }

                match world_maps.resources.get(row, col) {
                    Some(ResourceType::Common(common)) => {
                        self.resources.insert(index, common.kind);
                        self.rare_resources.remove(&index);
                    }
                    Some(ResourceType::Rare(rare)) => {
                        self.rare_resources.insert(index, rare.kind);
                        self.resources.remove(&index);
                    }
                    None => {
                        self.resources.remove(&index);
                        self.rare_resources.remove(&index);
                    }
                }

                match world_maps
                    .buildings
                    .get(row, col)
                    .and_then(|entity| buildings_query.get(entity).ok())
                {
                    Some((owner, building_type, _)) => {
                        self.buildings.insert(
                            index,
                            SeenBuilding {
                                owner: *owner,
                                building_type: *building_type,
                            },
                        );
                    }
                    None => {
                        self.buildings.remove(&index);
                    }
                }

                match world_maps
                    .units
                    .get(row, col)
                    .and_then(|entity| units_query.get(entity).ok())
                {
                    Some((owner, unit_type)) if *owner != bot_id => {
                        self.unit_sightings
                            .insert(index, (*owner, *unit_type, self.turn));
                    }
                    _ => {
                        self.unit_sightings.remove(&index);
                    }
                }
            }
        }
    }
}
