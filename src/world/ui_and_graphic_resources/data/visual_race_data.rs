use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
#[derive(Debug, Clone)]
pub struct RaceAssets {
    pub biomes: [Handle<StandardMaterial>; 4],

    pub resources: [Handle<StandardMaterial>; 3],

    pub buildings: [Handle<StandardMaterial>; 9],
}

impl RaceAssets {
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
            &self.buildings[8],
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
