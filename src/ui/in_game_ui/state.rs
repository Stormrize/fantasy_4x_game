//! Marker components and small resources shared by the in-game HUD modules.

use bevy::prelude::*;

use crate::game::game_logic_plugin::components::TileLayer;

/// Marker for the persistent left sidebar. It is visible in both phases but
/// shows different content during combat.
#[derive(Component)]
pub struct HudSidebar;

/// Container whose children are rebuilt from the current game state.
#[derive(Component)]
pub struct SidebarBody;

/// Container with one row per player in the relations overlay.
#[derive(Component)]
pub struct RelationsBody;

/// Root nodes whose visibility is driven by the game phase and overlay flags.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum HudPart {
    EndTurn,
    CombatBar,
    CombatHelp,
    Burger,
    Relations,
}

/// Text value of one of the top-bar resource pills.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Gold,
    Food,
    Science,
}

/// Small `+N` income label shown next to a top-bar resource value.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct ResourceIncomeKind(pub ResourceKind);

/// Fill node of the mana bar.
#[derive(Component)]
pub struct ManaFill;

/// `current / maximum` text of the mana bar.
#[derive(Component)]
pub struct ManaValueText;

/// Chip that says whose turn it is right now.
#[derive(Component)]
pub struct TurnStatusText;

/// Skill selector in the combat bar. `0` is the basic attack.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct SkillButton(pub u8);

/// Own-unit navigation button in the combat bar.
#[derive(Component, Clone, Copy)]
pub enum ArmyNav {
    First,
    Previous,
    Next,
    Last,
}

/// Status line in the combat command bar.
#[derive(Component)]
pub struct CombatStatusText;

/// Overlay flags controlled by the HUD buttons.
#[derive(Resource, Default)]
pub struct HudUiState {
    pub burger_open: bool,
    pub relations_open: bool,
    pub combat_help_open: bool,
    /// Whether the left information sidebar is visible.
    pub sidebar_open: bool,
}

/// Local (not yet simulated) combat planning state.
#[derive(Resource, Default)]
pub struct CombatUiState {
    /// `0` — basic attack, `1..=3` — unit abilities.
    pub armed_skill: u8,
}

/// Section of the sidebar tree.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SidebarSection {
    Units,
    Buildings,
    Resources,
    Terrain,
}

/// Which unit group the `Units` section is currently showing.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum UnitTab {
    #[default]
    Combat,
    Builder,
}

/// Expand/collapse state of the sidebar tree.
#[derive(Resource, Default)]
pub struct SidebarView {
    pub units_open: bool,
    pub buildings_open: bool,
    pub resources_open: bool,
    pub terrain_open: bool,
    pub unit_tab: UnitTab,
    /// Selection the sections were last synced with, so that opening a section
    /// only happens when the selection actually changes.
    pub last_selection: Option<(IVec2, TileLayer)>,
}

impl SidebarView {
    pub fn toggle(&mut self, section: SidebarSection) {
        match section {
            SidebarSection::Units => self.units_open = !self.units_open,
            SidebarSection::Buildings => self.buildings_open = !self.buildings_open,
            SidebarSection::Resources => self.resources_open = !self.resources_open,
            SidebarSection::Terrain => self.terrain_open = !self.terrain_open,
        }
    }
}
