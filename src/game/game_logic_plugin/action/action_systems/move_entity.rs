use crate::game::game_logic_plugin::action::action_systems::fog_update::fog_update;
use crate::game::game_logic_plugin::action::action_systems::{ActionCtx, tile};
use crate::game::game_logic_plugin::action::rules::move_rules::{
    check_move_validity, is_players_turn, is_players_unit,
};
use crate::game::game_logic_plugin::events_resources_structs::action_messages::{
    Clear, RevealSource,
};
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderMoveEntity;
use crate::world::components::GameMode;
use bevy::math::IVec2;

/// Moves a unit from `from` to `to`, spending movement points.
pub fn move_entity(ctx: &mut ActionCtx, from: IVec2, to: IVec2) {
    let size = ctx.terrain.size;
    let (Some((fx, fy)), Some((tx, ty))) = (tile(from, size), tile(to, size)) else {
        return;
    };

    let Some(unit) = ctx.units_map.get(fx, fy) else {
        ctx.clear.write(Clear);
        return;
    };
    let Ok((id, mut points)) = ctx.movers.get_mut(unit) else {
        ctx.clear.write(Clear);
        return;
    };

    if !is_players_unit(*id, ctx.turn_manager.current_player) {
        println!("Not your race");
        ctx.clear.write(Clear);
        return;
    }

    if !is_players_turn(ctx.config.player, ctx.turn_manager.current_player) {
        println!("Not your turn");
        ctx.clear.write(Clear);
        return;
    }

    let Some(cost) = check_move_validity(&ctx.cost_grid, &ctx.units_map, points.0, from, to) else {
        println!("Not valid move");
        ctx.clear.write(Clear);
        return;
    };

    if matches!(ctx.turn_manager.game_mode, GameMode::Online) {
        // TODO: send http request
    }

    ctx.units_map.set(fx, fy, None);
    ctx.units_map.set(tx, ty, Some(unit));
    ctx.render_move.write(RenderMoveEntity {
        entity: unit,
        current_pos: from,
        new_pos: to,
    });
    points.0 -= cost;
    ctx.clear.write(Clear);

    // Reveal around the NEW tile (the old one is empty now).
    fog_update(ctx, to, RevealSource::Unit, true);
}
