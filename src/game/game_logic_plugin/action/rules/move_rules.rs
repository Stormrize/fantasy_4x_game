use crate::world::components::Id;
use crate::world::game_configs::Player;
use bevy::math::IVec2;

pub fn is_players_unit(unit_id: Id, player: Player) -> bool {
    unit_id == player.id
}

pub fn check_move_validity(
    movement_points: u16,
    current_position: IVec2,
    target_position: IVec2,
) -> bool {
    // todo biome could be more difficult to walk
    let x_difference = (target_position.x - current_position.x).abs() as u32;
    let y_difference = (target_position.y - current_position.y).abs() as u32;

    let required_points = x_difference.max(y_difference);

    required_points <= movement_points as u32
}
