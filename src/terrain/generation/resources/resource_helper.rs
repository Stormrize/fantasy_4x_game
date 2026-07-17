use rand::prelude::IndexedRandom;
use rand_chacha::ChaCha8Rng;
use crate::terrain::enums::biome::BiomeType;
use crate::terrain::enums::resources::ResourceType;
use crate::terrain::generation::terrain::terrain_generation::TerrainMap;
use crate::terrain::terrain_config::TerrainConfig;

struct ResourceSpawn {
    resource: ResourceType,
    weight: u8,
}


fn biome_pool(biome: BiomeType) -> &'static [ResourceSpawn] {
    use BiomeType::*;
    use ResourceType::*;

    match biome {
        Water => &[],

        Grass => &[
            ResourceSpawn { resource: Food, weight: 60 },
            ResourceSpawn { resource: Wood, weight: 15 },
            ResourceSpawn { resource: Stone, weight: 15 },
            ResourceSpawn { resource: MagicShards, weight: 10 },
        ],

        Forest => &[
            ResourceSpawn { resource: Wood, weight: 80 },
            ResourceSpawn { resource: Food, weight: 10 },
            ResourceSpawn { resource: Stone, weight: 5 },
            ResourceSpawn { resource: MagicShards, weight: 5 },
        ],

        Hills => &[
            ResourceSpawn { resource: Stone, weight: 80 },
            ResourceSpawn { resource: Wood, weight: 5 },
            ResourceSpawn { resource: Food, weight: 10 },
            ResourceSpawn { resource: MagicShards, weight: 5 },
        ],
    }
}

fn choose_resource(
    biome: BiomeType,
    rng: &mut ChaCha8Rng,
) -> ResourceType {

    let pool = biome_pool(biome);

    pool.choose_weighted(rng, |r| r.weight)
        .map(|r| r.resource)
        .unwrap_or(ResourceType::None)
}


fn choose_position(
    candidates: &[(usize, usize)],
    rng: &mut ChaCha8Rng,
) -> (usize, usize) {

    *candidates
        .choose(rng)
        .expect("candidates list empty")
}


pub fn place_resource(
    terrain: &TerrainMap,
    map: &mut Vec<ResourceType>,
    rng: &mut ChaCha8Rng,
    candidates: &[(usize, usize)],
    config: &TerrainConfig,
) {
    let (y, x) = choose_position(
        candidates,
        rng,
    );

    let biome = terrain.get(x, y);

    let resource = choose_resource(
        biome,
        rng,
    );
    let index = y * config.terrain_size + x;

    map[index] = resource;
}