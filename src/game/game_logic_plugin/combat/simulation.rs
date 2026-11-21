#![allow(clippy::collapsible_if, clippy::needless_range_loop)]

use super::model::{CombatCommand, CombatOutcome, CombatPlayer, Combatant};
use crate::world::components::{Id, Mana};
use crate::world::game_resources::data::buffs_and_debuffs::{Buff, Debuff};
use crate::world::game_resources::data::unit::{UniqueLevelAbility, UnitType};
use bevy::math::IVec2;
use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque};

/// Чебышевская дистанция на глобальной карте.
fn distance(a: IVec2, b: IVec2) -> u16 {
    (a.x - b.x).unsigned_abs().max((a.y - b.y).unsigned_abs()) as u16
}

/// Разрешает боевые команды в порядке: Support → Skill → DamageDealer.
/// Внутри категории игроки чередуются по одной команде; если у кого-то
/// команды кончились, остальные ходят подряд. Взаимные атаки разрешаются
/// одновременно.
pub fn simulate(
    mut combatants: Vec<Combatant>,
    mut players: Vec<CombatPlayer>,
    commands: &[CombatCommand],
) -> CombatOutcome {
    let mut log: Vec<String> = Vec::new();
    let mut handled = vec![false; commands.len()];

    // 1. Взаимные атаки — одновременно.
    for i in 0..commands.len() {
        if handled[i] {
            continue;
        }
        for j in (i + 1)..commands.len() {
            if handled[j] {
                continue;
            }
            if commands[i].attacker_pos == commands[j].target_pos
                && commands[i].target_pos == commands[j].attacker_pos
            {
                resolve_mutual(
                    &mut combatants,
                    &mut players,
                    &commands[i],
                    &commands[j],
                    &mut log,
                );
                handled[i] = true;
                handled[j] = true;
                break;
            }
        }
    }

    // 2. Остальные — по категориям.
    for category in [UnitType::Support, UnitType::Skill, UnitType::DamageDealer] {
        let mut indices: Vec<usize> = (0..commands.len())
            .filter(|&i| {
                !handled[i]
                    && attacker_type(&combatants, commands[i].attacker_pos) == Some(category)
            })
            .collect();

        sort_indices(&combatants, commands, &mut indices, category);
        resolve_round_robin(&mut combatants, &mut players, commands, &indices, &mut log);

        for &i in &indices {
            handled[i] = true;
        }
    }

    CombatOutcome {
        combatants,
        players,
        log,
    }
}

fn attacker_type(combatants: &[Combatant], pos: IVec2) -> Option<UnitType> {
    combatants
        .iter()
        .find(|c| c.pos == pos && c.is_alive())
        .map(|c| c.unit_type)
}

/// Сортировка команд внутри категории.
fn sort_indices(
    combatants: &[Combatant],
    commands: &[CombatCommand],
    indices: &mut [usize],
    category: UnitType,
) {
    match category {
        UnitType::Support => {
            // Сначала способности без урона (баффы/хилы).
            indices.sort_by_key(|&i| ability_damage(combatants, commands[i]) != 0);
        }
        UnitType::Skill => {
            // Произвольный, но детерминированный порядок.
            indices.sort_by_key(|&i| (commands[i].target_pos.x, commands[i].target_pos.y));
        }
        UnitType::DamageDealer => {
            // От самых мощных к слабым.
            indices.sort_by_key(|&i| Reverse(ability_damage(combatants, commands[i])));
        }
        UnitType::Helper => {}
    }
}

/// Урон способности/базовой атаки команды (для сортировки).
fn ability_damage(combatants: &[Combatant], command: CombatCommand) -> u16 {
    if command.skill_type == 0 {
        return combatants
            .iter()
            .find(|c| c.pos == command.attacker_pos)
            .map(|c| c.attack())
            .unwrap_or(0);
    }
    combatants
        .iter()
        .find(|c| c.pos == command.attacker_pos)
        .and_then(|c| {
            c.abilities
                .get(command.skill_type as usize - 1)
                .map(|a| a.ability.damage)
        })
        .unwrap_or(0)
}

