use bevy::prelude::{Commands, Res, ResMut};
use crate::generation::races_generation::players_or_bots::generate_players_positions::PlayerTerrainData;
use crate::generation::races_generation::players_or_bots::voronoi_diagram::PlayerInfluenceMap;
use crate::generation::terrain_generator::PerlinPlaceholder;
use crate::world::buildings_map::BuildingMap;
use crate::world::fog_map::FogMap;
use crate::world::resource_map::ResourceMap;
use crate::world::terrain_config::TerrainConfig;
use crate::world::terrain_map::{GlobalTerrainMap, TerrainMap};
use crate::world::unit_map::UnitsMap;

pub mod terrain_generator;
pub mod resource_generator;
pub mod races_generation;
mod fog_generator;
mod units_generator;

pub fn insert_map_resources(mut commands: Commands) {
    let config = TerrainConfig::default();
    let perlin = PerlinPlaceholder::new(&config);
    let global_terrain = GlobalTerrainMap::new(&config, &perlin);
    let capital_coords =
        PlayerTerrainData::calculate_player_positions(&config, &global_terrain).unwrap();
    let influence_map =
        PlayerInfluenceMap::voronoi_diagram_for_players(&config, &capital_coords, &perlin);
    let terrain = TerrainMap::new(&config, &perlin, &global_terrain, &influence_map);
    let resources = ResourceMap::new(&config, &terrain);
    let cities = BuildingMap::new(&config, &capital_coords, &terrain);
    let units = UnitsMap::new(&config);
    let fog = FogMap::new(&config);

    commands.insert_resource(config);
    commands.insert_resource(terrain);
    commands.insert_resource(resources);
    commands.insert_resource(cities);
    commands.insert_resource(units);
    commands.insert_resource(fog);
}

pub fn regenerate_map(
    config: Res<TerrainConfig>,
    mut terrain: ResMut<TerrainMap>,
    mut resources: ResMut<ResourceMap>,
    mut cities: ResMut<BuildingMap>,
) {
    let next_perlin = PerlinPlaceholder::new(&config);
    let next_global_terrain = GlobalTerrainMap::new(&config, &next_perlin);
    let next_capital_coords =
        PlayerTerrainData::calculate_player_positions(&config, &next_global_terrain).unwrap();
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
