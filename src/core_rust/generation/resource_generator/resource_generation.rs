use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::world::{
    game_configs::GenerationConfig,
    game_resources::{
        data::{
            biome::BiomeType,
            resource::{CommonKind, CommonResource, RareKind, RareResource, ResourceType},
        },
        resource_map::ResourceMap,
        terrain_map::TerrainMap,
    },
};

pub struct ResourceWindow;

impl ResourceWindow {
    /// Из скольких клеток одного типа биома выбирается одна с ресурсом.
    pub const COMMON: usize = 3;
}

pub struct RareSpawn;

impl RareSpawn {
    /// Одна редкая находка на столько клеток суши.
    pub const TILES_PER_RESOURCE: usize = 20;
    /// Жёсткий потолок на всю карту.
    pub const MAX_TOTAL: usize = 16;
    /// Минимальное расстояние между двумя редкими ресурсами (в клетках).
    pub const MIN_DISTANCE: usize = 8;
}

impl ResourceMap {
    pub fn new(config: &GenerationConfig, terrain: &TerrainMap) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed as u64);
        let size = config.terrain_size;
        let mut map: Vec<Option<ResourceType>> = vec![None; size * size];

        // Сначала обычные (порядок вызовов важен для детерминизма seed),
        // потом редкие: они не меняют раскладку обычных.
        generate_common(&mut map, terrain, size, &mut rng);
        generate_rare(&mut map, terrain, size, &mut rng);

        Self { map, size }
    }
}

// ───────────── Обычные: еда / золото / наука по биому ─────────────

fn generate_common(
    map: &mut [Option<ResourceType>],
    terrain: &TerrainMap,
    size: usize,
    rng: &mut ChaCha8Rng,
) {
    let mut windows: [Vec<(usize, usize)>; CommonKind::ALL.len()] = Default::default();

    for row in 0..size {
        for col in 0..size {
            let Some(kind) = common_kind_for_biome(terrain.get_biome_type(row, col)) else {
                continue;
            };

            let window = &mut windows[kind.index()];
            window.push((row, col));

            if window.len() == ResourceWindow::COMMON {
                place_common(map, terrain, size, window, rng);
                window.clear();
            }
        }
    }
}

fn common_kind_for_biome(biome: BiomeType) -> Option<CommonKind> {
    match biome {
        BiomeType::SmallBiome => Some(CommonKind::Food),
        BiomeType::MediumBiome => Some(CommonKind::Gold),
        BiomeType::LargeBiome => Some(CommonKind::Science),
        _ => None,
    }
}

fn place_common(
    map: &mut [Option<ResourceType>],
    terrain: &TerrainMap,
    size: usize,
    window: &[(usize, usize)],
    rng: &mut ChaCha8Rng,
) {
    let (row, col) = window[rng.random_range(0..window.len())];

    // Владелец берётся из биома именно выбранной клетки,
    // поэтому чужие цвета на поле другой расы больше не попадают.
    let biome = terrain.get_biome(row, col);
    let Some(kind) = common_kind_for_biome(biome.biome_type) else {
        return;
    };

    let (min, max) = kind.amount_range();
    let amount = rng.random_range(min..=max);

    map[row * size + col] = Some(ResourceType::Common(CommonResource {
        kind,
        amount,
        owner_id: biome.id,
        owner_race: biome.owner_race.clone(),
    }));
}

// ───────────── Редкие: свитки ─────────────

fn generate_rare(
    map: &mut [Option<ResourceType>],
    terrain: &TerrainMap,
    size: usize,
    rng: &mut ChaCha8Rng,
) {
    // 1. Кандидаты: суша без ресурса. Сначала нейтральные земли
    //    (вне биомов игроков), чтобы свитки подталкивали к исследованию.
    let mut land_count = 0;
    let mut candidates = Vec::new();

    for row in 0..size {
        for col in 0..size {
            let biome = terrain.get_biome(row, col);
            if biome.biome_type == BiomeType::Water {
                continue;
            }
            land_count += 1;

            if map[row * size + col].is_some() {
                continue;
            }
            candidates.push((row, col));
        }
    }

    // 2. Сколько всего: пропорционально суше, но не больше потолка.
    let target = (land_count / RareSpawn::TILES_PER_RESOURCE).clamp(1, RareSpawn::MAX_TOTAL);

    // 3. Детерминированное перемешивание (Fisher–Yates).
    for i in (1..candidates.len()).rev() {
        let j = rng.random_range(0..=i);
        candidates.swap(i, j);
    }

    // 4. Берём кандидатов подряд, пропуская тех, кто слишком близко к уже выбранным.
    let mut placed: Vec<(usize, usize)> = Vec::new();

    for (row, col) in candidates {
        if placed.len() >= target {
            break;
        }
        let too_close = placed
            .iter()
            .any(|&(r, c)| row.abs_diff(r).max(col.abs_diff(c)) < RareSpawn::MIN_DISTANCE);
        if too_close {
            continue;
        }

        let kind = pick_rare_kind(rng);
        let power = rare_power(kind, row, col, size);

        let owner_race = terrain.get_biome(row, col).owner_race.clone();

        map[row * size + col] = Some(ResourceType::Rare(RareResource {
            kind,
            power,
            owner_race,
        }));
        placed.push((row, col));
    }
}

fn pick_rare_kind(rng: &mut ChaCha8Rng) -> RareKind {
    let total: u32 = RareKind::ALL.iter().map(|k| k.weight()).sum();
    let mut roll = rng.random_range(0..total);

    for kind in RareKind::ALL {
        if roll < kind.weight() {
            return kind;
        }
        roll -= kind.weight();
    }
    RareKind::ALL[0]
}

/// Центр карты считается спорной зоной: свитки там в два раза сильнее.
fn rare_power(kind: RareKind, row: usize, col: usize, size: usize) -> u32 {
    let center = size / 2;
    let dist = row.abs_diff(center).max(col.abs_diff(center));
    let bonus = if dist < size / 4 { 2 } else { 1 };
    kind.base_power() * bonus
}
