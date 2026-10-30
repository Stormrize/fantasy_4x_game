//! Чистые алгоритмы движения по карте (без зависимостей от ECS).
//!
//! Модель стоимости по заметкам из `notes.md`:
//! - вода непроходима;
//! - ground/small biome стоят 1 очко, medium — 2, large — 3;
//! - юнит с 1 очком всё равно может войти на любую непроходимую-не-воду клетку
//!   (платит `min(оставшиеся очки, стоимость клетки)`).
//!
//! Эти же функции используются правилами игры (валидация ходов) и ИИ
//! (планирование), поэтому игрок и боты ходят по одним законам.

use crate::world::game_resources::data::biome::BiomeType;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Недостижимая клетка.
pub const UNREACHABLE: u16 = u16::MAX;

/// Номинальная стоимость входа на клетку: 0 — заблокировано
/// (вода/неизвестно/враг), иначе 1..=3.
#[inline]
pub fn biome_cost(biome: BiomeType) -> u8 {
    match biome {
        BiomeType::Water => 0,
        BiomeType::SmallBiome | BiomeType::Ground => 1,
        BiomeType::MediumBiome => 2,
        BiomeType::LargeBiome => 3,
    }
}

#[inline]
pub fn is_blocked(cost: u8) -> bool {
    cost == 0
}

#[inline]
fn in_bounds(row: isize, col: isize, size: usize) -> bool {
    row >= 0 && col >= 0 && (row as usize) < size && (col as usize) < size
}

/// 8 соседей (диагонали разрешены, как в chebyshev-проверке ходов).
#[inline]
pub fn neighbors(row: usize, col: usize, size: usize) -> impl Iterator<Item = (usize, usize)> {
    const DELTAS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    DELTAS.into_iter().filter_map(move |(dr, dc)| {
        let r = row as isize + dr;
        let c = col as isize + dc;
        if in_bounds(r, c, size) {
            Some((r as usize, c as usize))
        } else {
            None
        }
    })
}

/// Минимальные потраченные очки движения до каждой клетки при бюджете `budget`.
///
/// Поиск по состояниям (клетка, потраченные очки): оставшийся бюджет влияет на
/// стоимость входа (`min(оставшееся, номинал)`), поэтому простой Дейкстры
/// по клеткам недостаточно. Результат — проекция на клетки: `out[i]` —
/// минимальные очки, которые нужно потратить, чтобы оказаться на клетке `i`
/// (`UNREACHABLE`, если за бюджет не добраться).
pub fn movement_field(costs: &[u8], size: usize, start: (usize, usize), budget: u16) -> Vec<u16> {
    let n = size * size;
    let layers = budget as usize + 1;

    let mut dist = vec![UNREACHABLE; n * layers];
    let mut heap: BinaryHeap<(Reverse<u16>, usize)> = BinaryHeap::new();

    let start_state = (start.0 * size + start.1) * layers;
    dist[start_state] = 0;
    heap.push((Reverse(0), start_state));

    while let Some((Reverse(spent), state)) = heap.pop() {
        if spent != dist[state] {
            continue;
        }
        if spent >= budget {
            continue;
        }

        let tile = state / layers;
        let (row, col) = (tile / size, tile % size);
        let remaining = budget - spent;

        for (nr, nc) in neighbors(row, col, size) {
            let nt = nr * size + nc;
            let cost = costs[nt];
            if is_blocked(cost) {
                continue;
            }

            let pay = remaining.min(cost as u16);
            let next_spent = spent + pay;
            let next_state = nt * layers + next_spent as usize;

            if next_spent < dist[next_state] {
                dist[next_state] = next_spent;
                heap.push((Reverse(next_spent), next_state));
            }
        }
    }

    let mut out = vec![UNREACHABLE; n];
    for tile in 0..n {
        for s in 0..layers {
            let d = dist[tile * layers + s];
            if d < out[tile] {
                out[tile] = d;
            }
        }
    }
    out[start.0 * size + start.1] = 0;
    out
}

