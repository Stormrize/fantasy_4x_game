use crate::world::components::{Id, Mana};
use crate::world::game_resources::data::unit::{
    CombatBasicStats, UniqueLevelAbility, UnitLevel, UnitType,
};
use bevy::math::IVec2;

/// Снимок боевого юнита для симуляции (независим от ECS).
#[derive(Debug, Clone)]
pub struct Combatant {
    pub owner: Id,
    pub pos: IVec2,
    pub unit_type: UnitType,
    pub level: UnitLevel,
    pub base: CombatBasicStats,
    pub max_health: u16,
    pub health: u16,
    /// Модификаторы статов от баффов/дебаффов (действуют в течение раунда).
    pub attack_mod: i32,
    pub defense_mod: i32,
    /// Способности с текущими кулдаунами.
    pub abilities: Vec<AbilityState>,
}

#[derive(Debug, Clone)]
pub struct AbilityState {
    pub ability: UniqueLevelAbility,
    pub current_cooldown: u16,
}

impl Combatant {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        owner: Id,
        pos: IVec2,
        unit_type: UnitType,
        level: UnitLevel,
        base: CombatBasicStats,
        abilities: Vec<UniqueLevelAbility>,
    ) -> Self {
        let max_health = base.health;
        Self {
            owner,
            pos,
            unit_type,
            level,
            base,
            max_health,
            health: max_health,
            attack_mod: 0,
            defense_mod: 0,
            abilities: abilities
                .into_iter()
                .map(|ability| AbilityState {
                    ability,
                    current_cooldown: 0,
                })
                .collect(),
        }
    }

    pub fn attack(&self) -> u16 {
        (self.base.attack as i32 + self.attack_mod).max(0) as u16
    }

    pub fn defense(&self) -> u16 {
        (self.base.defense as i32 + self.defense_mod).max(0) as u16
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Сколько способностей доступно на текущем уровне (по одной за уровень).
    pub fn available_abilities(&self) -> usize {
        (self.level as usize + 1).min(self.abilities.len())
    }
}

/// Боевая команда: юнит на `attacker_pos` действует по `target_pos`.
#[derive(Debug, Clone, Copy)]
pub struct CombatCommand {
    pub attacker_pos: IVec2,
    pub target_pos: IVec2,
    /// `0` — базовая атака, `1..=N` — способность.
    pub skill_type: u8,
}

/// Глобальная мана игрока.
#[derive(Debug, Clone, Copy)]
pub struct CombatPlayer {
    pub id: Id,
    pub mana: Mana,
}

/// Результат применения команд.
#[derive(Debug, Clone)]
pub struct CombatOutcome {
    pub combatants: Vec<Combatant>,
    pub players: Vec<CombatPlayer>,
    /// Лог действий (для отладки и проверки порядка).
    pub log: Vec<String>,
}
