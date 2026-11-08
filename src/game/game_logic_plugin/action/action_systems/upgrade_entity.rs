use bevy::math::IVec2;

use crate::game::game_logic_plugin::action::action_systems::{ActionCtx, tile};
use crate::game::game_logic_plugin::events_resources_structs::components::TileLayer;
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderBuildEntity;

/// Upgrades a building or a unit of the current player, paying gold.
pub fn upgrade_entity(ctx: &mut ActionCtx, pos: IVec2, layer: TileLayer) {
    let Some((x, y)) = tile(pos, ctx.terrain.size) else {
        return;
    };
    match layer {
        TileLayer::Building => upgrade_building(ctx, x, y),
        TileLayer::Unit => upgrade_unit(ctx, x, y),
        _ => {}
    }
}

fn upgrade_building(ctx: &mut ActionCtx, x: usize, y: usize) {
    let Some(entity) = ctx.buildings_map.get(x, y) else {
        return;
    };
    let Ok((id, _, _, mut level, levels)) = ctx.buildings.get_mut(entity) else {
        return;
    };
    if *id != ctx.turn_manager.current_player.id {
        return;
    }
    let Some(next) = level.next() else {
        return;
    };
    let cost = levels.stats(*level).upgrade_cost();

    let Some(player) = ctx.players.0.iter_mut().find(|p| p.id == *id) else {
        return;
    };
    if player.gold < cost {
        return;
    }
    player.gold -= cost;

    *level = next;
    ctx.render_build.write(RenderBuildEntity { entity });
}

fn upgrade_unit(ctx: &mut ActionCtx, x: usize, y: usize) {
    let Some(entity) = ctx.units_map.get(x, y) else {
        return;
    };
    let Ok((id, mut level, level_cost)) = ctx.leveled_units.get_mut(entity) else {
        return;
    };
    if *id != ctx.turn_manager.current_player.id {
        return;
    }

    let Some(next) = level.next() else {
        return;
    };

    // Number of levels is defined by the length of `LevelCost`.
    let Some(level_cost) = level_cost.level_cost(*level) else {
        return;
    };

    let cost = level_cost.new_level_cost;

    let Some(player) = ctx.players.0.iter_mut().find(|p| p.id == *id) else {
        return;
    };
    if player.gold < cost {
        return;
    }
    player.gold -= cost;

    *level = next;
}
