//! Mana bar in the middle of the top bar.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{ManaFill, ManaValueText};
use crate::ui::in_game_ui::theme;
use crate::world::game_configs::{GameLogicConfig, Players};

const MANA_BAR_WIDTH: f32 = 340.0;

// Spawning

pub(super) fn spawn_mana_bar(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    let mana = commands
        .spawn((
            Node {
                width: Val::Px(MANA_BAR_WIDTH),
                min_width: Val::Px(MANA_BAR_WIDTH),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(3.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(5.0)),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

    spawn_header(commands, mana, ui, language);
    spawn_track(commands, mana);
}

/// `MANA ........ 12 / 30`
fn spawn_header(commands: &mut Commands, mana: Entity, ui: &UiContext, language: Language) {
    let header = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ChildOf(mana),
        ))
        .id();

    theme::spawn_translation(commands, header, ui, language, "mana", 9.0, theme::MUTED);
    commands.spawn((
        theme::label(ui, language, "0 / 0", 11.0, theme::MANA_TEXT),
        ManaValueText,
        ChildOf(header),
    ));
}

fn spawn_track(commands: &mut Commands, mana: Entity) {
    let track = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(10.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(theme::MANA_TRACK),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::IGNORE,
            ChildOf(mana),
        ))
        .id();

    commands.spawn((
        Node {
            width: Val::Percent(0.0),
            height: Val::Percent(100.0),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(theme::MANA),
        ManaFill,
        Pickable::IGNORE,
        ChildOf(track),
    ));
}

// Live update

pub(crate) fn update_mana_bar(
    game_logic_config: Res<GameLogicConfig>,
    players: Res<Players>,
    mut fills: Query<&mut Node, With<ManaFill>>,
    mut values: Query<&mut Text, With<ManaValueText>>,
) {
    if !players.is_changed() {
        return;
    }

    let Some(player) = players
        .0
        .iter()
        .find(|player| player.id == game_logic_config.player.id)
    else {
        return;
    };

    let maximum = player.maximum_mana.max(1);
    let ratio = (player.current_mana as f32 / maximum as f32).clamp(0.0, 1.0);

    for mut node in &mut fills {
        let wanted = Val::Percent(ratio * 100.0);
        if node.width != wanted {
            node.width = wanted;
        }
    }
    for mut text in &mut values {
        let value = format!("{} / {}", player.current_mana, player.maximum_mana);
        if text.0 != value {
            text.0 = value;
        }
    }
}
