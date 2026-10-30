use crate::game::game_logic_plugin::action::rules::movement::{
    UNREACHABLE, movement_field, neighbors, nominal_field,
};
use crate::game::game_logic_plugin::ai::perception::{
    OwnBuilding, OwnCombatUnit, OwnHelper, WorldSnapshot,
};
use crate::game::game_logic_plugin::ai::plan::{AiAction, BotTurnPlan};
use crate::game::game_logic_plugin::ai::profile::AiProfile;
use crate::world::components::Gold;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::building::{BuildingData, BuildingLevel, BuildingType};
use crate::world::game_resources::data::resource::{CommonKind, RareKind};
use crate::world::game_resources::data::unit::{HELPER, UnitType};
use bevy::math::IVec2;
use bevy::tasks::ComputeTaskPool;
use std::cmp::Ordering;
use std::collections::HashSet;

/// Радиус обзора строителя для оценки пользы разведки.
const EXPLORATION_FOV: isize = 2;

/// Ожидаемая доля успеха шага в неизвестное (там может быть вода).
const EXPLORATION_RISK: f32 = 0.6;

/// Минимум неизведанных клеток, чтобы тратить свиток открытия местности.
const UNEXPLORED_SCROLL_THRESHOLD: usize = 64;

/// Оценка потребностей экономики: > 1 — ресурс нужен, < 1 — избыток.
#[derive(Clone, Copy)]
struct Needs {
    gold: f32,
    food: f32,
    science: f32,
}

impl Needs {
    fn from(snapshot: &WorldSnapshot, profile: &AiProfile) -> Self {
        Self {
            gold: need_factor(snapshot.gold as f32, 40.0),
            food: need_factor(snapshot.food as f32, 30.0),
            science: need_factor(snapshot.science as f32, 25.0) * profile.science_bias,
        }
    }
}

fn need_factor(current: f32, target: f32) -> f32 {
    ((target - current) / target).clamp(0.25, 2.5)
}

fn common_value(kind: CommonKind, needs: &Needs) -> f32 {
    match kind {
        CommonKind::Gold => 6.0 * needs.gold,
        CommonKind::Food => 5.0 * needs.food,
        CommonKind::Science => 7.0 * needs.science,
    }
}

fn rare_value(kind: RareKind) -> f32 {
    match kind {
        RareKind::RevealScroll => 4.0,
        RareKind::ManaScroll => 5.0,
        RareKind::HealScroll => 6.0,
        RareKind::AttackScroll => 3.0,
    }
}

/// Кандидат на строительство: клетка + тип здания + привлекательность.
#[derive(Clone, Copy)]
struct BuildSite {
    pos: IVec2,
    building_type: BuildingType,
    score: f32,
}

/// Цель-ресурс: клетка + ценность + виден ли ресурс в текущий ход.
#[derive(Clone, Copy)]
struct ResourceTarget {
    pos: IVec2,
    value: f32,
    visible: bool,
}

#[inline]
fn flat_of(pos: IVec2, size: usize) -> usize {
    pos.x as usize * size + pos.y as usize
}

