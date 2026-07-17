use crate::terrain::enums::biome::BiomeType;
use crate::terrain::enums::buildings::BuildingType;
use crate::terrain::enums::resources::ResourceType;

#[derive(Default, Copy, Clone)]
pub struct Tile {
    pub biome: BiomeType,
    pub resource: ResourceType,
    pub building: BuildingType,
}

impl Tile {
    pub fn new(biome: BiomeType, resource: ResourceType, building: BuildingType) -> Tile {
        Self { biome, resource, building }
    }
}