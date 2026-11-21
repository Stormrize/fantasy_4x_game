//! Spawns the in-game HUD and keeps the always-visible parts (resources, mana,
//! turn indicator, phase visibility) in sync with the game state.
//!
//! The HUD is intentionally split into independent absolute panels instead of
//! one fullscreen root: a fullscreen container would swallow world clicks even
//! where nothing is drawn.

use crate::game::events::action_messages::EndTurn;
use crate::game::game_logic_plugin::combat::integration::{CombatCycle, CombatPhase};
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::UiContext;
use crate::ui::in_game_ui::burger_menu;
use crate::ui::in_game_ui::combat_phase;
use crate::ui::in_game_ui::relations;
use crate::ui::in_game_ui::sidebar;
use crate::ui::in_game_ui::state::{
    HudPart, HudUiState, ManaFill, ManaValueText, ResourceIncomeKind, ResourceKind, TurnStatusText,
};
use crate::ui::in_game_ui::theme;
use crate::ui::menus::AppState;
use crate::world::components::Id;
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::building::{BuildingLevel, Levels};
use crate::world::turn_manager::TurnManager;
use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy_persistent::Persistent;

// Spawning

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
    spawn_end_turn_cluster(&mut commands, &ui, language, &game_logic_config);
    burger_menu::spawn_burger_overlay(&mut commands, &ui, language);
    relations::spawn_relations_panel(&mut commands, &ui, language);
    combat_phase::spawn_combat_bar(&mut commands, &ui, language);
}

fn spawn_top_bar(commands: &mut Commands, ui: &UiContext, language: Language) {
    let bar = commands
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
        .id();

    // Left group: empire resources.
    let resources = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            },
            ChildOf(bar),
        ))
        .id();

    spawn_resource_pill(
        commands,
        resources,
        ui,
        language,
        "resource_gold",
        theme::RES_GOLD,
        ResourceKind::Gold,
    );
    spawn_resource_pill(
        commands,
        resources,
        ui,
        language,
        "resource_food",
        theme::RES_FOOD,
        ResourceKind::Food,
    );
    spawn_resource_pill(
        commands,
        resources,
        ui,
        language,
        "resource_science",
        theme::RES_SCIENCE,
        ResourceKind::Science,
    );

    spawn_mana_bar(commands, bar, ui, language);

    // Right group: turn indicator, relations and the pause menu.
    let right = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            },
            ChildOf(bar),
        ))
        .id();

    let turn_chip = commands
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme::SLOT_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::IGNORE,
            ChildOf(right),
        ))
        .id();

    commands.spawn((
        theme::label(ui, language, "", 10.0, theme::MUTED),
        TurnStatusText,
        ChildOf(turn_chip),
    ));

    spawn_relations_button(commands, right, ui, language);
    spawn_sidebar_toggle(commands, right, ui, language);
    spawn_burger_button(commands, right);
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

    let value = theme::spawn_label(commands, value_row, ui, language, "0", 15.0, theme::TEXT);
    commands.entity(value).insert(kind);

    let income = theme::spawn_label(commands, value_row, ui, language, "", 9.0, color);
    commands.entity(income).insert(ResourceIncomeKind(kind));
}

