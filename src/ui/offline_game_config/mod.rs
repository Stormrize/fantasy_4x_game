mod build_game_config;
mod colors;
mod components;
mod dropdown;
mod popup;
mod race_panel;
mod scrollable;
mod sidebar;

use bevy::prelude::{
    AlignItems, App, ChildOf, Commands, DespawnOnExit, Node, OnEnter, Plugin, Res,
};
use bevy::ui::{BackgroundColor, FlexDirection, FlexWrap, Val};
use bevy::utils::default;
use bevy_persistent::Persistent;
pub use components::PendingGameSetup;

use crate::io::startup_loading::fonts_loader_plugin::Fonts;
use crate::io::startup_loading::localization_loader_plugin::Localization;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::colors::{COLOR_PANEL_BG, COLOR_SIDEBAR_BG};
use crate::ui::menus::AppState;

pub struct OfflineGameConfig;

impl Plugin for OfflineGameConfig {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::OfflineGameSetupMenu), spawn_setup_menu);
    }
}

fn spawn_setup_menu(
    mut commands: Commands,
    races_ui_registry: Res<RacesUiRegistry>,
    localization: Res<Localization>,
    fonts: Res<Fonts>,
    settings: Res<Persistent<SettingsConfig>>,
) {
    let language = settings.language;

    commands.insert_resource(PendingGameSetup::default());

    let panel = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            DespawnOnExit(AppState::OfflineGameSetupMenu),
        ))
        .id();

    let content = commands
        .spawn((
            Node {
                width: Val::Percent(60.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            BackgroundColor(COLOR_PANEL_BG),
            ChildOf(panel),
        ))
        .id();

    let sidebar = commands
        .spawn((
            Node {
                width: Val::Percent(40.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(COLOR_SIDEBAR_BG),
            ChildOf(panel),
        ))
        .id();

    race_panel::spawn_race_panel(&mut commands, content, panel, &races_ui_registry.0);
    sidebar::spawn_sidebar(&mut commands, sidebar, &localization, &fonts, language);
}
