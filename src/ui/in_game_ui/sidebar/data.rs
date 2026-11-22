//! Plain view structs gathered from the ECS for one rebuild.

use bevy::prelude::*;

use crate::game::game_logic_plugin::components::TileContent;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType};
use crate::world::game_resources::data::resource::{CommonKind, RareKind, ResourceType};
use crate::world::game_resources::data::unit::{UnitLevel, UnitType};

use super::format::{common_color, common_kind_key, rare_kind_key};
use super::grid::GridIndex;

// Units

pub(crate) struct AbilityView {
    pub name: String,
    pub damage: u16,
    pub mana: u16,
    pub cooldown: u16,
    pub range: u8,
    pub area: u8,
}

pub(crate) struct UnitView {
    pub pos: IVec2,
    pub owner: Id,
    pub race: PlayerRace,
    pub kind: UnitType,
    pub level: Option<UnitLevel>,
    pub hp: u16,
    pub max_hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub range: u16,
    pub moves: u16,
    pub moves_max: u16,
    pub build_points: Option<u8>,
    pub abilities: Vec<AbilityView>,
    pub tile_content: TileContent,
    /// The local player can currently see this unit (not hidden by fog).
    pub visible: bool,
}

// Buildings

pub(crate) struct BuildingView {
    pub pos: IVec2,
    pub owner: Id,
    pub kind: BuildingType,
    pub level: BuildingLevel,
    pub gold: u16,
    pub food: u16,
    pub science: u16,
    pub housing: u16,
    pub spawn_capacity: u16,
    pub maintenance: u16,
}

pub(crate) type UnitIndex = GridIndex<UnitView>;
pub(crate) type BuildingIndex = GridIndex<BuildingView>;

// Discovery

/// One discovered biome type. Only unique types are listed, never every tile.
pub(crate) struct BiomeEntry {
    pub kind: BiomeType,
    pub tiles: usize,
}

/// One discovered resource kind. Only unique kinds are listed.
pub(crate) enum ResourceEntry {
    Common {
        kind: CommonKind,
        tiles: usize,
        amount: u32,
    },
    Rare {
        kind: RareKind,
        tiles: usize,
        power: u32,
    },
}

impl ResourceEntry {
    pub(crate) fn matches(&self, resource: &ResourceType) -> bool {
        match (self, resource) {
            (Self::Common { kind, .. }, ResourceType::Common(common)) => *kind == common.kind,
            (Self::Rare { kind, .. }, ResourceType::Rare(rare)) => *kind == rare.kind,
            _ => false,
        }
    }

    pub(crate) fn key(&self) -> &'static str {
        match self {
            Self::Common { kind, .. } => common_kind_key(*kind),
            Self::Rare { kind, .. } => rare_kind_key(*kind),
        }
    }

    pub(crate) fn tiles(&self) -> usize {
        match self {
            Self::Common { tiles, .. } | Self::Rare { tiles, .. } => *tiles,
        }
    }

    pub(crate) fn detail(&self, ui: &UiContext, language: Language) -> String {
        match self {
            Self::Common { amount, .. } => {
                format!("{}: {amount}", ui.get(language, "amount"))
            }
            Self::Rare { power, .. } => format!("{}: {power}", ui.get(language, "power")),
        }
    }

    pub(crate) fn color(&self) -> Color {
        match self {
            Self::Common { kind, .. } => common_color(*kind),
            Self::Rare { .. } => theme::GOLD_TEXT,
        }
    }
}

/// Aggregated view of everything the player has explored.
pub(crate) struct Discovery {
    pub biomes: Vec<BiomeEntry>,
    pub resources: Vec<ResourceEntry>,
    pub resource_tiles: usize,
}
