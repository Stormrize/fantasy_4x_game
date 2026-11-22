//! One player card: race, stance, stockpile, army / city count and mana.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;
use crate::world::game_configs::PlayerData;

const RACE_ICON_SIZE: f32 = 34.0;

/// What the card shows for one player.
pub(super) struct PlayerCard<'a> {
    pub player: &'a PlayerData,
    pub is_local: bool,
    pub units: usize,
    pub buildings: usize,
}

pub(super) fn spawn_player_card(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    races: &RacesUiRegistry,
    data: &PlayerCard,
) {
    let card = spawn_card_root(commands, body, data.is_local);

    spawn_identity_row(commands, card, ui, language, races, data);
    spawn_stats_row(commands, card, ui, language, data);
    spawn_mana_row(
        commands,
        card,
        ui,
        language,
        data.player.current_mana,
        data.player.maximum_mana,
    );
}

/// The local player's card is highlighted.
fn spawn_card_root(commands: &mut Commands, body: Entity, is_local: bool) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(if is_local {
                theme::HIGHLIGHT_BG
            } else {
                theme::SLOT_BG
            }),
            BorderColor::all(if is_local {
                theme::BORDER
            } else {
                theme::BORDER_SOFT
            }),
            Pickable::IGNORE,
            ChildOf(body),
        ))
        .id()
}

/// `[icon]  Race name / You | Player N      (stance)`
fn spawn_identity_row(
    commands: &mut Commands,
    card: Entity,
    ui: &UiContext,
    language: Language,
    races: &RacesUiRegistry,
    data: &PlayerCard,
) {
    let player = data.player;

    let mut top_node = theme::row(8.0);
    top_node.width = Val::Percent(100.0);
    let top = commands
        .spawn((top_node, Pickable::IGNORE, ChildOf(card)))
        .id();

    // Race icon, or a colored dot when the race has no icon.
    if let Some(race_data) = races.0.get(&player.player_race) {
        theme::spawn_race_icon(commands, top, race_data.ui_icon.clone(), RACE_ICON_SIZE);
    } else {
        theme::spawn_dot(
            commands,
            top,
            theme::race_color(&player.player_race.0),
            RACE_ICON_SIZE,
        );
    }

    let names = commands.spawn((theme::grow_column(1.0), ChildOf(top))).id();
    theme::spawn_label(
        commands,
        names,
        ui,
        language,
        theme::race_display_name(&player.player_race.0),
        14.0,
        theme::GOLD_TEXT,
    );
    if data.is_local {
        theme::spawn_translation(commands, names, ui, language, "you", 9.0, theme::MUTED);
    } else {
        theme::spawn_label(
            commands,
            names,
            ui,
            language,
            format!("{} {}", ui.get(language, "player"), player.id.0),
            9.0,
            theme::MUTED,
        );
    }

    spawn_stance_badge(commands, top, ui, language);
}

/// Diplomatic stance. Always "neutral" until diplomacy exists.
fn spawn_stance_badge(commands: &mut Commands, top: Entity, ui: &UiContext, language: Language) {
    let stance = commands
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(8.0), Val::Px(2.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG_SOFT),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::IGNORE,
            ChildOf(top),
        ))
        .id();
    theme::spawn_translation(commands, stance, ui, language, "neutral", 8.0, theme::MUTED);
}

/// `120g  45f  8s    Units 4 · Buildings 2`
fn spawn_stats_row(
    commands: &mut Commands,
    card: Entity,
    ui: &UiContext,
    language: Language,
    data: &PlayerCard,
) {
    let player = data.player;

    let mut node = theme::row(12.0);
    node.width = Val::Percent(100.0);
    node.margin = UiRect::top(Val::Px(2.0));
    let row = commands.spawn((node, Pickable::IGNORE, ChildOf(card))).id();

    theme::spawn_label(
        commands,
        row,
        ui,
        language,
        format!("{}g", player.gold),
        11.0,
        theme::RES_GOLD,
    );
    theme::spawn_label(
        commands,
        row,
        ui,
        language,
        format!("{}f", player.food),
        11.0,
        theme::RES_FOOD,
    );
    theme::spawn_label(
        commands,
        row,
        ui,
        language,
        format!("{}s", player.science),
        11.0,
        theme::RES_SCIENCE,
    );
    theme::spawn_label(
        commands,
        row,
        ui,
        language,
        format!(
            "{} {}  ·  {} {}",
            ui.get(language, "units"),
            data.units,
            ui.get(language, "buildings"),
            data.buildings,
        ),
        9.0,
        theme::MUTED,
    );
}

/// `MANA [=====     ] 12 / 30`
fn spawn_mana_row(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    current: u16,
    maximum: u16,
) {
    let mut row_node = theme::row(6.0);
    row_node.width = Val::Percent(100.0);
    let row = commands
        .spawn((row_node, Pickable::IGNORE, ChildOf(parent)))
        .id();

    theme::spawn_translation(commands, row, ui, language, "mana", 8.0, theme::MUTED);

    let track = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                height: Val::Px(6.0),
                border_radius: BorderRadius::MAX,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(theme::MANA_TRACK),
            Pickable::IGNORE,
            ChildOf(row),
        ))
        .id();

    let ratio = if maximum == 0 {
        0.0
    } else {
        (current as f32 / maximum as f32).clamp(0.0, 1.0)
    };
    commands.spawn((
        Node {
            width: Val::Percent(ratio * 100.0),
            height: Val::Percent(100.0),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BackgroundColor(theme::MANA),
        Pickable::IGNORE,
        ChildOf(track),
    ));

    theme::spawn_label(
        commands,
        row,
        ui,
        language,
        format!("{current} / {maximum}"),
        9.0,
        theme::MANA_TEXT,
    );
}
