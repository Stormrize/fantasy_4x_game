//! The top bar: empire resources on the left, mana in the middle, turn
//! indicator and menu buttons on the right.
//!
//! This file only decides the layout; every widget lives in its own file.

mod buttons;
mod mana_bar;
mod resources;
mod turn_status;

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;

use buttons::{spawn_burger_button, spawn_relations_button, spawn_sidebar_toggle};
use mana_bar::spawn_mana_bar;
use resources::spawn_resource_pills;
use turn_status::spawn_turn_chip;

pub(crate) use mana_bar::update_mana_bar;
pub(crate) use resources::{update_income_texts, update_resource_texts};
pub(crate) use turn_status::update_turn_status;

pub(super) fn spawn_top_bar(commands: &mut Commands, ui: &UiContext, language: Language) {
    let bar = spawn_bar_root(commands);

    // Left group: empire resources.
    let left = spawn_group(commands, bar, JustifyContent::default());
    spawn_resource_pills(commands, left, ui, language);

    spawn_mana_bar(commands, bar, ui, language);

    // Right group: turn indicator, relations, sidebar toggle and the pause menu.
    let right = spawn_group(commands, bar, JustifyContent::FlexEnd);
    spawn_turn_chip(commands, right, ui, language);
    spawn_relations_button(commands, right, ui, language);
    spawn_sidebar_toggle(commands, right, ui, language);
    spawn_burger_button(commands, right);
}

fn spawn_bar_root(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                right: Val::Px(10.0),
                height: Val::Px(58.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(14.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
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

/// A growing row that holds one side of the bar.
fn spawn_group(commands: &mut Commands, bar: Entity, justify: JustifyContent) -> Entity {
    commands
        .spawn((
            Node {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                justify_content: justify,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            },
            ChildOf(bar),
        ))
        .id()
}
