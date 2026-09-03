use crate::world::game_config::PlayerRace;
use crate::world::ui_and_graphic_resources::player_registry::Id;
use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum UnitType {
    //вода
    WaterMageBeginner,
    WaterMageMaster,

    //огонь
    FireMageBeginner,
    FireMageMaster,

    //камень
    StoneMageBeginner,
    StoneMageMaster,

    ResourceGathererBuilder,
}

#[derive(Component, Debug)]
pub struct Health(u8);

#[derive(Component, Debug)]
pub struct Attack(u8);

#[derive(Component, Debug)]
pub struct Defense(u8);

pub struct Unit {
    pub player_id: Id,
    pub player_race: PlayerRace,
    pub unit: UnitType,
    pub attack: u8,
    pub defense: u8,
    pub health: u8,
}

impl UnitType {
    pub fn index(&self) -> usize {
        use UnitType::*;
        match self {
            WaterMageBeginner => 0,
            WaterMageMaster => 1,
            FireMageBeginner => 2,
            FireMageMaster => 3,
            StoneMageBeginner => 4,
            StoneMageMaster => 5,
            ResourceGathererBuilder => 6,
        }
    }
}
