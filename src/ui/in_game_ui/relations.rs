//! Relations overlay: one card per player with its race, stockpile, mana and
//! a quick army / city count.

use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;
use bevy_persistent::Persistent;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{HudPart, HudUiState, RelationsBody};
use crate::ui::in_game_ui::theme;
use crate::ui::menus::AppState;
use crate::world::components::Id;
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::unit_map::UnitsMap;
use std::collections::HashMap;

pub(crate) fn spawn_relations_panel(commands: &mut Commands, ui: &UiContext, language: Language) {
    let panel = commands
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
        .id();

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

    theme::spawn_wrapped_translation(
        commands,
        panel,
        ui,
        language,
        "relations_hint",
        9.0,
        theme::MUTED,
    );

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

#[allow(clippy::too_many_arguments)]
pub(crate) fn rebuild_relations(
    mut commands: Commands,
    hud: Res<HudUiState>,
    ui: UiContext,
    settings: Res<Persistent<SettingsConfig>>,
    players: Res<Players>,
    config: Res<GameLogicConfig>,
    races: Res<RacesUiRegistry>,
    units_map: Res<UnitsMap>,
    buildings_map: Res<BuildingMap>,
    body_query: Query<(Entity, &ScrollPosition), With<RelationsBody>>,
    ids: Query<&Id>,
) {
    if !hud.relations_open {
        return;
    }

    if !players.is_changed()
        && !units_map.is_changed()
        && !buildings_map.is_changed()
        && !settings.is_changed()
        && !hud.is_changed()
    {
        return;
    }

    let Ok((body, scroll)) = body_query.single() else {
        return;
    };
    let scroll = scroll.0;
    commands.entity(body).despawn_children();
    commands.entity(body).insert(ScrollPosition(scroll));

    let language = settings.language;
    let unit_counts = count_by_owner(&units_map.map, &ids);
    let building_counts = count_by_owner(&buildings_map.map, &ids);

    for player in &players.0 {
        let is_local = player.id == config.player.id;
        let card = commands
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
            .id();

        let mut top_node = theme::row(8.0);
        top_node.width = Val::Percent(100.0);
        let top = commands
            .spawn((top_node, Pickable::IGNORE, ChildOf(card)))
            .id();

        if let Some(race_data) = races.0.get(&player.player_race) {
            theme::spawn_race_icon(&mut commands, top, race_data.ui_icon.clone(), 34.0);
        } else {
            theme::spawn_dot(
                &mut commands,
                top,
                theme::race_color(&player.player_race.0),
                34.0,
            );
        }

        let names = commands.spawn((theme::grow_column(1.0), ChildOf(top))).id();
        theme::spawn_label(
            &mut commands,
            names,
            &ui,
            language,
            theme::race_display_name(&player.player_race.0),
            14.0,
            theme::GOLD_TEXT,
        );
        if is_local {
            theme::spawn_translation(
                &mut commands,
                names,
                &ui,
                language,
                "you",
                9.0,
                theme::MUTED,
            );
        } else {
            theme::spawn_label(
                &mut commands,
                names,
                &ui,
                language,
                format!("{} {}", ui.get(language, "player"), player.id.0),
                9.0,
                theme::MUTED,
            );
        }

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
        theme::spawn_translation(
            &mut commands,
            stance,
            &ui,
            language,
            "neutral",
            8.0,
            theme::MUTED,
        );

        let mut resources_node = theme::row(12.0);
        resources_node.width = Val::Percent(100.0);
        resources_node.margin = UiRect::top(Val::Px(2.0));
        let resources = commands
            .spawn((resources_node, Pickable::IGNORE, ChildOf(card)))
            .id();

        theme::spawn_label(
            &mut commands,
            resources,
            &ui,
            language,
            format!("{}g", player.gold),
            11.0,
            theme::RES_GOLD,
        );
        theme::spawn_label(
            &mut commands,
            resources,
            &ui,
            language,
            format!("{}f", player.food),
            11.0,
            theme::RES_FOOD,
        );
        theme::spawn_label(
            &mut commands,
            resources,
            &ui,
            language,
            format!("{}s", player.science),
            11.0,
            theme::RES_SCIENCE,
        );
        theme::spawn_label(
            &mut commands,
            resources,
            &ui,
            language,
            format!(
                "{} {}  ·  {} {}",
                ui.get(language, "units"),
                unit_counts.get(&player.id).copied().unwrap_or(0),
                ui.get(language, "buildings"),
                building_counts.get(&player.id).copied().unwrap_or(0),
            ),
            9.0,
            theme::MUTED,
        );

        spawn_mana_row(
            &mut commands,
            card,
            &ui,
            language,
            player.current_mana,
            player.maximum_mana,
        );
    }
}

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

fn count_by_owner(slots: &[Option<Entity>], ids: &Query<&Id>) -> HashMap<Id, usize> {
    let mut counts = HashMap::new();
    for entity in slots.iter().flatten() {
        let Ok(id) = ids.get(*entity) else { continue };
        *counts.entry(*id).or_insert(0) += 1;
    }
    counts
}