/// Главная точка входа планировщика небоевого хода бота.
///
/// Выполняется в фоновой задаче (`AsyncComputeTaskPool`), внутри неё выбор
/// действий строителей распараллелен по потокам `ComputeTaskPool`.
/// Планировщик видит только `WorldSnapshot` — то, что бот реально видел.
pub fn plan_bot_turn(snapshot: WorldSnapshot, profile: AiProfile) -> BotTurnPlan {
    let size = snapshot.size;
    let needs = Needs::from(&snapshot, &profile);

    let mut plan = BotTurnPlan::default();
    let mut gold = snapshot.gold as i32;

    // Города для найма строителей.
    let cities: Vec<IVec2> = snapshot
        .own_buildings
        .iter()
        .filter(|b| {
            matches!(
                b.building_type,
                BuildingType::Capital | BuildingType::CapturedCity | BuildingType::SpawningBuilding
            )
        })
        .map(|b| b.pos)
        .collect();
    let Some(&capital) = cities.first() else {
        // Нет городов — нечего делать.
        return plan;
    };

    // 1. Найм строителей, пока позволяет золото и лимит сложности.
    let mut hires_left = profile.max_hires_per_turn;
    while hires_left > 0 {
        if snapshot.own_helpers.len() + (profile.max_hires_per_turn - hires_left)
            >= profile.max_helpers
        {
            break;
        }
        if gold < HELPER.unit_cost.0 as i32 + profile.gold_reserve as i32 {
            break;
        }
        plan.actions.push(AiAction::SpawnUnit {
            city_pos: capital,
            unit_type: UnitType::Helper,
        });
        gold -= HELPER.unit_cost.0 as i32;
        hires_left -= 1;
    }

    // 1.5 Найм боевых юнитов, если построено здание спавна.
    let spawning_building = snapshot
        .own_buildings
        .iter()
        .find(|b| b.building_type == BuildingType::SpawningBuilding)
        .map(|b| b.pos);
    if let Some(spawn_pos) = spawning_building {
        for _ in 0..profile.max_combat_hires_per_turn {
            if snapshot.own_combat_units.len() >= profile.max_combat_units {
                break;
            }
            let unit_type = choose_combat_unit_type(&snapshot.own_combat_units);
            let cost = snapshot.combat_unit_costs[unit_type.index()] as i32;
            if cost == 0 || gold < cost + profile.gold_reserve as i32 {
                break;
            }
            plan.actions.push(AiAction::SpawnUnit {
                city_pos: spawn_pos,
                unit_type,
            });
            gold -= cost;
        }
    }

    // 2. Улучшения зданий с лучшим соотношением прироста дохода к цене.
    let mut upgrades_left = profile.max_upgrades_per_turn;
    let mut upgrade_candidates: Vec<(OwnBuilding, f32)> = snapshot
        .own_buildings
        .iter()
        .filter_map(|building| {
            upgrade_value(building, &needs).map(|(cost, value)| (*building, value / cost as f32))
        })
        .collect();
    upgrade_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

    for (building, _) in upgrade_candidates {
        if upgrades_left == 0 {
            break;
        }
        let Some(cost) = upgrade_cost(&building) else {
            continue;
        };
        if gold < cost as i32 + profile.gold_reserve as i32 {
            continue;
        }
        plan.actions
            .push(AiAction::UpgradeBuilding { pos: building.pos });
        gold -= cost as i32;
        upgrades_left -= 1;
    }

    // 3. Сетка движения: известно проходимое + блоки чужих юнитов и своих
    //    строителей. Отдельная сетка с «кольцом» неизвестного — для разведки.
    let mut grid = snapshot.cost.clone();
    for (index, unit) in snapshot.known_enemy_units.iter().enumerate() {
        if unit.is_some() {
            grid[index] = 0;
        }
    }
    for helper in &snapshot.own_helpers {
        grid[flat_of(helper.pos, size)] = 0;
    }
    let ring_grid = exploration_grid(&snapshot);

    // 4. Кандидаты для действий.
    let sites = compute_build_sites(&snapshot, &needs);
    let targets = resource_targets(&snapshot, &needs);
    let explore_targets = exploration_targets(&snapshot, &ring_grid);

    // 5. Параллельный выбор действия для каждого строителя.
    //    `async` без `move`: фьючеры заимствуют общие данные из scope.
    //    `get_or_init` — чтобы планирование работало и в юнит-тестах без App.
    let decisions: Vec<Option<(f32, Vec<AiAction>)>> =
        ComputeTaskPool::get_or_init(bevy::tasks::TaskPool::new).scope(|scope| {
            for helper in &snapshot.own_helpers {
                scope.spawn(async {
                    choose_helper_actions(
                        helper,
                        &grid,
                        &ring_grid,
                        size,
                        &sites,
                        &targets,
                        &explore_targets,
                        &snapshot.explored,
                        &profile,
                        gold.max(0) as Gold,
                    )
                });
            }
        });

    // 6. Слияние решений: лучшие по score, без конфликтов за одни и те же клетки.
    let mut ranked: Vec<(usize, f32, Vec<AiAction>)> = decisions
        .into_iter()
        .enumerate()
        .filter_map(|(index, decision)| decision.map(|(score, actions)| (index, score, actions)))
        .collect();
    ranked.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(Ordering::Equal)
            .then(a.0.cmp(&b.0))
    });

    // Стройки тратят золото — учитываем общий бюджет, чтобы не запланировать
    // больше строек, чем бот может оплатить.
    let mut gold_budget = gold;
    let mut claimed: HashSet<(i32, i32)> = HashSet::new();
    for (_, _, actions) in ranked {
        let build_cost = actions.iter().find_map(|action| match action {
            AiAction::Build { building_type, .. } => BuildingData::from_type(*building_type)
                .map(|data| data.stats(BuildingLevel::Standard).upgrade_cost()),
            _ => None,
        });
        if let Some(cost) = build_cost
            && gold_budget < cost as i32
        {
            continue;
        }

        let claim = match actions.first() {
            Some(AiAction::Move { to, .. }) => (to.x, to.y),
            Some(AiAction::Gather { pos }) => (pos.x, pos.y),
            Some(AiAction::Build { pos, .. }) => (pos.x, pos.y),
            _ => continue,
        };
        if claimed.contains(&claim) {
            continue;
        }
        claimed.insert(claim);
        if let Some(cost) = build_cost {
            gold_budget -= cost as i32;
        }
        plan.actions.extend(actions);
    }

    // 7. Свиток открытия местности — если ещё есть что открывать.
    let unexplored_count = snapshot
        .explored
        .iter()
        .filter(|explored| !**explored)
        .count();
    if snapshot.reveal_scrolls > 0
        && unexplored_count >= UNEXPLORED_SCROLL_THRESHOLD
        && let Some(center) = pick_reveal_center(&snapshot, &explore_targets)
    {
        plan.actions.push(AiAction::RevealMap { center });
    }

    plan
}

