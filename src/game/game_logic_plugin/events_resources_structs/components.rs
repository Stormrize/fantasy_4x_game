use bevy::prelude::{IVec2, Resource};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TileLayer {
    Terrain = 0,
    Resource = 1,
    Building = 2,
    Unit = 3,
    Fog = 4,
}

#[derive(Default, Debug, Clone)]
pub struct TileContent {
    pub biome: bool,
    pub unit: bool,
    pub building: bool,
    pub resource: bool,
    pub fog: bool,
}

impl TileContent {
    pub fn from_world(biome: bool, unit: bool, building: bool, resource: bool, fog: bool) -> Self {
        Self {
            biome,
            unit,
            building,
            resource,
            fog,
        }
    }

    pub fn get_top_layer(&self) -> TileLayer {
        if self.fog {
            TileLayer::Fog
        } else if self.unit {
            TileLayer::Unit
        } else if self.building {
            TileLayer::Building
        } else if self.resource {
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

#[derive(Debug, Clone)]
pub struct JustSelectedTile {
    pub pos: IVec2,
    pub tile_content: TileContent,
}
