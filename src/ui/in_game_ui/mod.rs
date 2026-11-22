mod burger_menu;
mod combat_phase;
mod end_turn_claster;
mod hud;
mod mini_interactive_ui;
mod relations;
mod sidebar;
mod spawn;
mod state;
mod theme;
mod top_bar;

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, OnEnter, in_state};

use crate::app_state::AppState;
use crate::ui::in_game_ui::state::{CombatUiState, HudUiState, SidebarView};

pub(super) struct InGameMenu;

impl Plugin for InGameMenu {
    fn build(&self, app: &mut App) {
        app.init_resource::<HudUiState>()
            .init_resource::<CombatUiState>()
            .init_resource::<SidebarView>()
            .add_systems(OnEnter(AppState::InGameRenderer), spawn::spawn_in_game_hud)
            .add_systems(
                Update,
                (
                    top_bar::update_resource_texts,
                    top_bar::update_income_texts,
                    top_bar::update_mana_bar,
                    top_bar::update_turn_status,
                    hud::sync_hud_visibility,
                    hud::toggle_burger_on_escape,
                    sidebar::sync_sidebar_visibility,
                    sidebar::rebuild_sidebar,
                    relations::rebuild_relations,
                    combat_phase::update_combat_status,
                    combat_phase::update_skill_highlight,
                )
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}