/// Сетка движения с «кольцом»: неизвестные клетки рядом с известной
/// проходимой местностью считаются проходимыми (бот может шагнуть в туман,
/// как и игрок; если там вода — ход честно не сработает).
fn exploration_grid(snapshot: &WorldSnapshot) -> Vec<u8> {
    let size = snapshot.size;
    let mut grid = snapshot.cost.clone();
    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            if grid[index] != 0 || snapshot.explored[index] {
                continue;
            }
            for (nr, nc) in neighbors(row, col, size) {
                let neighbor_index = nr * size + nc;
                if snapshot.cost[neighbor_index] != 0 {
                    grid[index] = 1;
                    break;
                }
            }
        }
    }
    grid
}

/// Клетки тумана, в которые бот может шагнуть в этом ходу (цели разведки).
fn exploration_targets(snapshot: &WorldSnapshot, ring_grid: &[u8]) -> Vec<(usize, usize)> {
    let size = snapshot.size;
    let mut targets = Vec::new();
    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            if !snapshot.explored[index] && ring_grid[index] != 0 {
                targets.push((row, col));
            }
        }
    }
    targets
}

fn compute_build_sites(snapshot: &WorldSnapshot, needs: &Needs) -> Vec<BuildSite> {
    let size = snapshot.size;
    let want_science = needs.science >= needs.gold * 0.8;
    let has_spawning = snapshot
        .own_buildings
        .iter()
        .any(|b| b.building_type == BuildingType::SpawningBuilding);

    let mut sites = Vec::new();
    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            if snapshot.cost[index] == 0 {
                continue;
            }
            if snapshot.known_buildings[index].is_some()
                || snapshot.known_enemy_units[index].is_some()
            {
                continue;
            }

            let building_type = match snapshot.biome[index] {
                BiomeType::SmallBiome => {
                    if want_science {
                        BuildingType::ResearchCenter
                    } else {
                        BuildingType::House
                    }
                }
                BiomeType::MediumBiome => BuildingType::MediumBuilding,
                BiomeType::LargeBiome => BuildingType::LargeBuilding,
                _ => continue,
            };

            let on_resource =
                snapshot.known_resources[index].is_some() || snapshot.known_rare[index].is_some();
            let mut frontier_adjacent = false;
            let mut own_neighbors = 0u8;
            for (nr, nc) in neighbors(row, col, size) {
                let neighbor_index = nr * size + nc;
                if snapshot.cost[neighbor_index] == 0 {
                    frontier_adjacent = true;
                }
                if let Some(building) = snapshot.known_buildings[neighbor_index]
                    && building.owner == snapshot.id
                {
                    own_neighbors += 1;
                }
            }

            let base = match building_type {
                BuildingType::MediumBuilding => 8.0 * needs.gold,
                BuildingType::LargeBuilding => 12.0 * needs.gold,
                BuildingType::ResearchCenter => 10.0 * needs.science,
                BuildingType::House => 2.0 * needs.food,
                _ => 0.0,
            };
            let score = base
                + if on_resource { 1.5 } else { 0.0 }
                + if frontier_adjacent { 1.0 } else { 0.0 }
                + own_neighbors as f32 * 0.8;

            sites.push(BuildSite {
                pos: IVec2::new(row as i32, col as i32),
                building_type,
                score,
            });

            // Военное здание спавна — строим одно, на любом проходимом биоме.
            if !has_spawning {
                sites.push(BuildSite {
                    pos: IVec2::new(row as i32, col as i32),
                    building_type: BuildingType::SpawningBuilding,
                    score: 7.0 + own_neighbors as f32 * 0.8,
                });
            }
        }
    }

    sites.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
    sites.truncate(128);
    sites
}