/// Разрешение оставшихся команд с чередованием игроков.
fn resolve_round_robin(
    combatants: &mut [Combatant],
    players: &mut [CombatPlayer],
    commands: &[CombatCommand],
    indices: &[usize],
    log: &mut Vec<String>,
) {
    // Группируем команды по владельцу атакующего.
    let mut owners: Vec<Id> = indices
        .iter()
        .filter_map(|&i| attacker_owner(combatants, commands[i].attacker_pos))
        .collect();
    owners.sort_by_key(|id| id.0);
    owners.dedup();

    let mut queues: HashMap<u16, VecDeque<usize>> = HashMap::new();
    for &i in indices {
        if let Some(owner) = attacker_owner(combatants, commands[i].attacker_pos) {
            queues.entry(owner.0).or_default().push_back(i);
        }
    }

    loop {
        let mut progressed = false;
        for owner in &owners {
            if let Some(queue) = queues.get_mut(&owner.0) {
                if let Some(i) = queue.pop_front() {
                    resolve_one(combatants, players, commands[i], log);
                    progressed = true;
                }
            }
        }
        if !progressed {
            break;
        }
    }
}

fn attacker_owner(combatants: &[Combatant], pos: IVec2) -> Option<Id> {
    combatants.iter().find(|c| c.pos == pos).map(|c| c.owner)
}

/// Взаимная атака: оба юнита получают урон одновременно.
fn resolve_mutual(
    combatants: &mut [Combatant],
    players: &mut [CombatPlayer],
    a: &CombatCommand,
    b: &CombatCommand,
    log: &mut Vec<String>,
) {
    let damage_a = compute_command_damage(combatants, players, a);
    let damage_b = compute_command_damage(combatants, players, b);

    apply_damage(combatants, a.target_pos, damage_a, log);
    apply_damage(combatants, b.target_pos, damage_b, log);
    log.push(format!(
        "mutual: {}<->{} ({} / {})",
        a.attacker_pos, b.attacker_pos, damage_a, damage_b
    ));
}

/// Вычисляет урон команды (с валидацией и списанием маны/кулдауна).
fn compute_command_damage(
    combatants: &mut [Combatant],
    players: &mut [CombatPlayer],
    command: &CombatCommand,
) -> u16 {
    let Some(attacker_idx) = index_at(combatants, command.attacker_pos) else {
        return 0;
    };
    if !combatants[attacker_idx].is_alive() {
        return 0;
    }

    if command.skill_type == 0 {
        // Базовая атака.
        let Some(target_idx) = index_at(combatants, command.target_pos) else {
            return 0;
        };
        let range = combatants[attacker_idx].base.range;
        if distance(command.attacker_pos, command.target_pos) > range {
            return 0;
        }
        let attack = combatants[attacker_idx].attack();
        let defense = combatants[target_idx].defense();
        attack.saturating_sub(defense).max(1)
    } else {
        // Способность.
        let ability_idx = command.skill_type as usize - 1;
        let available = combatants[attacker_idx].available_abilities();
        if ability_idx >= available {
            return 0;
        }
        let cooldown = combatants[attacker_idx].abilities[ability_idx].current_cooldown;
        if cooldown > 0 {
            return 0;
        }
        let mana_cost = combatants[attacker_idx].abilities[ability_idx]
            .ability
            .mana_amount_needs;
        let owner = combatants[attacker_idx].owner;
        if !spend_mana(players, owner, mana_cost) {
            return 0;
        }
        let range = combatants[attacker_idx].abilities[ability_idx]
            .ability
            .range as u16;
        if distance(command.attacker_pos, command.target_pos) > range {
            return 0;
        }
        combatants[attacker_idx].abilities[ability_idx].current_cooldown = combatants[attacker_idx]
            .abilities[ability_idx]
            .ability
            .cool_down_turns;

        let ability = &combatants[attacker_idx].abilities[ability_idx].ability;
        let damage = ability.damage;
        let ignore_defense = ability
            .debuffs
            .0
            .iter()
            .any(|d| matches!(d, Debuff::IgnoreDefense));
        if damage == 0 {
            return 0;
        }
        let Some(target_idx) = index_at(combatants, command.target_pos) else {
            return 0;
        };
        if ignore_defense {
            damage.max(1)
        } else {
            damage
                .saturating_sub(combatants[target_idx].defense())
                .max(1)
        }
    }
}

