use crate::game::game_logic_plugin::events_resources_structs::action_messages::Build;
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderBuildEntity;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::Players;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::buffs_and_debuffs::{Buffs, Debuffs};
use crate::world::game_resources::data::building::{
    Building, BuildingData, BuildingLevel, BuildingType,
};
use crate::world::game_resources::data::unit::HelperBasicStats;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;
use bevy::prelude::{Commands, MessageReader, MessageWriter, Query, Res, ResMut};

/// Здание по умолчанию для биома (когда тип не указан явно).
pub fn building_for_biome(biome: BiomeType) -> Option<BuildingType> {
    match biome {
        BiomeType::SmallBiome => Some(BuildingType::House),
        BiomeType::MediumBiome => Some(BuildingType::MediumBuilding),
        BiomeType::LargeBiome => Some(BuildingType::LargeBuilding),
        _ => None,
    }
}

/// Разрешено ли строить здание такого типа на этом биоме.
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

/// Строитель тратит заряд и возводит здание на клетке, на которой стоит.
/// Когда заряды кончаются — строитель исчезает.
#[allow(clippy::too_many_arguments)]
pub fn build_entity(
    mut build_command: MessageReader<Build>,
    turn_manager: Res<TurnManager>,
    terrain: Res<TerrainMap>,
    mut units_map: ResMut<UnitsMap>,
    mut buildings_map: ResMut<BuildingMap>,
    mut builders_query: Query<(&Id, &PlayerRace, &mut HelperBasicStats)>,
    mut players: ResMut<Players>,
    mut commands: Commands,
    mut render_build: MessageWriter<RenderBuildEntity>,
) {
    let size = terrain.size;

    for build in build_command.read() {
        let pos = build.entity_pos;
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= size || pos.y as usize >= size {
            continue;
        }
        let (x, y) = (pos.x as usize, pos.y as usize);

        let Some(builder_entity) = units_map.get(x, y) else {
            continue;
        };

        let charges_left = {
            let Ok((builder_id, builder_race, mut stats)) = builders_query.get_mut(builder_entity)
            else {
                continue;
            };
            if *builder_id != turn_manager.current_player.id {
                continue;
            }
            if stats.build_gather_points == 0 {
                continue;
            }

            let biome = terrain.get_biome_type(x, y);
            let Some(building_type) = build.building_type.or_else(|| building_for_biome(biome))
            else {
                continue;
            };
            if !biome_matches(building_type, biome) {
                continue;
            }
            if buildings_map.is_some(x, y) {
                continue;
            }

            let Some(building_data) = BuildingData::from_type(building_type) else {
                continue;
            };
            let cost = building_data.stats(BuildingLevel::Standard).upgrade_cost();

            let Some(player) = players.0.iter_mut().find(|p| p.id == *builder_id) else {
                continue;
            };
            if player.gold < cost {
                continue;
            }
            player.gold -= cost;

            let entity = commands
                .spawn(Building {
                    id: *builder_id,
                    player_race: builder_race.clone(),
                    level: BuildingLevel::Standard,
                    building_data,
                    active_buffs: Buffs::default(),
                    active_debuffs: Debuffs::default(),
                })
                .id();
            buildings_map.set(x, y, Some(entity));
            render_build.write(RenderBuildEntity { entity });

            stats.build_gather_points -= 1;
            stats.build_gather_points
        };

        if charges_left == 0 {
            // Строитель исчерпал заряды и исчезает.
            units_map.set(x, y, None);
            commands.entity(builder_entity).despawn();
        }
    }
}
