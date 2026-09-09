use bevy::prelude::Component;

#[derive(Debug, Clone)]
pub enum Buff {
    Heal {
        health: u16,
        duration: u16,
        turns_offset: u16,
    },
    DamageBuff {
        health: u16,
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
}

#[derive(Component, Debug, Clone)]
pub struct ActiveBuffs(pub Vec<Buff>);

#[derive(Debug, Clone)]
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

#[derive(Component, Debug, Clone)]
pub struct ActiveDebuffs(pub Vec<Debuff>);
