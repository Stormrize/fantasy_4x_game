use bevy::prelude::Component;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum Buff {
    Heal {
        health: u16,
        duration: u16,
        turns_offset: u16,
    },
    Cleanse,
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
    MovementBuff {
        movement_points: u16,
        duration: u16,
        turns_offset: u16,
    },
    DamageReflection {
        damage: u16,
        duration: u16,
        turns_offset: u16,
    },
    Summon {
        attack: u8,
        defense: u8,
        health: u8,
        range: u8,
        movement_points: u8,
        duration: u8,
    },
    VulnerabilityBuff {
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
    Freeze {
        duration: u16,
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
    IgnoreDefense,
    HealthDebuff {
        // less health
        health: u16,
        duration: u16,
        turns_offset: u16,
    },
    Slowdown {
        // less movement points
        movement_points: u16,
        duration: u16,
        turns_offset: u16,
    },
    RangeDebuff {
        range: u16,
        duration: u16,
        turns_offset: u16,
    },
}

#[derive(Component, Deserialize, Debug, Default, Clone)]
pub struct Debuffs(pub Vec<Debuff>);