fn spawn_mana_bar(commands: &mut Commands, parent: Entity, ui: &UiContext, language: Language) {
    let mana = commands
        .spawn((
            Node {
                width: Val::Px(340.0),
                min_width: Val::Px(340.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(3.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(5.0)),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

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

fn spawn_relations_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    let button = commands
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(7.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme::BUTTON_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_translation(
        commands,
        button,
        ui,
        language,
        "relations",
        11.0,
        theme::TEXT,
    );
    theme::add_hover(commands, button, theme::BUTTON_BG, theme::BUTTON_BG_HOVER);

    commands
        .entity(button)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.relations_open = !hud.relations_open;
            hud.burger_open = false;
        });
}

/// Toggles the left information sidebar so the map can be inspected without it.
fn spawn_sidebar_toggle(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    let button = commands
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(7.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme::BUTTON_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_translation(
        commands,
        button,
        ui,
        language,
        "sidebar_toggle",
        11.0,
        theme::TEXT,
    );
    theme::add_hover(commands, button, theme::BUTTON_BG, theme::BUTTON_BG_HOVER);

    commands
        .entity(button)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.sidebar_open = !hud.sidebar_open;
        });
}

fn spawn_burger_button(commands: &mut Commands, parent: Entity) {
    let button = commands
        .spawn((
            Node {
                width: Val::Px(40.0),
                height: Val::Px(34.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(theme::BUTTON_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    // Three drawn bars instead of a glyph so the menu works with any font.
    for _ in 0..3 {
        commands.spawn((
            Node {
                width: Val::Px(18.0),
                height: Val::Px(2.0),
                border_radius: BorderRadius::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(theme::TEXT),
            Pickable::IGNORE,
            ChildOf(button),
        ));
    }

    theme::add_hover(commands, button, theme::BUTTON_BG, theme::BUTTON_BG_HOVER);

    commands
        .entity(button)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.burger_open = !hud.burger_open;
            hud.relations_open = false;
        });
}

fn spawn_end_turn_cluster(
    commands: &mut Commands,
    ui: &UiContext,
    language: Language,
    game_logic_config: &GameLogicConfig,
) {
    let cluster = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(24.0),
                bottom: Val::Px(20.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                column_gap: Val::Px(10.0),
                ..default()
            },
            Pickable::IGNORE,
            HudPart::EndTurn,
            GlobalZIndex(10),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id();

    // Decorative placeholders for the future tech / units / buildings wheels.
    // Intentionally non-interactive for now.
    let decorations = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                padding: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(cluster),
        ))
        .id();

    theme::spawn_dot(commands, decorations, Color::srgb(0.16, 0.29, 0.58), 24.0);
    theme::spawn_dot(commands, decorations, Color::srgb(0.36, 0.20, 0.38), 24.0);

    let label_column = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(2.0),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(cluster),
        ))
        .id();

    let next_turn = commands
        .spawn((
            Node {
                width: Val::Px(84.0),
                height: Val::Px(84.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(theme::PRIMARY_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            ChildOf(label_column),
        ))
        .id();

    theme::spawn_label(commands, next_turn, ui, language, ">", 26.0, theme::TEXT);
    theme::spawn_translation(
        commands,
        next_turn,
        ui,
        language,
        "end_turn",
        8.0,
        theme::MUTED,
    );

    theme::add_hover(
        commands,
        next_turn,
        theme::PRIMARY_BG,
        theme::PRIMARY_BG_HOVER,
    );

    let local_player = game_logic_config.player;
    commands.entity(next_turn).observe(
        move |mut click: On<PointerClick>,
              mut end_turn: MessageWriter<EndTurn>,
              cycle: Res<CombatCycle>| {
            click.propagate(false);
            if matches!(cycle.phase, CombatPhase::Idle) {
                end_turn.write(EndTurn(local_player));
            }
        },
    );
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
    let local_id = game_logic_config.player.id;

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

    let net_gold = (gold - maintenance).max(0);

    for (mut text, kind) in &mut values {
        let value = match kind.0 {
            ResourceKind::Gold => net_gold,
            ResourceKind::Food => food,
            ResourceKind::Science => science,
        };
        let value = format!("+{value}");
        if text.0 != value {
            text.0 = value;
        }
    }
}

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

pub(crate) fn update_turn_status(
    ui: UiContext,
    settings: Res<Persistent<SettingsConfig>>,
    game_logic_config: Res<GameLogicConfig>,
    players: Res<Players>,
    turn_manager: Res<TurnManager>,
    cycle: Res<CombatCycle>,
    mut status: Query<(&mut Text, &mut TextColor), With<TurnStatusText>>,
) {
    let language = settings.language;
    let local_id = game_logic_config.player.id;
    let current = turn_manager.current_player;

    let (message, color) = if !matches!(cycle.phase, CombatPhase::Idle) {
        (
            theme::resolve(&ui, language, "turn_combat").to_uppercase(),
            theme::ENEMY,
        )
    } else if current.id == local_id {
        (
            theme::resolve(&ui, language, "turn_your").to_uppercase(),
            theme::GOLD_TEXT,
        )
    } else {
        let race = players
            .0
            .iter()
            .find(|player| player.id == current.id)
            .map(|player| theme::race_display_name(&player.player_race.0))
            .unwrap_or_else(|| theme::resolve(&ui, language, "enemy"));
        (
            format!(
                "{}: {}",
                theme::resolve(&ui, language, "turn").to_uppercase(),
                race.to_uppercase()
            ),
            theme::MUTED,
        )
    };

    for (mut text, mut text_color) in &mut status {
        if text.0 != message {
            text.0 = message.clone();
        }
        if text_color.0 != color {
            text_color.0 = color;
        }
    }
}

/// Shows/hides phase-specific HUD pieces and the overlays.
pub(crate) fn sync_hud_visibility(
    cycle: Res<CombatCycle>,
    hud: Res<HudUiState>,
    mut parts: Query<(&HudPart, &mut Node)>,
) {
    let in_combat = !matches!(cycle.phase, CombatPhase::Idle);

    for (part, mut node) in &mut parts {
        let visible = match part {
            HudPart::EndTurn => !in_combat,
            HudPart::CombatBar => in_combat,
            HudPart::CombatHelp => in_combat && hud.combat_help_open,
            HudPart::Burger => hud.burger_open,
            HudPart::Relations => hud.relations_open,
        };
        set_display(&mut node, visible);
    }
}

fn set_display(node: &mut Node, visible: bool) {
    let wanted = if visible {
        Display::Flex
    } else {
        Display::None
    };
    if node.display != wanted {
        node.display = wanted;
    }
}

/// `Esc` closes the topmost overlay or opens the pause menu.
pub(crate) fn toggle_burger_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hud: ResMut<HudUiState>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    if hud.relations_open {
        hud.relations_open = false;
    } else if hud.combat_help_open {
        hud.combat_help_open = false;
    } else {
        hud.burger_open = !hud.burger_open;
    }
}
