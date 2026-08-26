use crate::game::generation::races_generation::players_or_bots::generate_players_positions::PlayersTerrainData;
use crate::game::generation::races_generation::players_or_bots::voronoi_diagram::PlayerInfluenceMap;
use crate::game::generation::terrain_generator::PerlinPlaceholder;
use crate::world::game_config::GameConfig;
use crate::world::game_resources::buildings_map::BuildingMap;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::{GlobalTerrainMap, TerrainMap};
use crate::world::game_resources::unit_map::UnitsMap;
use bevy::prelude::{Commands, Component, Res, ResMut, Resource};
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};

mod fog_generator;
pub mod races_generation;
pub mod resource_generator;
pub mod terrain_generator;
mod units_generator;

pub fn regenerate_map(
    config: Res<GameConfig>,
    mut terrain: ResMut<TerrainMap>,
    mut resources: ResMut<ResourceMap>,
    mut cities: ResMut<BuildingMap>,
) {
    let next_perlin = PerlinPlaceholder::new(&config);
    let next_global_terrain = GlobalTerrainMap::new(&config, &next_perlin);
    let next_capital_coords =
        PlayersTerrainData::calculate_player_positions(&config, &next_global_terrain).unwrap();
    let next_influence_map = PlayerInfluenceMap::voronoi_diagram_for_players(
        &config,
        &next_capital_coords,
        &next_perlin,
    );
    let next_terrain = TerrainMap::new(
        &config,
        &next_perlin,
        &next_global_terrain,
        &next_influence_map,
    );
    let next_resources = ResourceMap::new(&config, &next_terrain);
    let next_cities = BuildingMap::new(&config, &next_capital_coords, &next_terrain);

    *terrain = next_terrain;
    *resources = next_resources;
    *cities = next_cities;
}
