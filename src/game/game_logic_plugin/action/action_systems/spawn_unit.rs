use crate::game::game_logic_plugin::events_resources_structs::action_messages::SpawnUnitCommand;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::buffs_and_debuffs::{Buffs, Debuffs};
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType, Levels};
use crate::world::game_resources::data::unit::{
    CombatUnit, HELPER, Helper, MovementPointsMax, UnitLevel, UnitType,
};
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;
use bevy::prelude::{Commands, MessageReader, Query, Res, ResMut};

/// Наём юнита в городе текущего игрока. Строитель (`unit_type: None`)
/// нанимается в любом городе, боевой юнит — в здании спавна
/// (`SpawningBuilding`) с положительной `spawn_capacity`.
#[allow(clippy::too_many_arguments)]
pub fn spawn_unit(
    mut spawn_command: MessageReader<SpawnUnitCommand>,
    turn_manager: Res<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    terrain: Res<TerrainMap>,
    mut units_map: ResMut<UnitsMap>,
    buildings_map: Res<BuildingMap>,
    buildings_query: Query<(&Id, &PlayerRace, &BuildingType, &BuildingLevel, &Levels)>,
    mut players: ResMut<Players>,
    mut commands: Commands,
) {
    let size = terrain.size;

    for command in spawn_command.read() {
        let pos = command.city_pos;
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= size || pos.y as usize >= size {
            continue;
        }
        let (x, y) = (pos.x as usize, pos.y as usize);

        let Some(city_entity) = buildings_map.get(x, y) else {
            continue;
        };
        let Ok((city_id, city_race, building_type, building_level, levels)) =
            buildings_query.get(city_entity)
        else {
            continue;
        };
        if *city_id != turn_manager.current_player.id {
            continue;
        }

        let Some(player) = players.0.iter_mut().find(|p| p.id == *city_id) else {
            continue;
        };
        let Some((new_x, new_y)) = find_free_adjacent(&terrain, &units_map, &buildings_map, x, y)
        else {
            continue;
        };

        match command.unit_type {
            // Строитель — нанимается в любом городе.
            UnitType::Helper => {
                if !matches!(
                    *building_type,
                    BuildingType::Capital
                        | BuildingType::CapturedCity
                        | BuildingType::SpawningBuilding
                ) {
                    continue;
                }
                if player.gold < HELPER.unit_cost.0 {
                    continue;
                }
                player.gold -= HELPER.unit_cost.0;

                let entity = commands
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
                units_map.set(new_x, new_y, Some(entity));
            }
            // Боевой юнит — только в здании спавна.
            unit_type => {
                if *building_type != BuildingType::SpawningBuilding {
                    continue;
                }
                if levels.stats(*building_level).spawn_capacity() == 0 {
                    continue;
                }
                let Some(units_data) = game_logic_config.units_data.get(city_race) else {
                    continue;
                };
                let data = &units_data.0[unit_type.index()];
                let cost = data.level_economy.0[0].new_level_cost;
                if player.gold < cost {
                    continue;
                }
                player.gold -= cost;

                let entity = commands
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
                units_map.set(new_x, new_y, Some(entity));
            }
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
