mod burger_menu;
mod combat_phase;
mod mini_interactive_ui;
mod relations;
mod sidebar;
mod spawn_in_game_ui;
mod state;
mod theme;

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
            .add_systems(
                OnEnter(AppState::InGameRenderer),
                spawn_in_game_ui::spawn_in_game_hud,
            )
            .add_systems(
                Update,
                (
                    spawn_in_game_ui::update_resource_texts,
                    spawn_in_game_ui::update_income_texts,
                    spawn_in_game_ui::update_mana_bar,
                    spawn_in_game_ui::update_turn_status,
                    spawn_in_game_ui::sync_hud_visibility,
                    sidebar::sync_sidebar_visibility,
                    spawn_in_game_ui::toggle_burger_on_escape,
                    sidebar::rebuild_sidebar,
                    relations::rebuild_relations,
                )
                    .run_if(in_state(AppState::InGameRenderer)),
            )
            .add_systems(
                Update,
                (
                    combat_phase::update_combat_status,
                    combat_phase::update_skill_highlight,
                )
                    .run_if(in_state(AppState::InGameRenderer)),
            );
    }
}
