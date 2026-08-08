use bevy::math::IVec2;
use bevy::prelude::Resource;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TileLayer {
    Terrain = 0,
    Resource = 1,
    Building = 2,
    Unit = 3,
    Fog = 4,
}

//---------------------------------------------------------------------------------------------------------------------------//

#[derive(Resource, Default)]
pub struct SelectedEntity {
    pub position: Option<IVec2>,
    pub layer: Option<TileLayer>,
}

//---------------------------------------------------------------------------------------------------------------------------//

impl SelectedEntity {
    pub fn clear(&mut self) {
        self.position = None;
        self.layer = None;
        println!("clear");
    }
}

//---------------------------------------------------------------------------------------------------------------------------//
// чтобы чекать что на клетке

pub struct TileContent {
    pub pos: IVec2,
    pub is_water: bool,
    pub has_unit: bool,
    pub has_building: bool,
    pub has_resource: bool,
    pub has_fog: bool,
}

impl TileContent {
    pub fn from_world(
        pos: IVec2,
        is_water: bool,
        has_unit: bool,
        has_building: bool,
        has_resource: bool,
        has_fog: bool,
    ) -> Self {

        Self {
            pos,
            is_water,
            has_unit,
            has_building,
            has_resource,
            has_fog,
        }
    }

    pub fn get_top_layer(&self) -> TileLayer {
        if self.has_fog {
            TileLayer::Fog
        } else if self.has_unit {
            TileLayer::Unit
        } else if self.has_building {
            TileLayer::Building
        } else if self.has_resource {
            TileLayer::Resource
        } else {
            TileLayer::Terrain
        }
    }
}