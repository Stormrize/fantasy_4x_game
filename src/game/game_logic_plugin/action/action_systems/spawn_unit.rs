use bevy::math::IVec2;

use crate::game::game_logic_plugin::action::action_systems::fog_update::fog_update;
use crate::game::game_logic_plugin::action::action_systems::{ActionCtx, tile};
use crate::game::game_logic_plugin::events_resources_structs::action_messages::RevealSource;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::buffs_and_debuffs::{Buffs, Debuffs};
use crate::world::game_resources::data::building::BuildingType;
use crate::world::game_resources::data::unit::{
    CombatUnit, HELPER, Helper, MovementPointsMax, UnitLevel, UnitType,
};
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;

/// Hires a unit in the city at `pos`; it appears on a free adjacent tile.
pub fn spawn_unit(ctx: &mut ActionCtx, pos: IVec2, unit_type: UnitType) {
    let Some((x, y)) = tile(pos, ctx.terrain.size) else {
        return;
    };
    let Some(city) = ctx.buildings_map.get(x, y) else {
        return;
    };
    let Ok((city_id, city_race, building_type, building_level, levels)) = ctx.buildings.get(city)
    else {
        return;
    };
    if *city_id != ctx.turn_manager.current_player.id {
        return;
    }
    let Some((nx, ny)) = find_free_adjacent(&ctx.terrain, &ctx.units_map, &ctx.buildings_map, x, y)
    else {
        return;
    };
    let Some(player) = ctx.players.0.iter_mut().find(|p| p.id == *city_id) else {
        return;
    };

    match unit_type {
        // Helper: only in a capital, a captured city or a spawning building.
        UnitType::Helper => {
            if !matches!(
                *building_type,
                BuildingType::Capital | BuildingType::CapturedCity | BuildingType::SpawningBuilding
            ) {
                return;
            }
            if player.gold < HELPER.unit_cost.0 {
                return;
            }
            player.gold -= HELPER.unit_cost.0;

            let entity = ctx
                .commands
                .spawn((
                    Helper {
                        player_id: *city_id,
                        player_race: city_race.clone(),
                        unit_data: HELPER,
                        active_buffs: Buffs::default(),
                        active_debuffs: Debuffs::default(),
                    },
                    MovementPointsMax(HELPER.movement_points.0),
                ))
                .id();
            ctx.units_map.set(nx, ny, Some(entity));
            // ASSUMPTION: field name `field_of_view` (see questions).

            fog_update(
                ctx,
                IVec2::new(nx as i32, ny as i32),
                RevealSource::Unit,
                true,
            );
        }
        // Combat unit: only in a spawning building.
        unit_type => {
            if *building_type != BuildingType::SpawningBuilding {
                return;
            }
            if levels.stats(*building_level).spawn_capacity() == 0 {
                return;
            }
            let Some(units_data) = ctx.config.units_data.get(city_race) else {
                return;
            };
            let data = &units_data.0[unit_type.index()];
            let cost = data
                .level_economy
                .level_cost(UnitLevel::Beginner)
                .unwrap()
                .new_level_cost;
            if player.gold < cost {
                return;
            }
            player.gold -= cost;

            let entity = ctx
                .commands
                .spawn((
                    CombatUnit {
                        player_id: *city_id,
                        player_race: city_race.clone(),
                        level: UnitLevel::Beginner,
                        unit_data: data.clone(),
                        active_buffs: Buffs::default(),
                        active_debuffs: Debuffs::default(),
                    },
                    MovementPointsMax(data.movement_points.0),
                ))
                .id();
            ctx.units_map.set(nx, ny, Some(entity));
            fog_update(
                ctx,
                IVec2::new(nx as i32, ny as i32),
                RevealSource::Unit,
                true,
            );
        }
    }
}

fn find_free_adjacent(
    terrain: &TerrainMap,
    units_map: &UnitsMap,
    buildings_map: &BuildingMap,
    x: usize,
    y: usize,
) -> Option<(usize, usize)> {
    const DELTAS: [(isize, isize); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (dx, dy) in DELTAS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 || nx as usize >= terrain.size || ny as usize >= terrain.size {
            continue;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if terrain.get_biome_type(nx, ny) == BiomeType::Water {
            continue;
        }
        if units_map.is_some(nx, ny) || buildings_map.is_some(nx, ny) {
            continue;
        }
        return Some((nx, ny));
    }
    None
}
