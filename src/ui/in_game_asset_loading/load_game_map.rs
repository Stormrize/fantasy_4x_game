use crate::game::generation::races_generation::players_or_bots::generate_players_positions::PlayersTerrainData;
use crate::game::generation::races_generation::players_or_bots::voronoi_diagram::PlayerInfluenceMap;
use crate::world::game_config::GameConfig;
use crate::world::game_resources::buildings_map::BuildingMap;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::{GlobalTerrainMap, TerrainMap};
use crate::world::game_resources::unit_map::UnitsMap;
use bevy::prelude::{Commands, Res, Resource};
use bevy::tasks::{AsyncComputeTaskPool, Task};

#[derive(Resource)]
pub(super) struct MapGenerationTask {
    pub task: Task<GeneratedMapResources>,
}

pub(super) struct GeneratedMapResources {
    pub terrain: TerrainMap,
    pub resources: ResourceMap,
    pub cities: BuildingMap,
    pub units: UnitsMap,
    pub fog: FogMap,
}

fn generate_map_resources(config: &GameConfig) -> GeneratedMapResources {
    let perlin = crate::game::generation::terrain_generator::PerlinPlaceholder::new(config);
    let global_terrain = GlobalTerrainMap::new(config, &perlin);
    let player_terrain_data =
        PlayersTerrainData::calculate_player_positions(config, &global_terrain).unwrap();
    let influence_map =
        PlayerInfluenceMap::voronoi_diagram_for_players(config, &player_terrain_data, &perlin);
    let terrain = TerrainMap::new(config, &perlin, &global_terrain, &influence_map);
    let resources = ResourceMap::new(config, &terrain);
    let cities = BuildingMap::new(config, &player_terrain_data, &terrain);
    let units = UnitsMap::new(config, &player_terrain_data);
    let fog = FogMap::new(config, &player_terrain_data);

    GeneratedMapResources {
        terrain,
        resources,
        cities,
        units,
        fog,
    }
}

pub(super) fn start_map_generation(mut commands: Commands, config: Res<GameConfig>) {
    let config = config.clone();
    let task = AsyncComputeTaskPool::get().spawn(async move { generate_map_resources(&config) });

    commands.insert_resource(MapGenerationTask { task });
}
