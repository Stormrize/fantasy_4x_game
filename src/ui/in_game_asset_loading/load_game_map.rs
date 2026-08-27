use crate::game::generation::{
    races_generation::players_or_bots::{
        generate_players_positions::PlayersTerrainData, voronoi_diagram::PlayerInfluenceMap,
    },
    terrain_generator::PerlinPlaceholder,
};
use crate::world::{
    game_configs::{ConfigType, GenerationConfig},
    game_resources::{
        buildings_map::BuildingMap,
        fog_map::FogMap,
        resource_map::ResourceMap,
        terrain_map::{GlobalTerrainMap, TerrainMap},
        unit_map::UnitsMap,
    },
};
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

fn generate_map_resources(config: &GenerationConfig) -> GeneratedMapResources {
    let perlin = PerlinPlaceholder::new(config);
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

pub(super) fn start_map_generation(mut commands: Commands, config: Res<ConfigType>) {
    match &*config {
        ConfigType::Local(config) => {
            let generation_config = GenerationConfig::from(config);

            let task = AsyncComputeTaskPool::get()
                .spawn(async move { generate_map_resources(&generation_config) });

            commands.insert_resource(MapGenerationTask { task });
        }
        ConfigType::Multiplayer(config) => {
            // todo send a request to a server
        }
    }
}
