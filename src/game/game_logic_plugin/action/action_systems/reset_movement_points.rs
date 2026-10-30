use crate::world::components::Id;
use crate::world::game_resources::data::unit::{MovementPoints, MovementPointsMax};
use crate::world::turn_manager::TurnManager;
use bevy::prelude::{DetectChanges, Query, Res};

/// В начале хода игрока восстанавливает очки движения его юнитов.
pub fn reset_movement_points(
    turn_manager: Res<TurnManager>,
    mut units_query: Query<(&Id, &MovementPointsMax, &mut MovementPoints)>,
) {
    if !turn_manager.is_changed() {
        return;
    }
    let current_id = turn_manager.current_player.id;

    for (id, max, mut movement) in units_query.iter_mut() {
        if *id == current_id {
            movement.0 = max.0;
        }
    }
}
