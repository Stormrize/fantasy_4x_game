use bevy::math::IVec2;

use crate::game::game_logic_plugin::action::action_systems::{ActionCtx, tile};
use crate::game::game_logic_plugin::events_resources_structs::action_messages::RevealSource;
use crate::world::components::BUILDING_SEE_RADIUS;
use crate::world::game_resources::data::resource::RareKind;

/// Updates fog around `center`.
/// Plain function: called from the router (`Action::RevealMap`) and from `move_entity`.
/// The source entity must already exist (be queryable).
pub fn fog_update(ctx: &mut ActionCtx, center: IVec2, source: RevealSource, full: bool) {
    let Some((x, y)) = tile(center, ctx.fog_map.size) else {
        return;
    };
    let local_id = ctx.config.player.id;

    match source {
        RevealSource::Unit => {
            let Some((owner, radius)) = ctx
                .units_map
                .get(x, y)
                .and_then(|e| ctx.sight.get(e).ok())
                .map(|(id, fov)| (*id, fov.0 as usize))
            else {
                return;
            };
            ctx.fog_map.clear_visibility();
            ctx.fog_map
                .reveal_area(x, y, radius, owner, local_id, true, full);
        }
        RevealSource::Building => {
            let Some(owner) = ctx
                .buildings_map
                .get(x, y)
                .and_then(|e| ctx.buildings.get(e).ok())
                .map(|(id, ..)| *id)
            else {
                return;
            };
            ctx.fog_map.clear_visibility();
            ctx.fog_map
                .reveal_area(x, y, BUILDING_SEE_RADIUS, owner, local_id, true, full);
        }
        RevealSource::RevealScroll => {
            // Uses up one scroll of the current player (replaces the old `reveal_map` system).
            // Does not clear existing visibility: a scroll only adds to it.
            let player_id = ctx.turn_manager.current_player.id;
            let Some(player) = ctx.players.0.iter_mut().find(|p| p.id == player_id) else {
                return;
            };
            let Some(index) = player
                .inventory
                .iter()
                .position(|rare| rare.kind == RareKind::RevealScroll)
            else {
                return;
            };
            let radius = player.inventory.remove(index).power as usize;
            ctx.fog_map
                .reveal_area(x, y, radius, player_id, local_id, true, full);
        }
    }
}

/// Debug: whole map is visible to the local player.
pub fn apply_debug_visibility(ctx: &mut ActionCtx) {
    if !ctx.fog_map.debug_full_visibility {
        return;
    }
    let size = ctx.fog_map.size;
    let local_id = ctx.config.player.id;
    for row in 0..size {
        for col in 0..size {
            ctx.fog_map
                .set_visibility(row, col, local_id, local_id, true, true);
        }
    }
}
