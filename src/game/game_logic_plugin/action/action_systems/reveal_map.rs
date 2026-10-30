use crate::game::game_logic_plugin::events_resources_structs::action_messages::RevealMap;
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::data::resource::RareKind;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::turn_manager::TurnManager;
use bevy::prelude::{MessageReader, Res, ResMut};

/// Применяет свиток открытия местности из инвентаря текущего игрока:
/// навсегда раскрывает местность вокруг `center` на радиус силы свитка.
/// Знание о местности сохраняется (в отличие от обычной видимости).
pub fn reveal_map(
    mut reveal_command: MessageReader<RevealMap>,
    turn_manager: Res<TurnManager>,
    game_logic_config: Res<GameLogicConfig>,
    mut fog_map: ResMut<FogMap>,
    mut players: ResMut<Players>,
) {
    let size = fog_map.size;

    for command in reveal_command.read() {
        let player_id = turn_manager.current_player.id;
        let center = command.center;
        if center.x < 0 || center.y < 0 || center.x as usize >= size || center.y as usize >= size {
            continue;
        }

        let Some(player) = players.0.iter_mut().find(|p| p.id == player_id) else {
            continue;
        };
        let Some(scroll_index) = player
            .inventory
            .iter()
            .position(|rare| rare.kind == RareKind::RevealScroll)
        else {
            continue;
        };
        let radius = player.inventory[scroll_index].power as isize;
        player.inventory.remove(scroll_index);

        for dr in -radius..=radius {
            for dc in -radius..=radius {
                let r = center.y as isize + dr;
                let c = center.x as isize + dc;
                if r < 0 || c < 0 || r as usize >= size || c as usize >= size {
                    continue;
                }
                fog_map.set_visibility(
                    r as usize,
                    c as usize,
                    player_id,
                    game_logic_config.player.id,
                    true,
                    true,
                );
            }
        }
    }
}
