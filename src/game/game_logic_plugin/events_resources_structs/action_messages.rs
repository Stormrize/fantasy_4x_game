use crate::game::game_logic_plugin::events_resources_structs::components::{
    JustSelectedTile, TileLayer,
};
use crate::world::game_resources::data::building::BuildingType;
use crate::world::game_resources::data::unit::UnitType;
use bevy::prelude::{IVec2, Message};

// tile clicked
#[derive(Message, Debug)]
pub struct TileClickedEvent {
    pub pos: IVec2,
}

// selection systems
#[derive(Message, Debug, Clone)]
pub struct Cycle(pub JustSelectedTile);

#[derive(Message, Debug, Clone)]
pub struct NewTile(pub JustSelectedTile);

#[derive(Message, Debug, Clone)]
pub struct Clear;

// general
#[derive(Message)]
pub struct EndTurn;

#[derive(Message)]
pub struct WorldView;

// All actions(player and AI) are send in one message-union.
#[derive(Message, Debug, Clone, Copy)]
pub enum Action {
    Upgrade {
        entity_pos: IVec2,
        layer: TileLayer,
    },
    Delete {
        entity_pos: IVec2,
        layer: TileLayer,
    },
    Info {
        entity_pos: IVec2,
        layer: TileLayer,
    },
    Move {
        unit_pos: IVec2,
        target_pos: IVec2,
    },
    /// `skill_type: 0` — basic skill (no mana), `1..` — actual skills from
    /// `unique_level_abilities`.
    Attack {
        unit_pos: IVec2,
        target_pos: IVec2,
        skill_type: u8,
    },
    Gather {
        resource_pos: IVec2,
    },
    Build {
        entity_pos: IVec2,
        /// `None` — здание по умолчанию для биома клетки.
        building_type: Option<BuildingType>,
    },
    /// Наём юнита в городе на `city_pos`.
    /// `unit_type: Helper` — строитель, остальные типы — боевые юниты.
    SpawnUnit {
        city_pos: IVec2,
        unit_type: UnitType,
    },
    RevealMap {
        center: IVec2,
        reveal_source: RevealSource,
        full: bool,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum RevealSource {
    Unit,
    Building,
    RevealScroll,
}