/// Разрешение одной команды (пошаговое).
fn resolve_one(
    combatants: &mut [Combatant],
    players: &mut [CombatPlayer],
    command: CombatCommand,
    log: &mut Vec<String>,
) {
    let Some(attacker_idx) = index_at(combatants, command.attacker_pos) else {
        return;
    };
    if !combatants[attacker_idx].is_alive() {
        return;
    }

    let mut ability: Option<(usize, UniqueLevelAbility)> = None;
    let range: u16;
    if command.skill_type == 0 {
        range = combatants[attacker_idx].base.range;
    } else {
        let ability_idx = command.skill_type as usize - 1;
        if ability_idx >= combatants[attacker_idx].available_abilities() {
            return;
        }
        let cloned = combatants[attacker_idx].abilities[ability_idx]
            .ability
            .clone();
        if combatants[attacker_idx].abilities[ability_idx].current_cooldown > 0 {
            return;
        }
        let owner = combatants[attacker_idx].owner;
        if !spend_mana(players, owner, cloned.mana_amount_needs) {
            return;
        }
        range = cloned.range as u16;
        combatants[attacker_idx].abilities[ability_idx].current_cooldown = cloned.cool_down_turns;
        ability = Some((ability_idx, cloned));
    }

    if distance(command.attacker_pos, command.target_pos) > range {
        return;
    }

    match ability {
        None => {
            // Базовая атака: урон по цели.
            let attack = combatants[attacker_idx].attack();
            let target_idx = index_at(combatants, command.target_pos);
            let Some(target_idx) = target_idx else { return };
            let damage = attack
                .saturating_sub(combatants[target_idx].defense())
                .max(1);
            apply_damage(combatants, command.target_pos, damage, log);
            log.push(format!(
                "basic: {} -> {} ({})",
                command.attacker_pos, command.target_pos, damage
            ));
        }
        Some((_, ability)) => {
            apply_ability(
                combatants,
                command.attacker_pos,
                command.target_pos,
                &ability,
                log,
            );
        }
    }
}

/// Применяет способность (урон + баффы/дебаффы), с учётом области.
fn apply_ability(
    combatants: &mut [Combatant],
    attacker_pos: IVec2,
    target_pos: IVec2,
    ability: &UniqueLevelAbility,
    log: &mut Vec<String>,
) {
    let Some(attacker_idx) = index_at(combatants, attacker_pos) else {
        return;
    };
    let owner = combatants[attacker_idx].owner;
    let area = ability.area as u16;

    // Урон врагам в области.
    if ability.damage > 0 {
        let ignore_defense = ability
            .debuffs
            .0
            .iter()
            .any(|d| matches!(d, Debuff::IgnoreDefense));
        for target_idx in 0..combatants.len() {
            if combatants[target_idx].owner == owner || !combatants[target_idx].is_alive() {
                continue;
            }
            if area == 0 && combatants[target_idx].pos != target_pos {
                continue;
            }
            if area > 0 && distance(target_pos, combatants[target_idx].pos) > area {
                continue;
            }
            let defense = if ignore_defense {
                0
            } else {
                combatants[target_idx].defense()
            };
            let damage = ability.damage.saturating_sub(defense).max(1);
            combatants[target_idx].health = combatants[target_idx].health.saturating_sub(damage);
            log.push(format!(
                "ability {}: {} -> {} ({})",
                ability.name, attacker_pos, combatants[target_idx].pos, damage
            ));
        }
    }

    // Баффы — своим в области (включая себя).
    for buff in &ability.buffs.0 {
        for target_idx in 0..combatants.len() {
            if combatants[target_idx].owner != owner || !combatants[target_idx].is_alive() {
                continue;
            }
            if area == 0 && combatants[target_idx].pos != target_pos {
                continue;
            }
            if area > 0 && distance(target_pos, combatants[target_idx].pos) > area {
                continue;
            }
            apply_buff(&mut combatants[target_idx], buff);
        }
    }

    // Дебаффы — врагам в области (кроме IgnoreDefense, который уже учтён).
    for debuff in &ability.debuffs.0 {
        if matches!(debuff, Debuff::IgnoreDefense) {
            continue;
        }
        for target_idx in 0..combatants.len() {
            if combatants[target_idx].owner == owner || !combatants[target_idx].is_alive() {
                continue;
            }
            if area == 0 && combatants[target_idx].pos != target_pos {
                continue;
            }
            if area > 0 && distance(target_pos, combatants[target_idx].pos) > area {
                continue;
            }
            apply_debuff(&mut combatants[target_idx], debuff);
        }
    }
}

