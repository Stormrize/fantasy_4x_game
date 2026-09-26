use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;
use bevy::ecs::system::SystemParam;
use bevy::prelude::Res;

pub mod building_map;
pub mod data;
pub mod fog_map;
pub mod resource_map;
pub mod terrain_map;
pub mod unit_map;

#[derive(SystemParam)]
pub struct WorldMaps<'w> {
    pub terrain: Res<'w, TerrainMap>,
    pub resources: Res<'w, ResourceMap>,
    pub buildings: Res<'w, BuildingMap>,
    pub units: Res<'w, UnitsMap>,
    pub fog: Res<'w, FogMap>,
}
