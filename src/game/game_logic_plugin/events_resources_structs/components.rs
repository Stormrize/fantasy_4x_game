use crate::world::{
    game_config::GameConfig,
    game_resources::{
        buildings_map::BuildingMap,
        data::{biome::BiomeType, buildings::BuildingType, resources::ResourceType, units::Unit},
        fog_map::FogMap,
        resource_map::ResourceMap,
        terrain_map::TerrainMap,
        unit_map::UnitsMap,
    },
};

use bevy::ecs::system::SystemParam;
use bevy::prelude::{IVec2, Res, Resource};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TileLayer {
    Terrain = 0,
    Resource = 1,
    Building = 2,
    Unit = 3,
    Fog = 4,
}

//---------------------------------------------------------------------------------------------------------------------------//

// #[derive(Resource, Default)]
// pub struct AlreadySelectedTileContent {
//     pub pos: Option<IVec2>,
//     pub layer: Option<TileLayer>,
//     pub biome: BiomeType,
//     pub building: BuildingType,
//     pub resource: ResourceType,
//     pub fog: bool,
// }
//
// impl AlreadySelectedTileContent {
//     pub fn clear(&mut self) {
//         self.pos = None;
//         self.layer = None;
//     }
// }

//---------------------------------------------------------------------------------------------------------------------------//
// чтобы чекать что на клетке

#[derive(Default)]
pub struct TileContent {
    pub biome: BiomeType,
    pub unit: Unit,
    pub building: BuildingType,
    pub resource: ResourceType,
    pub fog: bool,
}

impl TileContent {
    pub fn from_world(
        biome: BiomeType,
        unit: Unit,
        building: BuildingType,
        resource: ResourceType,
        fog: bool,
    ) -> Self {
        Self {
            biome,
            unit,
            building,
            resource,
            fog,
        }
    }

    pub fn get_top_layer(&self) -> TileLayer {
        if self.fog == true {
            TileLayer::Fog
        } else if self.unit != Unit::None {
            TileLayer::Unit
        } else if self.building != BuildingType::None {
            TileLayer::Building
        } else if self.resource != ResourceType::None {
            TileLayer::Resource
        } else {
            TileLayer::Terrain
        }
    }
}

#[derive(Resource, Default)]
pub struct AlreadySelectedTile {
    pub pos: Option<IVec2>,
    pub layer: Option<TileLayer>,
    pub tile_content: TileContent,
}

pub struct JustSelectedTile {
    pub pos: IVec2,
    pub tile_content: TileContent,
}

//---------------------------------------------------------------------------------------------------------------------------//
// system_params для читаемости

#[derive(SystemParam)]
pub struct WorldMaps<'w> {
    pub config: Res<'w, GameConfig>,
    pub terrain: Res<'w, TerrainMap>,
    pub resources: Res<'w, ResourceMap>,
    pub buildings: Res<'w, BuildingMap>,
    pub units: Res<'w, UnitsMap>,
    pub fog: Res<'w, FogMap>,
}