fn apply_buff(unit: &mut Combatant, buff: &Buff) {
    match buff {
        Buff::Heal { health, .. } => {
            unit.health = unit.health.saturating_add(*health).min(unit.max_health);
        }
        Buff::DamageBuff { damage, .. } => unit.attack_mod += *damage as i32,
        Buff::DefenseBuff { defense, .. } => unit.defense_mod += *defense as i32,
        Buff::HealthBuff { health, .. } => {
            unit.max_health = unit.max_health.saturating_add(*health);
            unit.health = unit.health.saturating_add(*health);
        }
        Buff::Cleanse => {
            unit.attack_mod = unit.attack_mod.max(0);
            unit.defense_mod = unit.defense_mod.max(0);
        }
        // Остальные баффы в однораундовой симуляции не влияют на исход.
        _ => {}
    }
}

fn apply_debuff(unit: &mut Combatant, debuff: &Debuff) {
    match debuff {
        Debuff::SustainedDamage { damage, .. } => {
            unit.health = unit.health.saturating_sub(*damage);
        }
        Debuff::DamageDebuff { attack, .. } => unit.attack_mod -= *attack as i32,
        Debuff::DefenseDebuff { defense, .. } => unit.defense_mod -= *defense as i32,
        Debuff::HealthDebuff { health, .. } => {
            unit.max_health = unit.max_health.saturating_sub(*health);
            unit.health = unit.health.min(unit.max_health);
        }
        // Stun/Freeze/Root/Silence/Slowdown и т.п. — эффект на следующие ходы.
        _ => {}
    }
}

fn apply_damage(combatants: &mut [Combatant], pos: IVec2, damage: u16, log: &mut Vec<String>) {
    if let Some(idx) = index_at(combatants, pos) {
        combatants[idx].health = combatants[idx].health.saturating_sub(damage);
        log.push(format!("damage to {}: {}", pos, damage));
    }
}

fn spend_mana(players: &mut [CombatPlayer], owner: Id, cost: Mana) -> bool {
    if let Some(player) = players.iter_mut().find(|p| p.id == owner) {
        if player.mana >= cost {
            player.mana -= cost;
            return true;
        }
    }
    false
}

