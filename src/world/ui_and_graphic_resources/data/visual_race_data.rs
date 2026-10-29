use bevy::asset::{Handle, UntypedHandle};
use bevy::pbr::StandardMaterial;
use bevy::world_serialization::WorldAsset;
#[derive(Debug, Clone)]
pub struct RaceAssets {
    pub biomes: [Handle<StandardMaterial>; 4],

    pub resources: [Handle<StandardMaterial>; 7],

    pub buildings: [Handle<WorldAsset>; 9],

    pub units: [Handle<WorldAsset>; 4],
}

impl RaceAssets {
    pub fn handles(&self) -> impl Iterator<Item = UntypedHandle> {
        [
            self.biomes[0].clone().untyped(),
            self.biomes[1].clone().untyped(),
            self.biomes[2].clone().untyped(),
            self.biomes[3].clone().untyped(),
            self.resources[0].clone().untyped(),
            self.resources[1].clone().untyped(),
            self.resources[2].clone().untyped(),
            self.resources[3].clone().untyped(),
            self.resources[4].clone().untyped(),
            self.resources[5].clone().untyped(),
            self.resources[6].clone().untyped(),
            self.buildings[0].clone().untyped(),
            self.buildings[1].clone().untyped(),
            self.buildings[2].clone().untyped(),
            self.buildings[3].clone().untyped(),
            self.buildings[4].clone().untyped(),
            self.buildings[5].clone().untyped(),
            self.buildings[6].clone().untyped(),
            self.buildings[7].clone().untyped(),
            self.buildings[8].clone().untyped(),
            self.units[0].clone().untyped(),
            self.units[1].clone().untyped(),
            self.units[2].clone().untyped(),
            self.units[3].clone().untyped(),
        ]
        .into_iter()
    }

    pub fn get_biome(&self, index: usize) -> Handle<StandardMaterial> {
        self.biomes[index].clone()
    }
    pub fn get_resource(&self, index: usize) -> Handle<StandardMaterial> {
        self.resources[index].clone()
    }
    pub fn get_building(&self, index: usize) -> Handle<WorldAsset> {
        self.buildings[index].clone()
    }
    pub fn get_unit(&self, index: usize) -> Handle<WorldAsset> {
        self.units[index].clone()
    }
}
