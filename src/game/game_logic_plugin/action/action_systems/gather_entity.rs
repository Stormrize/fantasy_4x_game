use crate::game::game_logic_plugin::ai::profile::AiProfile;
use crate::game::game_logic_plugin::events_resources_structs::action_messages::Gather;
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderGatherEntity;
use crate::world::components::{Food, Gold, Id, Science};
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::data::resource::{CommonKind, ResourceType};
use crate::world::game_resources::data::unit::HelperBasicStats;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;
use bevy::prelude::{MessageReader, MessageWriter, Query, Res, ResMut};

/// Строитель, стоящий на клетке ресурса, собирает его:
/// обычные ресурсы дают мгновенный доход, редкие — попадают в инвентарь.
#[allow(clippy::too_many_arguments)]
pub fn gather_entity(
    mut gather_command: MessageReader<Gather>,
    turn_manager: Res<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    mut resource_map: ResMut<ResourceMap>,
    units_map: Res<UnitsMap>,
    builders_query: Query<(&Id, &HelperBasicStats)>,
    mut players: ResMut<Players>,
    mut render_gather: MessageWriter<RenderGatherEntity>,
) {
    let size = resource_map.size;

    for gather in gather_command.read() {
        let pos = gather.0;
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= size || pos.y as usize >= size {
            continue;
        }
        let (x, y) = (pos.x as usize, pos.y as usize);

        let Some(builder_entity) = units_map.get(x, y) else {
            continue;
        };
        let Ok((builder_id, _helper)) = builders_query.get(builder_entity) else {
            continue;
        };
        if *builder_id != turn_manager.current_player.id {
            continue;
        }

        let Some(resource) = resource_map.get(x, y).cloned() else {
            continue;
        };
        let multiplier =
            AiProfile::income_multiplier_for(&game_logic_config, turn_manager.current_player);

        match &resource {
            ResourceType::Common(common) => {
                let amount = ((common.amount as f32) * multiplier).round() as u32;
                if let Some(player) = players.0.iter_mut().find(|p| p.id == *builder_id) {
                    let Some(owner_id) = common.owner_id else {
                        continue;
                    };

                    if owner_id != *builder_id {
                        continue;
                    }

                    match common.kind {
                        CommonKind::Gold => {
                            player.gold = player.gold.saturating_add(amount as Gold);
                        }
                        CommonKind::Food => {
                            player.food = player.food.saturating_add(amount as Food);
                        }
                        CommonKind::Science => {
                            player.science = player.science.saturating_add(amount as Science);
                        }
                    }
                }
            }
            ResourceType::Rare(rare) => {
                if let Some(player) = players.0.iter_mut().find(|p| p.id == *builder_id) {
                    player.inventory.push(rare.clone());
                }
            }
        }

        resource_map.set(x, y, None);
        render_gather.write(RenderGatherEntity {
            entity: builder_entity,
        });
    }
}