fn index_at(combatants: &[Combatant], pos: IVec2) -> Option<usize> {
    combatants.iter().position(|c| c.pos == pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::components::Id;
    use crate::world::game_resources::data::buffs_and_debuffs::{Buffs, Debuffs};
    use crate::world::game_resources::data::unit::{CombatBasicStats, UnitLevel, UnitType};

    fn ability(name: &str, damage: u16, mana: u16, range: u8, area: u8) -> UniqueLevelAbility {
        UniqueLevelAbility {
            name: name.into(),
            buffs: Buffs::default(),
            debuffs: Debuffs::default(),
            mana_amount_needs: mana,
            damage,
            cool_down_turns: 1,
            range,
            area,
        }
    }

    fn combatant(
        owner: u16,
        pos: (i32, i32),
        unit_type: UnitType,
        attack: u16,
        defense: u16,
        health: u16,
        abilities: Vec<UniqueLevelAbility>,
    ) -> Combatant {
        Combatant::new(
            Id(owner),
            IVec2::new(pos.0, pos.1),
            unit_type,
            UnitLevel::Beginner,
            CombatBasicStats {
                health,
                defense,
                attack,
                range: 1,
            },
            abilities,
        )
    }

    #[test]
    fn basic_attack_subtracts_defense() {
        let attacker = combatant(1, (0, 0), UnitType::DamageDealer, 30, 0, 100, vec![]);
        let target = combatant(2, (1, 0), UnitType::Support, 0, 10, 100, vec![]);
        let players = vec![CombatPlayer { id: Id(1), mana: 0 }];
        let commands = vec![CombatCommand {
            attacker_pos: IVec2::new(0, 0),
            target_pos: IVec2::new(1, 0),
            skill_type: 0,
        }];

        let outcome = simulate(vec![attacker, target], players, &commands);
        let target = outcome
            .combatants
            .iter()
            .find(|c| c.pos == IVec2::new(1, 0))
            .unwrap();
        assert_eq!(target.health, 80); // 100 - (30 - 10)
    }

    #[test]
    fn mutual_attack_is_simultaneous() {
        let a = combatant(1, (0, 0), UnitType::DamageDealer, 30, 0, 40, vec![]);
        let b = combatant(2, (1, 0), UnitType::DamageDealer, 30, 0, 40, vec![]);
        let players = vec![];
        let commands = vec![
            CombatCommand {
                attacker_pos: IVec2::new(0, 0),
                target_pos: IVec2::new(1, 0),
                skill_type: 0,
            },
            CombatCommand {
                attacker_pos: IVec2::new(1, 0),
                target_pos: IVec2::new(0, 0),
                skill_type: 0,
            },
        ];

        let outcome = simulate(vec![a, b], players, &commands);
        let a = outcome
            .combatants
            .iter()
            .find(|c| c.pos == IVec2::new(0, 0))
            .unwrap();
        let b = outcome
            .combatants
            .iter()
            .find(|c| c.pos == IVec2::new(1, 0))
            .unwrap();
        // Оба выжили и получили урон, потому что атака одновременна.
        assert_eq!(a.health, 10);
        assert_eq!(b.health, 10);
    }

    #[test]
    fn categories_resolve_in_order() {
        let support = combatant(
            1,
            (0, 0),
            UnitType::Support,
            0,
            0,
            100,
            vec![ability("S", 10, 5, 3, 0)],
        );
        let skill = combatant(
            1,
            (0, 1),
            UnitType::Skill,
            0,
            0,
            100,
            vec![ability("K", 20, 5, 3, 0)],
        );
        let dd = combatant(1, (0, 2), UnitType::DamageDealer, 30, 0, 100, vec![]);
        let target = combatant(2, (1, 1), UnitType::DamageDealer, 0, 0, 1000, vec![]);

        let players = vec![CombatPlayer {
            id: Id(1),
            mana: 100,
        }];
        let commands = vec![
            CombatCommand {
                attacker_pos: IVec2::new(0, 2),
                target_pos: IVec2::new(1, 1),
                skill_type: 0,
            },
            CombatCommand {
                attacker_pos: IVec2::new(0, 0),
                target_pos: IVec2::new(1, 1),
                skill_type: 1,
            },
            CombatCommand {
                attacker_pos: IVec2::new(0, 1),
                target_pos: IVec2::new(1, 1),
                skill_type: 1,
            },
        ];

        let outcome = simulate(vec![support, skill, dd, target], players, &commands);

        let support_pos = outcome
            .log
            .iter()
            .position(|l| l.contains("ability S"))
            .unwrap();
        let skill_pos = outcome
            .log
            .iter()
            .position(|l| l.contains("ability K"))
            .unwrap();
        let dd_pos = outcome
            .log
            .iter()
            .position(|l| l.contains("basic"))
            .unwrap();
        assert!(support_pos < skill_pos);
        assert!(skill_pos < dd_pos);
    }

    #[test]
    fn ability_spends_mana() {
        let caster = combatant(
            1,
            (0, 0),
            UnitType::Skill,
            0,
            0,
            100,
            vec![ability("Bolt", 50, 20, 3, 0)],
        );
        let target = combatant(2, (1, 0), UnitType::DamageDealer, 0, 0, 100, vec![]);
        let players = vec![CombatPlayer {
            id: Id(1),
            mana: 50,
        }];
        let commands = vec![CombatCommand {
            attacker_pos: IVec2::new(0, 0),
            target_pos: IVec2::new(1, 0),
            skill_type: 1,
        }];

        let outcome = simulate(vec![caster, target], players, &commands);
        assert_eq!(outcome.players[0].mana, 30);
    }
}
