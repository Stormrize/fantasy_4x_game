//! The relations overlay shell. Its body is filled by `rebuild_relations`.

use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;

use crate::app_state::AppState;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{HudPart, HudUiState, RelationsBody};
use crate::ui::in_game_ui::theme;

pub(crate) fn spawn_relations_panel(commands: &mut Commands, ui: &UiContext, language: Language) {
    let panel = spawn_panel_root(commands);

    spawn_header(commands, panel, ui, language);

    theme::spawn_wrapped_translation(
        commands,
        panel,
        ui,
        language,
        "relations_hint",
        9.0,
        theme::MUTED,
    );

    spawn_body(commands, panel);
}

fn spawn_panel_root(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            HudPart::Relations,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(78.0),
                right: Val::Px(10.0),
                bottom: Val::Px(96.0),
                width: Val::Px(380.0),
                display: Display::None,
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
            GlobalZIndex(20),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id()
}

/// Title on the left, close button on the right.
fn spawn_header(commands: &mut Commands, panel: Entity, ui: &UiContext, language: Language) {
    let header = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(panel),
        ))
        .id();

    theme::spawn_translation(
        commands,
        header,
        ui,
        language,
        "relations",
        17.0,
        theme::GOLD_TEXT,
    );
    commands.spawn((theme::spacer(), Pickable::IGNORE, ChildOf(header)));

    spawn_close_button(commands, header, ui, language);
}

fn spawn_close_button(commands: &mut Commands, header: Entity, ui: &UiContext, language: Language) {
    let close = commands
        .spawn((
            Node {
                width: Val::Px(24.0),
                height: Val::Px(22.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(theme::BUTTON_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(header),
        ))
        .id();

    theme::spawn_label(commands, close, ui, language, "x", 11.0, theme::TEXT);
    theme::add_hover(commands, close, theme::BUTTON_BG, theme::BUTTON_BG_HOVER);
    commands
        .entity(close)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.relations_open = false;
        });
}

/// Scrollable container for the player cards.
fn spawn_body(commands: &mut Commands, panel: Entity) {
    commands.spawn((
        RelationsBody,
        ScrollArea,
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(8.0),
            padding: UiRect::right(Val::Px(4.0)),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        ChildOf(panel),
    ));
}