fn resource_targets(snapshot: &WorldSnapshot, needs: &Needs) -> Vec<ResourceTarget> {
    let size = snapshot.size;
    let mut targets = Vec::new();
    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            let pos = IVec2::new(row as i32, col as i32);
            let visible = snapshot.visible_now[index];

            if let Some(kind) = snapshot.known_resources[index] {
                targets.push(ResourceTarget {
                    pos,
                    value: common_value(kind, needs),
                    visible,
                });
            }
            if let Some(kind) = snapshot.known_rare[index] {
                targets.push(ResourceTarget {
                    pos,
                    value: rare_value(kind),
                    visible,
                });
            }
        }
    }
    targets
}

/// Выбирает лучшее действие одного строителя. Возвращает (score, действия).
#[allow(clippy::too_many_arguments)]
fn choose_helper_actions(
    helper: &OwnHelper,
    grid: &[u8],
    ring_grid: &[u8],
    size: usize,
    sites: &[BuildSite],
    targets: &[ResourceTarget],
    explore_targets: &[(usize, usize)],
    explored: &[bool],
    profile: &AiProfile,
    gold: Gold,
) -> Option<(f32, Vec<AiAction>)> {
    let start = (helper.pos.x as usize, helper.pos.y as usize);
    let start_flat = start.0 * size + start.1;

    // Без очков движения можно только собрать ресурс или построиться под собой.
    if helper.movement_points == 0 {
        let mut best: Option<(f32, Vec<AiAction>)> = None;
        if let Some(target) = targets.iter().find(|t| flat_of(t.pos, size) == start_flat) {
            let score = target.value * if target.visible { 1.15 } else { 1.0 };
            best = Some((score, vec![AiAction::Gather { pos: target.pos }]));
        }
        if helper.charges > 0 {
            for site in sites {
                if flat_of(site.pos, size) != start_flat {
                    continue;
                }
                let Some(data) = BuildingData::from_type(site.building_type) else {
                    continue;
                };
                if data.stats(BuildingLevel::Standard).upgrade_cost() > gold {
                    continue;
                }
                if best
                    .as_ref()
                    .is_none_or(|(best_score, _)| site.score > *best_score)
                {
                    best = Some((
                        site.score,
                        vec![AiAction::Build {
                            pos: site.pos,
                            building_type: site.building_type,
                        }],
                    ));
                }
            }
        }
        return best;
    }

    let bounded = movement_field(grid, size, start, helper.movement_points);

    let mut best: Option<(f32, Vec<AiAction>)> = None;
    let mut consider = |score: f32, actions: Vec<AiAction>| {
        if best
            .as_ref()
            .is_none_or(|(best_score, _)| score > *best_score)
        {
            best = Some((score, actions));
        }
    };

    // 1. Сбор ресурсов в зоне досягаемости.
    for target in targets {
        let distance = bounded[flat_of(target.pos, size)];
        if distance == UNREACHABLE {
            continue;
        }
        let score =
            target.value / (distance as f32 + 1.0) * if target.visible { 1.15 } else { 1.0 };
        let actions = if distance == 0 {
            vec![AiAction::Gather { pos: target.pos }]
        } else {
            vec![
                AiAction::Move {
                    from: helper.pos,
                    to: target.pos,
                },
                AiAction::Gather { pos: target.pos },
            ]
        };
        consider(score, actions);
    }

    // 2. Строительство, если остались заряды и хватает золота на стройку.
    if helper.charges > 0 {
        for site in sites {
            // Не планируем то, что не можем оплатить.
            let Some(data) = BuildingData::from_type(site.building_type) else {
                continue;
            };
            if data.stats(BuildingLevel::Standard).upgrade_cost() > gold {
                continue;
            }
            let distance = bounded[flat_of(site.pos, size)];
            if distance == UNREACHABLE {
                continue;
            }
            let score = site.score / (distance as f32 + 1.0);
            let actions = if distance == 0 {
                vec![AiAction::Build {
                    pos: site.pos,
                    building_type: site.building_type,
                }]
            } else {
                vec![
                    AiAction::Move {
                        from: helper.pos,
                        to: site.pos,
                    },
                    AiAction::Build {
                        pos: site.pos,
                        building_type: site.building_type,
                    },
                ]
            };
            consider(score, actions);
        }
    }

    // 3. Разведка: шаг в туман на границе известного.
    if profile.exploration_weight > 0.0 && !explore_targets.is_empty() {
        let ring_bounded = movement_field(ring_grid, size, start, helper.movement_points);
        let mut best_explore: Option<(f32, IVec2)> = None;
        for &(row, col) in explore_targets {
            let distance = ring_bounded[row * size + col];
            if distance == UNREACHABLE {
                continue;
            }
            let reveal = estimate_reveal(explored, size, (row, col), EXPLORATION_FOV);
            let score = profile.exploration_weight * reveal as f32 * EXPLORATION_RISK
                / (distance as f32 + 1.0);
            if best_explore.is_none_or(|(best_score, _)| score > best_score) {
                best_explore = Some((score, IVec2::new(row as i32, col as i32)));
            }
        }
        if let Some((score, to)) = best_explore {
            consider(
                score,
                vec![AiAction::Move {
                    from: helper.pos,
                    to,
                }],
            );
        }
    }

    // 4. Подход к самой ценной дальней цели, если ничего лучше нет.
    if best.is_none() {
        let (nominal, parents) = nominal_field(grid, size, start);
        let mut best_target: Option<(f32, usize)> = None;
        for target in targets {
            let index = flat_of(target.pos, size);
            let distance = nominal[index];
            if distance == UNREACHABLE {
                continue;
            }
            let score = target.value / (distance as f32 + 1.0);
            if best_target.is_none_or(|(best_score, _)| score > best_score) {
                best_target = Some((score, index));
            }
        }

        if let Some((score, target_index)) = best_target {
            let mut path = vec![target_index];
            let mut current = target_index;
            while current != start_flat && parents[current] != u32::MAX {
                current = parents[current] as usize;
                path.push(current);
            }

            if current == start_flat {
                path.reverse();
                let mut destination = start_flat;
                for &tile in &path {
                    if bounded[tile] != UNREACHABLE {
                        destination = tile;
                    } else {
                        break;
                    }
                }
                if destination != start_flat {
                    let to = IVec2::new((destination / size) as i32, (destination % size) as i32);
                    best = Some((
                        score * 0.7,
                        vec![AiAction::Move {
                            from: helper.pos,
                            to,
                        }],
                    ));
                }
            }
        }
    }

    best
}

