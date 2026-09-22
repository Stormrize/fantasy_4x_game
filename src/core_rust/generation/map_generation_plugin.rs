use std::fmt::Debug;

use bevy::app::App;
use bevy::prelude::{
    AppExtStates, Commands, Entity, IntoScheduleConfigs, NextState, OnEnter, Plugin, Res, ResMut,
    Resource, States, Update, in_state,
};
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};

use crate::core_rust::generation::races_generation::capitals_generation::CapitalsPos;
use crate::core_rust::generation::races_generation::capitals_generation::voronoi_diagram::PlayerInfluenceMap;
use crate::core_rust::generation::races_generation::generate_cities::BuildingMapNoBevy;
use crate::core_rust::generation::terrain_generator::PerlinPlaceholder;
use crate::core_rust::generation::units_generator::generate_units::UnitsMapNoBevy;
use crate::world::game_configs::{ConfigType, GameMode};
use crate::world::{
    game_configs::GenerationConfig,
    game_resources::{
        building_map::BuildingMap,
        data::{
            buffs_and_debuffs::{Buffs, Debuffs},
            building::{
                BARBARIAN_CITY, Building, BuildingLevel, BuildingType, CAPITAL, CAPTURED_CITY,
                HOUSE, LARGE_BUILDING, MEDIUM_BUILDING, RESEARCH_CENTER, SPAWNING_BUILDING,
            },
            unit::{HELPER, Helper, UnitType},
        },
        fog_map::FogMap,
        resource_map::ResourceMap,
        terrain_map::TerrainMap,
        unit_map::UnitsMap,
    },
};

#[derive(Resource)]
struct MapGenerationTask {
    pub task: Task<GeneratedMapResources>,
}

struct GeneratedMapResources {
    pub terrain: TerrainMap,
    pub resources: ResourceMap,
    pub fog: FogMap,
    pub buildings: BuildingMapNoBevy,
    pub units: UnitsMapNoBevy,
}
fn generate_map_resources(config: &GenerationConfig) -> GeneratedMapResources {
    let perlin = PerlinPlaceholder::new(config);
    let mut terrain = TerrainMap::new_global_terrain_map(config, &perlin);
    let capitals_pos =
        CapitalsPos::calculate_player_positions_flood_fill(config, &terrain).unwrap();
    let influence_map =
        PlayerInfluenceMap::voronoi_diagram_for_players(config, &capitals_pos, &perlin);
    terrain.final_terrain_map(config, &perlin, &influence_map);
    let resources = ResourceMap::new(config, &terrain);
    let fog = FogMap::new(config, &capitals_pos);
    let buildings = BuildingMapNoBevy::new(config, &capitals_pos, &terrain);
    let units = UnitsMapNoBevy::new(config, &capitals_pos);

    GeneratedMapResources {
        terrain,
        resources,
        fog,
        buildings,
        units,
    }
}

fn start_map_generation(mut commands: Commands, config_type: Res<ConfigType>) {
    match &config_type.game_mode {
        GameMode::Local(config) => {
            let generation_config = GenerationConfig::new(&config_type.player_config, config);
            let task = AsyncComputeTaskPool::get()
                .spawn(async move { generate_map_resources(&generation_config) });

            commands.insert_resource(MapGenerationTask { task });
            commands.remove_resource::<GenerationConfig>();
        }
        GameMode::Multiplayer(_) => {
            // http request config body
        }
    }
}

fn poll_map_generation(mut commands: Commands, task: &mut MapGenerationTask) -> bool {
    if let Some(generated) = future::block_on(future::poll_once(&mut task.task)) {
        commands.insert_resource(generated.terrain);
        commands.insert_resource(generated.resources);
        commands.insert_resource(generated.fog);
        let units_map = spawn_units(generated.units, &mut commands);
        commands.insert_resource(units_map);
        let buildings_map = spawn_building(generated.buildings, &mut commands);
        commands.insert_resource(buildings_map);
        commands.remove_resource::<MapGenerationTask>();
        commands.remove_resource::<GenerationConfig>();
        true
    } else {
        false
    }
}

fn spawn_units(units_map_no_bevy: UnitsMapNoBevy, commands: &mut Commands) -> UnitsMap {
    let mut units_map: Vec<Option<Entity>> = Vec::with_capacity(units_map_no_bevy.size);
    for unit in units_map_no_bevy.units.into_iter() {
        let Some(unit) = unit else {
            units_map.push(None);
            continue;
        };
        match unit.unit_type {
            UnitType::Helper => {
                let unit = commands
                    .spawn(Helper {
                        player_id: unit.id,
                        player_race: unit.player_race,
                        unit_data: HELPER,
                        active_buffs: Buffs::default(),
                        active_debuffs: Debuffs::default(),
                    })
                    .id();
                units_map.push(Some(unit));
            }
            _ => unreachable!(),
        }
    }
    UnitsMap {
        map: units_map,
        size: units_map_no_bevy.size,
    }
}

fn spawn_building(
    buildings_map_no_bevy: BuildingMapNoBevy,
    commands: &mut Commands,
) -> BuildingMap {
    let mut buildings_map = Vec::<Option<Entity>>::with_capacity(buildings_map_no_bevy.size);

    for building in buildings_map_no_bevy.buildings.into_iter() {
        let Some(building) = building else {
            buildings_map.push(None);
            continue;
        };
        let building_data = match building.building_type {
            BuildingType::Capital => CAPITAL,
            BuildingType::CapturedCity => CAPTURED_CITY,
            BuildingType::BarbarianCity => BARBARIAN_CITY,
            BuildingType::SpawningBuilding => SPAWNING_BUILDING,
            BuildingType::House => HOUSE,
            BuildingType::MediumBuilding => MEDIUM_BUILDING,
            BuildingType::LargeBuilding => LARGE_BUILDING,
            BuildingType::ResearchCenter => RESEARCH_CENTER,
            _ => unreachable!(),
        };
        let building = commands
            .spawn(Building {
                id: building.id,
                player_race: building.player_race,
                level: BuildingLevel::Standard,
                building_data,
                active_buffs: Buffs::default(),
                active_debuffs: Debuffs::default(),
            })
            .id();
        buildings_map.push(Some(building));
    }
    BuildingMap {
        map: buildings_map,
        size: buildings_map_no_bevy.size,
    }
}

fn map_ready(
    commands: Commands,
    mut task: ResMut<MapGenerationTask>,
    mut next_state: ResMut<NextState<MapGenerationLoadingState>>,
) {
    if poll_map_generation(commands, &mut *task) {
        println!("map ready");
        next_state.set(MapGenerationLoadingState::Ready);
    }
}

// loading states
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MapGenerationLoadingState {
    Loading,
    #[default]
    Ready,
}

pub struct GameMapGenerationPlugin;

impl Plugin for GameMapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MapGenerationLoadingState>()
            .add_systems(
                OnEnter(MapGenerationLoadingState::Loading),
                start_map_generation,
            )
            .add_systems(
                Update,
                map_ready.run_if(in_state(MapGenerationLoadingState::Loading)),
            );
    }
}
