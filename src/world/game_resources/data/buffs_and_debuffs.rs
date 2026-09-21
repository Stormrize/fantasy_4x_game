use bevy::prelude::Component;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum Buff {
    Heal {
        health: u16,
        duration: u16,
        turns_offset: u16,
    },
    DamageBuff {
        damage: u16,
        duration: u16,
        turns_offset: u16,
    },
    DefenseBuff {
        defense: u16,
        duration: u16,
        turns_offset: u16,
    },
    HealthBuff {
        health: u16,
        duration: u16,
        turns_offset: u16,
    },
    IgnoreDefense {
        defense: u16,
        duration: u16,
        turns_offset: u16,
    },
    MovementBuff {
        movement_points: u16,
        duration: u16,
        turns_offset: u16,
    },
    DamageReflection {
        damage_percentage: u16,
        duration: u16,
        turns_offset: u16,
    },
}

#[derive(Component, Deserialize, Debug, Default, Clone)]
pub struct Buffs(pub Vec<Buff>);

#[derive(Deserialize, Debug, Clone)]
pub enum Debuff {
    SustainedDamage {
        damage: u16,
        duration: u16,
        turns_offset: u16, // after how many turns skill is going to be used
    },
    Stun {
        duration: u16, // cant do anything
        turns_offset: u16,
    },
    Root {
        duration: u16, // cant walk
        turns_offset: u16,
    },
    Silence {
        duration: u16, // cant use skills
        turns_offset: u16,
    },
    Parasite {
        // less hp every turn
        health: u16,
        duration: u16,
        turns_offset: u16,
    },
    DamageDebuff {
        // less hp
        attack: u16,
        duration: u16,
        turns_offset: u16,
    },
    DefenseDebuff {
        // less defense
        defense: u16,
        duration: u16,
        turns_offset: u16,
    },
    HealthDebuff {
        // less health
        health: u16,
        duration: u16,
        turns_offset: u16,
    },
    MovementDebuff {
        // less movement points
        movement_points: u16,
        duration: u16,
        turns_offset: u16,
    },
}

#[derive(Component, Deserialize, Debug, Default, Clone)]
pub struct Debuffs(pub Vec<Debuff>);