/// Сколько неизвестных клеток откроется вокруг точки (в радиусе `radius`).
fn estimate_reveal(
    explored: &[bool],
    size: usize,
    (row, col): (usize, usize),
    radius: isize,
) -> u32 {
    let mut count = 0;
    for dr in -radius..=radius {
        for dc in -radius..=radius {
            let r = row as isize + dr;
            let c = col as isize + dc;
            if r < 0 || c < 0 || r as usize >= size || c as usize >= size {
                continue;
            }
            if !explored[(r as usize) * size + (c as usize)] {
                count += 1;
            }
        }
    }
    count
}

/// Лучшая точка для свитка открытия местности: граница, вокруг которой
/// больше всего неизвестного.
fn pick_reveal_center(
    snapshot: &WorldSnapshot,
    explore_targets: &[(usize, usize)],
) -> Option<IVec2> {
    if explore_targets.is_empty() {
        return None;
    }
    let radius = RareKind::RevealScroll.base_power() as isize;

    let mut best: Option<(u32, IVec2)> = None;
    let mut considered = 0usize;
    for &(row, col) in explore_targets {
        let reveal = estimate_reveal(&snapshot.explored, snapshot.size, (row, col), radius);
        if best.is_none_or(|(value, _)| reveal > value) {
            best = Some((reveal, IVec2::new(row as i32, col as i32)));
        }
        considered += 1;
        if considered >= 256 {
            break;
        }
    }
    best.map(|(_, pos)| pos)
}

