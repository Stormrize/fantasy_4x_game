//! Units and buildings per owner.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::world::components::Id;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::unit_map::UnitsMap;

pub(super) struct Census {
    units: HashMap<Id, usize>,
    buildings: HashMap<Id, usize>,
}

impl Census {
    pub(super) fn new(units_map: &UnitsMap, buildings_map: &BuildingMap, ids: &Query<&Id>) -> Self {
        Self {
            units: count_by_owner(&units_map.map, ids),
            buildings: count_by_owner(&buildings_map.map, ids),
        }
    }

    pub(super) fn units_of(&self, owner: Id) -> usize {
        self.units.get(&owner).copied().unwrap_or(0)
    }

    pub(super) fn buildings_of(&self, owner: Id) -> usize {
        self.buildings.get(&owner).copied().unwrap_or(0)
    }
}

fn count_by_owner(slots: &[Option<Entity>], ids: &Query<&Id>) -> HashMap<Id, usize> {
    let mut counts = HashMap::new();
    for entity in slots.iter().flatten() {
        let Ok(id) = ids.get(*entity) else { continue };
        *counts.entry(*id).or_insert(0) += 1;
    }
    counts
}
