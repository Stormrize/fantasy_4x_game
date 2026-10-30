use super::model::{CombatCommand, Combatant};
use crate::world::components::Mana;
use crate::world::game_resources::data::buffs_and_debuffs::{Buff, Debuff};
use bevy::math::IVec2;

fn distance(a: IVec2, b: IVec2) -> u16 {
    (a.x - b.x).unsigned_abs().max((a.y - b.y).unsigned_abs()) as u16
}

/// Боевой ИИ: жадный выбор команд для своих юнитов.
///
/// Работает на фоне (`AsyncComputeTaskPool`), пока игрок готовит атаки.
/// Пока это жадная эвристика с оценкой действия; в дальнейшем можно
/// расширить до перебора «мои команды × вероятные команды врага» + оценка
/// позиции через `simulate`.
pub fn plan_combat(own: &[Combatant], enemies: &[Combatant], mut mana: Mana) -> Vec<CombatCommand> {
    let mut commands = Vec::new();

    let mut own_sorted: Vec<&Combatant> = own.iter().collect();
    own_sorted.sort_by_key(|c| (c.pos.x, c.pos.y));

    for unit in own_sorted {
        if !unit.is_alive() {
            continue;
        }
        if let Some((command, cost)) = best_action(unit, own, enemies, mana) {
            commands.push(command);
            mana = mana.saturating_sub(cost);
        }
    }

    commands
}

/// Лучшее действие одного юнита: (команда, стоимость маны).
fn best_action(
    unit: &Combatant,
    own: &[Combatant],
    enemies: &[Combatant],
    mana: Mana,
) -> Option<(CombatCommand, Mana)> {
    let mut best: Option<(f32, CombatCommand, Mana)> = None;
    let mut consider = |score: f32, command: CombatCommand, cost: Mana| {
        if best
            .as_ref()
            .is_none_or(|(best_score, _, _)| score > *best_score)
        {
            best = Some((score, command, cost));
        }
    };

    // Базовая атака (без маны).
    for enemy in enemies {
        if !enemy.is_alive() || distance(unit.pos, enemy.pos) > unit.base.range {
            continue;
        }
        let damage = unit.attack().saturating_sub(enemy.defense()).max(1);
        let score = damage as f32 + kill_bonus(enemy, damage);
        consider(
            score,
            CombatCommand {
                attacker_pos: unit.pos,
                target_pos: enemy.pos,
                skill_type: 0,
            },
            0,
        );
    }

    // Способности.
    for index in 0..unit.available_abilities() {
        let state = &unit.abilities[index];
        if state.current_cooldown > 0 || mana < state.ability.mana_amount_needs {
            continue;
        }
        let cost = state.ability.mana_amount_needs;

        if state.ability.damage > 0 {
            for enemy in enemies {
                if !enemy.is_alive() || distance(unit.pos, enemy.pos) > state.ability.range as u16 {
                    continue;
                }
                let ignore = state
                    .ability
                    .debuffs
                    .0
                    .iter()
                    .any(|d| matches!(d, Debuff::IgnoreDefense));
                let defense = if ignore { 0 } else { enemy.defense() };
                let damage = state.ability.damage.saturating_sub(defense).max(1);
                let score = damage as f32 + kill_bonus(enemy, damage);
                consider(
                    score,
                    CombatCommand {
                        attacker_pos: unit.pos,
                        target_pos: enemy.pos,
                        skill_type: index as u8 + 1,
                    },
                    cost,
                );
            }
        } else {
            // Способность без урона — хил/бафф. Выбираем цель по эффекту.
            for buff in &state.ability.buffs.0 {
                let (target, value) = buff_target(unit, own, state.ability.range as u16, buff);
                if let Some(target) = target {
                    consider(
                        value,
                        CombatCommand {
                            attacker_pos: unit.pos,
                            target_pos: target,
                            skill_type: index as u8 + 1,
                        },
                        cost,
                    );
                }
            }
        }
    }

    best.map(|(_, command, cost)| (command, cost))
}

/// Бонус за добивание цели: предпочитаем убивать раненых.
fn kill_bonus(enemy: &Combatant, damage: u16) -> f32 {
    if damage >= enemy.health {
        enemy.health as f32 * 2.0
    } else {
        0.0
    }
}

/// Цель для баффа/хила и ценность применения.
fn buff_target(
    unit: &Combatant,
    own: &[Combatant],
    range: u16,
    buff: &Buff,
) -> (Option<IVec2>, f32) {
    let mut best: Option<(IVec2, f32)> = None;
    for ally in own {
        if !ally.is_alive() || distance(unit.pos, ally.pos) > range {
            continue;
        }
        let value = match buff {
            Buff::Heal { health, .. } => {
                let missing = ally.max_health.saturating_sub(ally.health);
                (*health).min(missing) as f32 * 1.5
            }
            Buff::DamageBuff { damage, .. } => {
                // Бафф атаки ценнее на дамагерах.
                let type_weight = if matches!(
                    ally.unit_type,
                    crate::world::game_resources::data::unit::UnitType::DamageDealer
                ) {
                    1.5
                } else {
                    1.0
                };
                *damage as f32 * type_weight
            }
            Buff::DefenseBuff { defense, .. } => {
                // Защита ценнее на раненых танках.
                let low_hp = (ally.health as f32) / (ally.max_health.max(1) as f32) < 0.5;
                *defense as f32 * if low_hp { 1.5 } else { 1.0 }
            }
            Buff::HealthBuff { health, .. } => *health as f32,
            _ => 1.0,
        };
        if best.is_none_or(|(_, best_value)| value > best_value) {
            best = Some((ally.pos, value));
        }
    }
    best.map_or((None, 0.0), |(pos, value)| (Some(pos), value))
}
