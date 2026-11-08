use bevy::math::IVec2;

use crate::game::game_logic_plugin::action::action_systems::{ActionCtx, tile};
use crate::game::game_logic_plugin::ai::profile::AiProfile;
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderGatherEntity;
use crate::world::components::{Food, Gold, Science};
use crate::world::game_resources::data::resource::{CommonKind, ResourceType};

/// A helper standing on a resource tile picks it up.
pub fn gather_entity(ctx: &mut ActionCtx, pos: IVec2) {
    let Some((x, y)) = tile(pos, ctx.resource_map.size) else {
        return;
    };
    let Some(helper) = ctx.units_map.get(x, y) else {
        return;
    };
    let Ok((helper_id, ..)) = ctx.helpers.get(helper) else {
        return;
    };
    let helper_id = *helper_id;
    if helper_id != ctx.turn_manager.current_player.id {
        return;
    }

    let Some(resource) = ctx.resource_map.get(x, y).cloned() else {
        return;
    };
    let multiplier = AiProfile::income_multiplier_for(&ctx.config, ctx.turn_manager.current_player);
    let Some(player) = ctx.players.0.iter_mut().find(|p| p.id == helper_id) else {
        return;
    };

    match &resource {
        ResourceType::Common(common) => {
            // Common resources can only be taken by their owner.
            if common.owner_id != Some(helper_id) {
                return;
            }
            let amount = ((common.amount as f32) * multiplier).round() as u32;
            match common.kind {
                CommonKind::Gold => player.gold = player.gold.saturating_add(amount as Gold),
                CommonKind::Food => player.food = player.food.saturating_add(amount as Food),
                CommonKind::Science => {
                    player.science = player.science.saturating_add(amount as Science)
                }
            }
        }
        ResourceType::Rare(rare) => player.inventory.push(rare.clone()),
    }

    ctx.resource_map.set(x, y, None);
    ctx.render_gather
        .write(RenderGatherEntity { entity: helper });
}
