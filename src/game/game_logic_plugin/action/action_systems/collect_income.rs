use crate::game::game_logic_plugin::ai::profile::AiProfile;
use crate::game::game_logic_plugin::events_resources_structs::action_messages::EndTurn;
use crate::world::components::{Food, Gold, Id, Science};
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::building::{BuildingLevel, Levels};
use crate::world::turn_manager::TurnManager;
use bevy::prelude::{MessageReader, Query, Res, ResMut};

/// Регенерация глобальной маны за ход.
const MANA_REGENERATION: u16 = 10;

/// Когда игрок заканчивает ход, собирает пассивный доход с его зданий
/// и вычитает золото на их содержание.
pub fn collect_income(
    mut turn_ended: MessageReader<EndTurn>,
    turn_manager: Res<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    buildings_map: Res<BuildingMap>,
    buildings_query: Query<(&Id, &BuildingLevel, &Levels)>,
    mut players: ResMut<Players>,
) {
    if turn_ended.read().count() == 0 {
        return;
    }

    let player = turn_manager.current_player;
    let multiplier = AiProfile::income_multiplier_for(&game_logic_config, player);

    let mut gold: u32 = 0;
    let mut food: u32 = 0;
    let mut science: u32 = 0;
    let mut maintenance: u32 = 0;

    for slot in &buildings_map.map {
        let Some(entity) = *slot else { continue };
        let Ok((building_id, level, levels)) = buildings_query.get(entity) else {
            continue;
        };
        if *building_id != player.id {
            continue;
        }
        let stats = levels.stats(*level);
        gold += stats.gold() as u32;
        food += stats.food() as u32;
        science += stats.science() as u32;
        maintenance += stats.maintenance() as u32;
    }

    gold = ((gold as f32) * multiplier).round() as u32;
    food = ((food as f32) * multiplier).round() as u32;
    science = ((science as f32) * multiplier).round() as u32;

    if let Some(player) = players.0.iter_mut().find(|p| p.id == player.id) {
        player.gold = player
            .gold
            .saturating_add(gold.saturating_sub(maintenance) as Gold);
        player.food = player.food.saturating_add(food as Food);
        player.science = player.science.saturating_add(science as Science);
        player.current_mana = player
            .current_mana
            .saturating_add(MANA_REGENERATION)
            .min(player.maximum_mana);
    }
}
