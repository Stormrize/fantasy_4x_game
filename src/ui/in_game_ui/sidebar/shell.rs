//! The outer sidebar panel: container, player card, scroll body and hint.

use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;

use crate::app_state::AppState;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{HudSidebar, HudUiState, SidebarBody};
use crate::ui::in_game_ui::theme;
use crate::world::game_configs::{GameLogicConfig, Players};

use super::widgets::{XButton, spawn_x_button};

const RACE_ICON_SIZE: f32 = 38.0;

pub(crate) fn spawn_sidebar(
    commands: &mut Commands,
    ui: &UiContext,
    language: Language,
    players: &Players,
    config: &GameLogicConfig,
    races: &RacesUiRegistry,
) {
    let panel = spawn_panel(commands);

    spawn_player_card(commands, panel, ui, language, players, config, races);
    spawn_body(commands, panel);
    theme::spawn_translation(
        commands,
        panel,
        ui,
        language,
        "sidebar_hint",
        9.0,
        theme::MUTED,
    );
}

/// Hides or shows the whole sidebar panel according to [`HudUiState::sidebar_open`].
pub(crate) fn sync_sidebar_visibility(
    hud: Res<HudUiState>,
    mut sidebars: Query<&mut Node, With<HudSidebar>>,
) {
    let wanted = if hud.sidebar_open {
        Display::Flex
    } else {
        Display::None
    };

    for mut node in &mut sidebars {
        if node.display != wanted {
            node.display = wanted;
        }
    }
}

fn spawn_panel(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            HudSidebar,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(78.0),
                left: Val::Px(10.0),
                bottom: Val::Px(16.0),
                width: Val::Px(318.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(8.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            GlobalZIndex(10),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id()
}

/// Scrollable container; its children are rebuilt by `rebuild_sidebar`.
fn spawn_body(commands: &mut Commands, panel: Entity) {
    commands.spawn((
        SidebarBody,
        ScrollArea,
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(10.0),
            padding: UiRect::right(Val::Px(4.0)),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        ChildOf(panel),
    ));
}

fn spawn_player_card(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    players: &Players,
    config: &GameLogicConfig,
    races: &RacesUiRegistry,
) {
    let Some(player) = players
        .0
        .iter()
        .find(|player| player.id == config.player.id)
    else {
        return;
    };

    let card = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(theme::SLOT_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::IGNORE,
            ChildOf(parent),
        ))
        .id();

    // Race icon, or a colored dot when the race has no icon.
    if let Some(race_data) = races.0.get(&player.player_race) {
        theme::spawn_race_icon(commands, card, race_data.ui_icon.clone(), RACE_ICON_SIZE);
    } else {
        theme::spawn_dot(
            commands,
            card,
            theme::race_color(&player.player_race.0),
            RACE_ICON_SIZE,
        );
    }

    // Race name and player number.
    let text = commands.spawn((theme::column(1.0), ChildOf(card))).id();
    theme::spawn_label(
        commands,
        text,
        ui,
        language,
        theme::race_display_name(&player.player_race.0),
        16.0,
        theme::GOLD_TEXT,
    );
    theme::spawn_label(
        commands,
        text,
        ui,
        language,
        format!("{} {}", ui.get(language, "player"), player.id.0),
        9.0,
        theme::MUTED,
    );

    commands.spawn((theme::spacer(), Pickable::IGNORE, ChildOf(card)));

    // Close button.
    let close = spawn_x_button(commands, card, ui, language, XButton::close());
    commands
        .entity(close)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.sidebar_open = false;
        });
}