/// Номинальные расстояния без учёта бюджета (обычный Дейкстра по номинальной
/// стоимости) + родитель каждой клетки для восстановления пути.
///
/// Используется ИИ для оценки дальних целей и построения маршрутов подхода.
pub fn nominal_field(costs: &[u8], size: usize, start: (usize, usize)) -> (Vec<u16>, Vec<u32>) {
    let n = size * size;
    let mut dist = vec![UNREACHABLE; n];
    let mut parent = vec![u32::MAX; n];

    let start_idx = start.0 * size + start.1;
    dist[start_idx] = 0;

    let mut heap: BinaryHeap<(Reverse<u16>, usize)> = BinaryHeap::new();
    heap.push((Reverse(0), start_idx));

    while let Some((Reverse(d), tile)) = heap.pop() {
        if d != dist[tile] {
            continue;
        }
        let (row, col) = (tile / size, tile % size);

        for (nr, nc) in neighbors(row, col, size) {
            let nt = nr * size + nc;
            let cost = costs[nt];
            if is_blocked(cost) {
                continue;
            }
            let nd = d + cost as u16;
            if nd < dist[nt] {
                dist[nt] = nd;
                parent[nt] = tile as u32;
                heap.push((Reverse(nd), nt));
            }
        }
    }

    (dist, parent)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::erasing_op, clippy::identity_op)]

    use super::*;

    fn grid_of(size: usize, overrides: &[(usize, usize, u8)]) -> Vec<u8> {
        let mut grid = vec![1u8; size * size];
        for &(row, col, cost) in overrides {
            grid[row * size + col] = cost;
        }
        grid
    }

    #[test]
    fn biome_costs_are_mapped() {
        assert_eq!(biome_cost(BiomeType::Water), 0);
        assert_eq!(biome_cost(BiomeType::Ground), 1);
        assert_eq!(biome_cost(BiomeType::SmallBiome), 1);
        assert_eq!(biome_cost(BiomeType::MediumBiome), 2);
        assert_eq!(biome_cost(BiomeType::LargeBiome), 3);
    }

    #[test]
    fn one_point_still_enters_large_biome() {
        // Правило из notes.md: с 1 очком можно шагнуть даже на large biome.
        let grid = grid_of(3, &[(1, 0, 3)]);
        let dist = movement_field(&grid, 3, (0, 0), 1);
        assert_eq!(dist[1 * 3 + 0], 1);
    }

    #[test]
    fn medium_biome_costs_two_points() {
        let grid = grid_of(3, &[(1, 0, 2)]);
        let dist = movement_field(&grid, 3, (0, 0), 2);
        assert_eq!(dist[1 * 3 + 0], 2);

        // А с одним очком — тоже можно (см. правило выше).
        let dist_one = movement_field(&grid, 3, (0, 0), 1);
        assert_eq!(dist_one[1 * 3 + 0], 1);
    }

    #[test]
    fn water_blocks_movement() {
        let grid = grid_of(3, &[(1, 0, 0)]);
        let dist = movement_field(&grid, 3, (0, 0), 2);
        assert_eq!(dist[1 * 3 + 0], UNREACHABLE);
    }

    #[test]
    fn budget_limits_reach() {
        let grid = vec![1u8; 5 * 5];
        let dist = movement_field(&grid, 5, (0, 0), 2);
        assert_eq!(dist[0 * 5 + 2], 2);
        assert_eq!(dist[0 * 5 + 3], UNREACHABLE);
        assert_eq!(dist[2 * 5 + 2], 2); // диагонали разрешены
    }

    #[test]
    fn diagonal_moves_allowed() {
        let grid = vec![1u8; 3 * 3];
        let dist = movement_field(&grid, 3, (0, 0), 1);
        assert_eq!(dist[1 * 3 + 1], 1);
    }

    #[test]
    fn nominal_parents_reconstruct_path() {
        let grid = vec![1u8; 5 * 5];
        let (dist, parents) = nominal_field(&grid, 5, (0, 0));

        let target = 1 * 5 + 2;
        assert_eq!(dist[target], 2);

        let mut path = vec![target];
        let mut current = target;
        while current != 0 && parents[current] != u32::MAX {
            current = parents[current] as usize;
            path.push(current);
        }
        assert_eq!(current, 0);
        assert_eq!(path.len(), 3); // старт + 2 шага
    }
}
