//! Entry point: spawns every HUD panel once when the game screen is entered.

use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::UiContext;
use crate::ui::in_game_ui::{burger_menu, combat_phase, end_turn_claster, relations, sidebar};
use crate::world::game_configs::{GameLogicConfig, Players};

use super::top_bar::spawn_top_bar;

pub(crate) fn spawn_in_game_hud(
    mut commands: Commands,
    ui: UiContext,
    settings: Res<Persistent<SettingsConfig>>,
    players: Res<Players>,
    game_logic_config: Res<GameLogicConfig>,
    races: Res<RacesUiRegistry>,
) {
    let language = settings.language;

    spawn_top_bar(&mut commands, &ui, language);
    sidebar::spawn_sidebar(
        &mut commands,
        &ui,
        language,
        &players,
        &game_logic_config,
        &races,
    );

    end_turn_claster::spawn_end_turn_cluster(&mut commands, &ui, language, &game_logic_config);
    burger_menu::spawn_burger_overlay(&mut commands, &ui, language);
    relations::spawn_relations_panel(&mut commands, &ui, language);
    combat_phase::spawn_combat_bar(&mut commands, &ui, language);
}
