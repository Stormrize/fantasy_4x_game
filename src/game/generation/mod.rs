use crate::game::generation::races_generation::players_or_bots::generate_players_positions::PlayersTerrainData;
use crate::game::generation::races_generation::players_or_bots::voronoi_diagram::PlayerInfluenceMap;
use crate::game::generation::terrain_generator::PerlinPlaceholder;
use crate::world::game_config::GameConfig;
use crate::world::game_resources::buildings_map::BuildingMap;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::{GlobalTerrainMap, TerrainMap};
use crate::world::game_resources::unit_map::UnitsMap;
use bevy::prelude::{Commands, Res, ResMut};

mod fog_generator;
pub mod races_generation;
pub mod resource_generator;
pub mod terrain_generator;
mod units_generator;

pub fn insert_map_resources(commands: &mut Commands) {
    // TODO можно тут типа инициализировать struct для разных цивилизаций и потом откинуть так не нужны больше в рам

    let config = GameConfig::default();
    let perlin = PerlinPlaceholder::new(&config);
    let global_terrain = GlobalTerrainMap::new(&config, &perlin);
    let capital_coords =
        PlayersTerrainData::calculate_player_positions(&config, &global_terrain).unwrap();
    let influence_map =
        PlayerInfluenceMap::voronoi_diagram_for_players(&config, &capital_coords, &perlin);
    let terrain = TerrainMap::new(&config, &perlin, &global_terrain, &influence_map);
    let resources = ResourceMap::new(&config, &terrain);
    let cities = BuildingMap::new(&config, &capital_coords, &terrain);
    let units = UnitsMap::new(&config, &capital_coords);
    let fog = FogMap::new(&config);

    commands.insert_resource(config);
    commands.insert_resource(terrain);
    commands.insert_resource(resources);
    commands.insert_resource(cities);
    commands.insert_resource(units);
    commands.insert_resource(fog);
}

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
