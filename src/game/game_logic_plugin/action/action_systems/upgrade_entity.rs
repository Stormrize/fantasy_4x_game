use crate::game::game_logic_plugin::events_resources_structs::action_messages::Upgrade;
use crate::game::game_logic_plugin::events_resources_structs::components::TileLayer;
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderBuildEntity;
use crate::world::components::Id;
use crate::world::game_configs::Players;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::building::{BuildingLevel, Levels};
use crate::world::game_resources::data::unit::{LevelCost, UnitLevel};
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;
use bevy::prelude::{MessageReader, MessageWriter, Query, Res, ResMut};

/// Улучшение здания или боевого юнита за золото (стоимость — `new_level_cost`
/// текущего уровня). Апгрейд выполняется во время хода, до SimulationTime.
#[allow(clippy::too_many_arguments)]
pub fn upgrade_entity(
    mut upgrade_command: MessageReader<Upgrade>,
    turn_manager: Res<TurnManager>,
    buildings_map: Res<BuildingMap>,
    units_map: Res<UnitsMap>,
    mut buildings_query: Query<(&Id, &mut BuildingLevel, &Levels)>,
    mut units_query: Query<(&Id, &mut UnitLevel, &LevelCost)>,
    mut players: ResMut<Players>,
    mut render_build: MessageWriter<RenderBuildEntity>,
) {
    let size = buildings_map.size;

    for upgrade in upgrade_command.read() {
        let pos = upgrade.entity_pos;
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= size || pos.y as usize >= size {
            continue;
        }
        let (x, y) = (pos.x as usize, pos.y as usize);

        match upgrade.layer {
            TileLayer::Building => {
                let Some(building_entity) = buildings_map.get(x, y) else {
                    continue;
                };
                let Ok((building_id, mut level, levels)) = buildings_query.get_mut(building_entity)
                else {
                    continue;
                };
                if *building_id != turn_manager.current_player.id {
                    continue;
                }
                let Some(next) = level.next() else {
                    continue;
                };
                let cost = levels.stats(*level).upgrade_cost();

                let Some(player) = players.0.iter_mut().find(|p| p.id == *building_id) else {
                    continue;
                };
                if player.gold < cost {
                    continue;
                }
                player.gold -= cost;

                *level = next;
                render_build.write(RenderBuildEntity {
                    entity: building_entity,
                });
            }
            TileLayer::Unit => {
                let Some(unit_entity) = units_map.get(x, y) else {
                    continue;
                };
                let Ok((unit_id, mut level, level_cost)) = units_query.get_mut(unit_entity) else {
                    continue;
                };
                if *unit_id != turn_manager.current_player.id {
                    continue;
                }

                // Количество уровней юнита задаётся длиной `LevelCost`.
                let current = *level as usize;
                if current + 1 >= level_cost.0.len() {
                    continue;
                }
                let cost = level_cost.0[current].new_level_cost;

                let Some(player) = players.0.iter_mut().find(|p| p.id == *unit_id) else {
                    continue;
                };
                if player.gold < cost {
                    continue;
                }
                player.gold -= cost;

                *level = match *level {
                    UnitLevel::Beginner => UnitLevel::Master,
                    UnitLevel::Master => UnitLevel::Holy,
                    UnitLevel::Holy => continue,
                };
            }
            _ => continue,
        }
    }
}
