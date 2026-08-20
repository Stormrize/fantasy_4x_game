use bevy::asset::{Asset, Handle};
use bevy::pbr::StandardMaterial;
use bevy::prelude::TypePath;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct RawVisualRaceData {
    //biome
    pub water: String,
    pub small_biome: String,
    pub medium_biome: String,
    pub large_biome: String,

    //resource
    pub small_resource: String,
    pub medium_resource: String,
    pub large_resource: String,

    //building
    pub capital: String,
    pub empty_cities: String,
    pub empty_cities_colonized: String,
    pub medium_building: String,
    pub large_building: String,
    pub spawning_building: String,
    pub house: String,
    pub research_center: String,
}

#[derive(Debug, Clone)]
pub struct VisualRaceData {
    pub biomes: [Handle<StandardMaterial>; 4],

    pub resources: [Handle<StandardMaterial>; 3],

    pub buildings: [Handle<StandardMaterial>; 8],
}

impl VisualRaceData {
    pub fn handles(&self) -> impl Iterator<Item = &Handle<StandardMaterial>> {
        [
            &self.biomes[0],
            &self.biomes[1],
            &self.biomes[2],
            &self.biomes[3],
            &self.resources[0],
            &self.resources[1],
            &self.resources[2],
            &self.buildings[0],
            &self.buildings[1],
            &self.buildings[2],
            &self.buildings[3],
            &self.buildings[4],
            &self.buildings[5],
            &self.buildings[6],
            &self.buildings[7],
        ]
        .into_iter()
    }

    pub fn get_biome(&self, index: usize) -> Handle<StandardMaterial> {
        self.biomes[index].clone()
    }
    pub fn get_resource(&self, index: usize) -> Handle<StandardMaterial> {
        self.resources[index].clone()
    }
    pub fn get_building(&self, index: usize) -> Handle<StandardMaterial> {
        self.buildings[index].clone()
    }
}

//
// small_biome: "0.99, 0.0, 0.0",
// medium_biome: "0.66, 0.0, 0.0",
// large_biome: "0.99, 0.0, 0.0",
//
// //resource
// small_resource: "0.33, 0.0, 0.0",
// medium_resource: "0.66, 0.0, 0.0",
// large_resource: "0.99, 0.0, 0.0",
//
// //building
// capital: "0.0, 1.0, 0.0",
// empty_cities: "0.0, 0.0, 1.0",
// empty_city_colonized: "0.0, 0.0, 0.0",
// medium_building: "0.0, 0.0, 0.0",
// large_building: "0.0, 0.0, 0.0",
// spawning_building: "0.0, 0.0, 0.0",
// house: "0.0, 0.0, 0.0",
// research_center: "0.0, 0.0, 0.0",
