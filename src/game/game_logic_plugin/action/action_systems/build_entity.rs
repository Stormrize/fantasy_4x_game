use bevy::math::IVec2;

use crate::game::game_logic_plugin::action::action_systems::fog_update::fog_update;
use crate::game::game_logic_plugin::action::action_systems::{ActionCtx, tile};
use crate::game::game_logic_plugin::events_resources_structs::action_messages::RevealSource;
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderBuildEntity;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::buffs_and_debuffs::{Buffs, Debuffs};
use crate::world::game_resources::data::building::{
    Building, BuildingData, BuildingLevel, BuildingType,
};

pub fn building_for_biome(biome: BiomeType) -> Option<BuildingType> {
    match biome {
        BiomeType::SmallBiome => Some(BuildingType::House),
        BiomeType::MediumBiome => Some(BuildingType::MediumBuilding),
        BiomeType::LargeBiome => Some(BuildingType::LargeBuilding),
        _ => None,
    }
}

pub fn biome_matches(building_type: BuildingType, biome: BiomeType) -> bool {
    match building_type {
        BuildingType::House | BuildingType::ResearchCenter | BuildingType::SpawningBuilding => {
            biome == BiomeType::SmallBiome
        }
        BuildingType::MediumBuilding => biome == BiomeType::MediumBiome,
        BuildingType::LargeBuilding => biome == BiomeType::LargeBiome,
        _ => false,
    }
}

/// Builder builds a building on its own tile, spending one build charge and gold.
/// `building_type: None` means the default building for the tile's biome.
pub fn build_entity(ctx: &mut ActionCtx, pos: IVec2, building_type: Option<BuildingType>) {
    let Some((x, y)) = tile(pos, ctx.terrain.size) else {
        return;
    };
    let Some(builder) = ctx.units_map.get(x, y) else {
        return;
    };
    let Ok((id, race, mut stats)) = ctx.helpers.get_mut(builder) else {
        return;
    };
    let builder_id = *id;
    if builder_id != ctx.turn_manager.current_player.id || stats.build_gather_points == 0 {
        return;
    }

    let biome = ctx.terrain.get_biome_type(x, y);
    let Some(building_type) = building_type.or_else(|| building_for_biome(biome)) else {
        return;
    };
    if !biome_matches(building_type, biome) || ctx.buildings_map.is_some(x, y) {
        return;
    }

    let Some(building_data) = BuildingData::from_type(building_type) else {
        return;
    };
    let cost = building_data.stats(BuildingLevel::Standard).upgrade_cost();

    let Some(player) = ctx.players.0.iter_mut().find(|p| p.id == builder_id) else {
        return;
    };
    if player.gold < cost {
        return;
    }
    player.gold -= cost;

    let entity = ctx
        .commands
        .spawn(Building {
            id: builder_id,
            player_race: race.clone(),
            level: BuildingLevel::Standard,
            building_data,
            active_buffs: Buffs::default(),
            active_debuffs: Debuffs::default(),
        })
        .id();
    ctx.buildings_map.set(x, y, Some(entity));
    ctx.render_build.write(RenderBuildEntity { entity });

    stats.build_gather_points -= 1;
    if stats.build_gather_points == 0 {
        // The builder is used up and disappears.
        ctx.units_map.set(x, y, None);
        ctx.commands.entity(builder).despawn();
    }
    fog_update(ctx, pos, RevealSource::Building, true);
}