fn upgrade_cost(building: &OwnBuilding) -> Option<Gold> {
    let data = BuildingData::from_type(building.building_type)?;
    Some(data.stats(building.level).upgrade_cost())
}

/// (стоимость улучшения, ценность прироста доходов по текущим потребностям)
fn upgrade_value(building: &OwnBuilding, needs: &Needs) -> Option<(Gold, f32)> {
    let data = BuildingData::from_type(building.building_type)?;
    let current = data.stats(building.level);
    let next_level = building.level.next()?;
    let next = data.stats(next_level);

    let delta = (next.gold() as f32 - current.gold() as f32) * needs.gold
        + (next.food() as f32 - current.food() as f32) * needs.food
        + (next.science() as f32 - current.science() as f32) * needs.science;

    Some((current.upgrade_cost(), delta.max(0.0)))
}

/// Какой боевой юнит нанять следующим, чтобы держать армию сбалансированной.
fn choose_combat_unit_type(units: &[OwnCombatUnit]) -> UnitType {
    let damage = units
        .iter()
        .filter(|u| u.unit_type == UnitType::DamageDealer)
        .count();
    let support = units
        .iter()
        .filter(|u| u.unit_type == UnitType::Support)
        .count();
    let skill = units
        .iter()
        .filter(|u| u.unit_type == UnitType::Skill)
        .count();

    if damage == 0 {
        UnitType::DamageDealer
    } else if support == 0 {
        UnitType::Support
    } else if skill == 0 {
        UnitType::Skill
    } else if damage <= support && damage <= skill {
        UnitType::DamageDealer
    } else if support <= skill {
        UnitType::Support
    } else {
        UnitType::Skill
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::game_logic_plugin::ai::memory::SeenBuilding;
    use crate::world::components::{AiDifficulty, Id};

    /// Мини-карта: всё известно, биом small, капитолий на (0,0).
    fn snapshot(size: usize, helpers: Vec<OwnHelper>, gold: u16) -> WorldSnapshot {
        let mut known_buildings = vec![None; size * size];
        known_buildings[0] = Some(SeenBuilding {
            owner: Id(1),
            building_type: BuildingType::Capital,
        });

        WorldSnapshot {
            id: Id(1),
            size,
            cost: vec![1u8; size * size],
            biome: vec![BiomeType::SmallBiome; size * size],
            explored: vec![true; size * size],
            visible_now: vec![true; size * size],
            full_visible_now: vec![false; size * size],
            known_resources: vec![None; size * size],
            known_rare: vec![None; size * size],
            known_buildings,
            known_enemy_units: vec![None; size * size],
            own_helpers: helpers,
            own_buildings: vec![OwnBuilding {
                pos: IVec2::new(0, 0),
                building_type: BuildingType::Capital,
                level: BuildingLevel::Standard,
            }],
            own_combat_units: vec![],
            combat_unit_costs: [0; 3],
            reveal_scrolls: 0,
            gold,
            food: 20,
            science: 10,
        }
    }

    fn normal_profile() -> AiProfile {
        AiProfile::from_difficulty(AiDifficulty::Normal)
    }

    #[test]
    fn hires_helper_when_affordable() {
        let snap = snapshot(5, vec![], 30);
        let plan = plan_bot_turn(snap, normal_profile());
        assert!(plan.actions.iter().any(
            |action| matches!(action, AiAction::SpawnUnit { city_pos, unit_type: UnitType::Helper } if *city_pos == IVec2::new(0, 0))
        ));
    }

    #[test]
    fn does_not_hire_without_gold() {
        let snap = snapshot(5, vec![], 9);
        let plan = plan_bot_turn(snap, normal_profile());
        assert!(
            !plan
                .actions
                .iter()
                .any(|action| matches!(action, AiAction::SpawnUnit { .. }))
        );
    }

    #[test]
    fn helper_gathers_nearby_gold() {
        let mut snap = snapshot(
            5,
            vec![OwnHelper {
                pos: IVec2::new(0, 0),
                movement_points: 2,
                charges: 3,
            }],
            5,
        );
        snap.known_resources[1] = Some(CommonKind::Gold);

        let plan = plan_bot_turn(snap, normal_profile());
        assert!(
            plan.actions.iter().any(
                |action| matches!(action, AiAction::Gather { pos } if *pos == IVec2::new(0, 1))
            )
        );
    }

    #[test]
    fn helper_without_charges_does_not_build() {
        let mut snap = snapshot(
            5,
            vec![OwnHelper {
                pos: IVec2::new(2, 2),
                movement_points: 2,
                charges: 0,
            }],
            100,
        );
        snap.known_rare[2 * 5 + 3] = Some(RareKind::ManaScroll);

        let plan = plan_bot_turn(snap, normal_profile());
        assert!(
            !plan
                .actions
                .iter()
                .any(|action| matches!(action, AiAction::Build { .. }))
        );
    }

    #[test]
    fn does_not_plan_unaffordable_building() {
        // Золота хватает только на найм? Нет — золота нет ни на что.
        let mut snap = snapshot(
            5,
            vec![OwnHelper {
                pos: IVec2::new(2, 2),
                movement_points: 2,
                charges: 3,
            }],
            5,
        );
        snap.known_resources[2 * 5 + 3] = Some(CommonKind::Gold);

        let plan = plan_bot_turn(snap, normal_profile());
        assert!(
            !plan
                .actions
                .iter()
                .any(|action| matches!(action, AiAction::Build { .. }))
        );
    }

    #[test]
    fn uses_reveal_scroll_on_unexplored_map() {
        let mut snap = snapshot(12, vec![], 5);
        // Известен только угол 6x6, остальное — туман.
        for row in 0..12 {
            for col in 0..12 {
                let index = row * 12 + col;
                let known = row < 6 && col < 6;
                snap.explored[index] = known;
                if !known {
                    snap.cost[index] = 0;
                    snap.biome[index] = BiomeType::Water;
                }
            }
        }
        snap.reveal_scrolls = 1;

        let plan = plan_bot_turn(snap, normal_profile());
        assert!(
            plan.actions
                .iter()
                .any(|action| matches!(action, AiAction::RevealMap { .. }))
        );
    }

    #[test]
    fn keeps_reveal_scroll_on_explored_map() {
        // Карта маленькая и полностью известна — свиток не тратим.
        let snap = snapshot(5, vec![], 5);
        let mut snap = snap;
        snap.reveal_scrolls = 1;

        let plan = plan_bot_turn(snap, normal_profile());
        assert!(
            !plan
                .actions
                .iter()
                .any(|action| matches!(action, AiAction::RevealMap { .. }))
        );
    }

    #[test]
    fn explores_fog_when_nothing_else_to_do() {
        let mut snap = snapshot(
            8,
            vec![OwnHelper {
                pos: IVec2::new(3, 3),
                movement_points: 2,
                charges: 3,
            }],
            5,
        );
        // Известен только угол 4x4, ресурсов нет.
        for row in 0..8 {
            for col in 0..8 {
                let index = row * 8 + col;
                let known = row < 4 && col < 4;
                snap.explored[index] = known;
                if !known {
                    snap.cost[index] = 0;
                    snap.biome[index] = BiomeType::Water;
                }
            }
        }

        let plan = plan_bot_turn(snap, normal_profile());
        assert!(
            plan.actions
                .iter()
                .any(|action| matches!(action, AiAction::Move { .. }))
        );
    }
}
