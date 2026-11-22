//! Gold / food / science pills: current totals and the `+N` income hint.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{ResourceIncomeKind, ResourceKind};
use crate::ui::in_game_ui::theme;
use crate::world::components::Id;
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::building::{BuildingLevel, Levels};

// Spawning

pub(super) fn spawn_resource_pills(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    let pills = [
        ("resource_gold", theme::RES_GOLD, ResourceKind::Gold),
        ("resource_food", theme::RES_FOOD, ResourceKind::Food),
        (
            "resource_science",
            theme::RES_SCIENCE,
            ResourceKind::Science,
        ),
    ];

    for (name_key, color, kind) in pills {
        spawn_resource_pill(commands, parent, ui, language, name_key, color, kind);
    }
}

fn spawn_resource_pill(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    name_key: &'static str,
    color: Color,
    kind: ResourceKind,
) {
    let pill = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(theme::SLOT_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_dot(commands, pill, color, 10.0);

    let text_column = commands.spawn((theme::column(0.0), ChildOf(pill))).id();

    theme::spawn_translation(
        commands,
        text_column,
        ui,
        language,
        name_key,
        8.0,
        theme::MUTED,
    );

    let value_row = commands.spawn((theme::row(4.0), ChildOf(text_column))).id();

    // Total, updated by `update_resource_texts`.
    let value = theme::spawn_label(commands, value_row, ui, language, "0", 15.0, theme::TEXT);
    commands.entity(value).insert(kind);

    // Income hint, updated by `update_income_texts`.
    let income = theme::spawn_label(commands, value_row, ui, language, "", 9.0, color);
    commands.entity(income).insert(ResourceIncomeKind(kind));
}

// Live updates

pub(crate) fn update_resource_texts(
    game_logic_config: Res<GameLogicConfig>,
    players: Res<Players>,
    mut values: Query<(&mut Text, &ResourceKind)>,
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

    for (mut text, kind) in &mut values {
        let value = match kind {
            ResourceKind::Gold => player.gold.to_string(),
            ResourceKind::Food => player.food.to_string(),
            ResourceKind::Science => player.science.to_string(),
        };
        if text.0 != value {
            text.0 = value;
        }
    }
}

/// Recomputes the `+N` income hints next to the top-bar resource pills from the
/// local player's buildings. This used to live in the sidebar, but income is an
/// empire-wide value and belongs next to the resource totals.
pub(crate) fn update_income_texts(
    game_logic_config: Res<GameLogicConfig>,
    buildings_map: Res<BuildingMap>,
    buildings: Query<(&Id, &BuildingLevel, &Levels)>,
    mut values: Query<(&mut Text, &ResourceIncomeKind)>,
) {
    let income = empire_income(game_logic_config.player.id, &buildings_map, &buildings);

    for (mut text, kind) in &mut values {
        let value = match kind.0 {
            ResourceKind::Gold => income.gold,
            ResourceKind::Food => income.food,
            ResourceKind::Science => income.science,
        };
        let value = format!("+{value}");
        if text.0 != value {
            text.0 = value;
        }
    }
}

/// Per-turn income of the local player. Gold is already net of maintenance.
struct Income {
    gold: i32,
    food: i32,
    science: i32,
}

fn empire_income(
    local_id: Id,
    buildings_map: &BuildingMap,
    buildings: &Query<(&Id, &BuildingLevel, &Levels)>,
) -> Income {
    let mut gold: i32 = 0;
    let mut maintenance: i32 = 0;
    let mut food: i32 = 0;
    let mut science: i32 = 0;

    for slot in buildings_map.map.iter().flatten() {
        let Ok((id, level, levels)) = buildings.get(*slot) else {
            continue;
        };
        if *id != local_id {
            continue;
        }
        let stats = levels.stats(*level);
        gold += stats.gold() as i32;
        maintenance += stats.maintenance() as i32;
        food += stats.food() as i32;
        science += stats.science() as i32;
    }

    Income {
        gold: (gold - maintenance).max(0),
        food,
        science,
    }
}
