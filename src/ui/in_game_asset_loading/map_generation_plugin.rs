// use crate::game::generation::{
//     races_generation::players_or_bots::{
//         generate_players_positions::PlayersTerrainData, voronoi_diagram::PlayerInfluenceMap,
//     },
//     terrain_generator::PerlinPlaceholder,
// };
// use crate::ui::menus::AppState;
// use crate::world::{
//     game_configs::{ConfigType, GenerationConfig},
//     game_resources::{
//         buildings_map::BuildingMap, fog_map::FogMap, resource_map::ResourceMap,
//         terrain_map::TerrainMap, unit_map::UnitsMap,
//     },
// };
// use bevy::app::App;
// use bevy::pbr::LightmapPlugin;
// use bevy::prelude::{Commands, IntoScheduleConfigs, NextState, OnEnter, Plugin, Res, ResMut, Resource, States, Update, in_state, AppExtStates};
// use bevy::tasks::futures_lite::future;
// use bevy::tasks::{AsyncComputeTaskPool, Task};
//
// // loading states
// #[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// pub(super) enum MapGenerationLoadingState {
//     #[default]
//     Loading,
//     Ready,
// }
//
// #[derive(Resource)]
// struct MapGenerationTask {
//     pub task: Task<GeneratedMapResources>,
// }
//
// struct GeneratedMapResources {
//     pub terrain: TerrainMap,
//     pub resources: ResourceMap,
//     pub cities: BuildingMap,
//     pub units: UnitsMap,
//     pub fog: FogMap,
// }
// fn generate_map_resources(config: &GenerationConfig) -> GeneratedMapResources {
//     let perlin = PerlinPlaceholder::new(config);
//     let mut terrain = TerrainMap::new_global_terrain_map(config, &perlin);
//     let player_terrain_data =
//         PlayersTerrainData::calculate_player_positions_flood_fill(config, &terrain).unwrap();
//     let influence_map =
//         PlayerInfluenceMap::voronoi_diagram_for_players(config, &player_terrain_data, &perlin);
//     terrain.final_terrain_map(config, &perlin, &influence_map);
//     let resources = ResourceMap::new(config, &terrain);
//     let cities = BuildingMap::new(config, &player_terrain_data, &terrain);
//     let units = UnitsMap::new(config, &player_terrain_data);
//     let fog = FogMap::new(config, &player_terrain_data);
//
//     GeneratedMapResources {
//         terrain,
//         resources,
//         cities,
//         units,
//         fog,
//     }
// }
//
// fn start_map_generation(mut commands: Commands, config: Res<ConfigType>) {
//     match &*config {
//         ConfigType::Local(config) => {
//             let generation_config = GenerationConfig::from_player_config(config);
//
//             let task = AsyncComputeTaskPool::get()
//                 .spawn(async move { generate_map_resources(&generation_config) });
//
//             commands.insert_resource(MapGenerationTask { task });
//         }
//         ConfigType::Multiplayer(config) => {
//             // todo send a request to a server
//         }
//     }
// }
//
// fn poll_map_generation(mut commands: Commands, task: &mut MapGenerationTask) -> bool {
//     if let Some(generated) = future::block_on(future::poll_once(&mut task.task)) {
//         commands.insert_resource(generated.terrain);
//         commands.insert_resource(generated.resources);
//         commands.insert_resource(generated.cities);
//         commands.insert_resource(generated.units);
//         commands.insert_resource(generated.fog);
//
//         commands.remove_resource::<MapGenerationTask>();
//         true
//     } else {
//         false
//     }
// }
//
// fn map_ready(
//     commands: Commands,
//     mut task: ResMut<MapGenerationTask>,
//     mut next_state: ResMut<NextState<MapGenerationLoadingState>>,
// ) {
//     if poll_map_generation(commands, &mut *task) {
//         println!("map ready");
//         next_state.set(MapGenerationLoadingState::Ready);
//     }
// }
//
// pub(super) struct GameMapGenerationPlugin;
//
// impl Plugin for GameMapGenerationPlugin {
//     fn build(&self, app: &mut App) {
//         app.init_state::<MapGenerationLoadingState>()
//             .add_systems(OnEnter(AppState::InGameAssetLoading), start_map_generation)
//             .add_systems(
//                 Update,
//                 map_ready
//                     .run_if(in_state(MapGenerationLoadingState::Loading))
//                     .run_if(in_state(AppState::InGameAssetLoading)),
//             );
//     }
// }
