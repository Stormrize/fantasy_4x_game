use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use rand::RngExt;
use rand_chacha::ChaCha8Rng;

use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::terrain_map::TerrainMap;
// ---------- 1. Компоненты связности (кто с кем на одной земле) ----------

pub(super) fn compute_land_components(terrain: &TerrainMap, terrain_size: usize) -> Vec<i32> {
    let mut comp = vec![-1i32; terrain_size * terrain_size];
    let mut next_id = 0i32;
    let mut stack = Vec::new();

    for x in 0..terrain_size {
        for y in 0..terrain_size {
            let idx = x * terrain_size + y;
            if comp[idx] != -1 || !matches!(terrain.get(x, y), BiomeType::Ground) {
                continue;
            }
            // Обычный DFS-flood-fill; порядок (стек вместо очереди) роли не играет,
            // важно только что мы посещаем все связные Ground-тайлы.
            comp[idx] = next_id;
            stack.push((x, y));
            while let Some((cx, cy)) = stack.pop() {
                for (nx, ny) in neighbors4(cx, cy, terrain_size) {
                    let nidx = ny * terrain_size + nx;
                    if comp[nidx] == -1 && matches!(terrain.get(nx, ny), BiomeType::Ground) {
                        comp[nidx] = next_id;
                        stack.push((nx, ny));
                    }
                }
            }
            next_id += 1;
        }
    }
    comp
}

pub(super) fn largest_component_id(comp: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &c in comp {
        if c >= 0 {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .max_by_key(|&(_, cnt)| cnt)
        .map(|(id, _)| id)
}

// note можно добавить еще по диагонали
fn neighbors4(x: usize, y: usize, size: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut v = Vec::with_capacity(4);
    if x > 0 {
        v.push((x - 1, y));
    }
    if x + 1 < size {
        v.push((x + 1, y));
    }
    if y > 0 {
        v.push((x, y - 1));
    }
    if y + 1 < size {
        v.push((x, y + 1));
    }
    v.into_iter()
}

// ---------- 2. Жадный max-min (farthest-point sampling) для затравок ----------

pub(super) fn farthest_point_seeds(
    candidates: &[(usize, usize)],
    count: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<(usize, usize)> {
    let mut chosen = Vec::with_capacity(count);
    // note random но в принципе пока норм, можно сделать немного ограничений
    let first = candidates[rng.random_range(0..candidates.len())];
    chosen.push(first);

    let mut min_dist_sq: Vec<i64> = candidates
        .iter()
        .map(|&(x, y)| dist_sq(x, y, first.0, first.1))
        .collect();

    while chosen.len() < count {
        let (best_idx, _) = min_dist_sq
            .iter()
            .enumerate()
            .max_by_key(|&(_, &d)| d)
            .unwrap();
        let picked = candidates[best_idx];
        chosen.push(picked);

        for (i, &(x, y)) in candidates.iter().enumerate() {
            let d = dist_sq(x, y, picked.0, picked.1);
            if d < min_dist_sq[i] {
                min_dist_sq[i] = d;
            }
        }
    }
    chosen
}

fn dist_sq(x1: usize, y1: usize, x2: usize, y2: usize) -> i64 {
    let (dx, dy) = (x1 as i64 - x2 as i64, y1 as i64 - y2 as i64);
    dx * dx + dy * dy
}

// ---------- 3. Балансирующий flood fill (multi-source Dijkstra) ----------

#[derive(Eq, PartialEq)]
struct FrontierNode {
    priority_key: (usize, u32), // (размер региона владельца на момент push, расстояние)
    owner: usize,
    x: usize,
    y: usize,
    cost: u32,
}

impl Ord for FrontierNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority_key.cmp(&self.priority_key) // инвертируем — BinaryHeap это max-heap
    }
}
impl PartialOrd for FrontierNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(super) fn flood_fill_balance(
    global_terrain: &TerrainMap,
    terrain_size: usize,
    seeds: &[(usize, usize)],
) -> Vec<Vec<(usize, usize)>> {
    let mut owner: Vec<Option<usize>> = vec![None; terrain_size * terrain_size];
    let mut claimed_count = vec![0usize; seeds.len()];
    let mut regions: Vec<Vec<(usize, usize)>> = vec![Vec::new(); seeds.len()];
    let mut heap = BinaryHeap::new();

    let push_neighbors = |heap: &mut BinaryHeap<FrontierNode>,
                          owner: usize,
                          x: usize,
                          y: usize,
                          cost: u32,
                          claimed_count: &[usize]| {
        for (nx, ny) in neighbors4(x, y, terrain_size) {
            heap.push(FrontierNode {
                priority_key: (claimed_count[owner], cost + 1),
                owner,
                x: nx,
                y: ny,
                cost: cost + 1,
            });
        }
    };

    for (i, &(sx, sy)) in seeds.iter().enumerate() {
        owner[sy * terrain_size + sx] = Some(i);
        claimed_count[i] += 1;
        regions[i].push((sx, sy));
        push_neighbors(&mut heap, i, sx, sy, 0, &claimed_count);
    }

    while let Some(node) = heap.pop() {
        let idx = node.y * terrain_size + node.x;
        if owner[idx].is_some() || !matches!(global_terrain.get(node.x, node.y), BiomeType::Ground)
        {
            continue; // уже занято или вода — рост тут обрывается
        }
        owner[idx] = Some(node.owner);
        claimed_count[node.owner] += 1;
        regions[node.owner].push((node.x, node.y));
        push_neighbors(
            &mut heap,
            node.owner,
            node.x,
            node.y,
            node.cost,
            &claimed_count,
        );
    }

    regions
}
