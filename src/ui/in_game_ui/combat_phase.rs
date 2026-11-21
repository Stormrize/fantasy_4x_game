//! Combat-phase HUD: the bottom command bar and the "explaining" help panel.
//!
//! The bar exposes the Frozen Synapse-inspired planning loop:
//! select one of your units, arm a skill, click a visible enemy in the sidebar,
//! then simulate the round. Orders themselves are stored in `CombatOrders`.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::app_state::AppState;
use crate::game::events::action_messages::NewTile;
use crate::game::game_logic_plugin::combat::integration::{
    CombatCycle, CombatPhase, ResolveCombat,
};
use crate::game::game_logic_plugin::components::{
    AlreadySelectedTile, JustSelectedTile, TileContent,
};
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{
    ArmyNav, CombatStatusText, CombatUiState, HudPart, HudUiState, SkillButton,
};
use crate::ui::in_game_ui::theme;

use crate::world::components::Id;
use crate::world::game_configs::GameLogicConfig;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::unit_map::UnitsMap;
use bevy_persistent::Persistent;

pub(crate) fn spawn_combat_bar(commands: &mut Commands, ui: &UiContext, language: Language) {
    let bar = commands
        .spawn((
            HudPart::CombatBar,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(340.0),
                right: Val::Px(150.0),
                bottom: Val::Px(18.0),
                display: Display::None,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(7.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            GlobalZIndex(12),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id();

    // Row 1: phase title, live status and the timeline-style navigation.
    let header = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(bar),
        ))
        .id();

    theme::spawn_translation(
        commands,
        header,
        ui,
        language,
        "combat_phase",
        17.0,
        theme::GOLD_TEXT,
    );

    commands.spawn((
        theme::label(ui, language, "", 11.0, theme::MUTED),
        CombatStatusText,
        ChildOf(header),
    ));

    commands.spawn((theme::spacer(), Pickable::IGNORE, ChildOf(header)));

    spawn_square_button(commands, header, ui, language, "|<", ArmyNav::First);
    spawn_square_button(commands, header, ui, language, "<", ArmyNav::Previous);
    spawn_square_button(commands, header, ui, language, ">", ArmyNav::Next);
    spawn_square_button(commands, header, ui, language, ">|", ArmyNav::Last);

    let simulate_short =
        spawn_square_button_raw(commands, header, ui, language, "OK", theme::PRIMARY_BG);
    commands.entity(simulate_short).observe(
        |mut click: On<PointerClick>, mut resolve: MessageWriter<ResolveCombat>| {
            click.propagate(false);
            resolve.write(ResolveCombat);
        },
    );

    // Row 2: skill selection and the main actions.
    let controls = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6.0),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(bar),
        ))
        .id();

    theme::spawn_translation(
        commands,
        controls,
        ui,
        language,
        "skill",
        10.0,
        theme::MUTED,
    );

    for (skill, label) in [(0u8, "ATK"), (1, "1"), (2, "2"), (3, "3")] {
        spawn_skill_button(commands, controls, ui, language, skill, label);
    }

    commands.spawn((theme::spacer(), Pickable::IGNORE, ChildOf(controls)));

    let explain = theme::spawn_button(
        commands,
        controls,
        ui,
        language,
        "explain",
        theme::BUTTON_BG,
        theme::BUTTON_BG_HOVER,
    );
    commands
        .entity(explain)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.combat_help_open = !hud.combat_help_open;
        });

    let simulate = theme::spawn_button(
        commands,
        controls,
        ui,
        language,
        "simulate_step",
        theme::PRIMARY_BG,
        theme::PRIMARY_BG_HOVER,
    );
    commands.entity(simulate).observe(
        |mut click: On<PointerClick>, mut resolve: MessageWriter<ResolveCombat>| {
            click.propagate(false);
            resolve.write(ResolveCombat);
        },
    );

    // Row 3: short instructions.
    theme::spawn_wrapped_translation(
        commands,
        bar,
        ui,
        language,
        "combat_instructions",
        9.0,
        theme::MUTED,
    );

    spawn_help_panel(commands, ui, language);
}

fn spawn_help_panel(commands: &mut Commands, ui: &UiContext, language: Language) {
    let panel = commands
        .spawn((
            HudPart::CombatHelp,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(340.0),
                right: Val::Px(150.0),
                bottom: Val::Px(160.0),
                display: Display::None,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            GlobalZIndex(14),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id();

    theme::spawn_section_header(commands, panel, ui, language, "how_combat_works");

    for key in [
        "combat_help_1",
        "combat_help_2",
        "combat_help_3",
        "combat_help_4",
        "combat_help_5",
        "combat_help_6",
    ] {
        theme::spawn_wrapped_translation(commands, panel, ui, language, key, 10.0, theme::TEXT);
    }
}

fn spawn_square_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    label: &str,
    nav: ArmyNav,
) -> Entity {
    let button = spawn_square_button_raw(commands, parent, ui, language, label, theme::BUTTON_BG);

    commands.entity(button).observe(
        move |mut click: On<PointerClick>,
              selected: Res<AlreadySelectedTile>,
              units_map: Res<UnitsMap>,
              buildings_map: Res<BuildingMap>,
              resources_map: Res<ResourceMap>,
              fog: Res<FogMap>,
              ids: Query<&Id>,
              combat_levels: Query<&crate::world::game_resources::data::unit::UnitLevel>,
              config: Res<GameLogicConfig>,
              mut new_tile: MessageWriter<NewTile>| {
            click.propagate(false);

            let own = collect_own_positions(&units_map, &ids, &combat_levels, config.player.id);
            if own.is_empty() {
                return;
            }

            let current = selected
                .pos
                .and_then(|pos| own.iter().position(|candidate| *candidate == pos));

            let target = match nav {
                ArmyNav::First => own.first().copied(),
                ArmyNav::Last => own.last().copied(),
                ArmyNav::Next => match current {
                    Some(index) => own.get((index + 1) % own.len()).copied(),
                    None => own.first().copied(),
                },
                ArmyNav::Previous => match current {
                    Some(index) => own.get((index + own.len() - 1) % own.len()).copied(),
                    None => own.last().copied(),
                },
            };

            let Some(target) = target else {
                return;
            };
            let content = tile_content_at(target, &units_map, &buildings_map, &resources_map, &fog);
            new_tile.write(NewTile(JustSelectedTile {
                pos: target,
                tile_content: content,
            }));
        },
    );

    button
}

fn spawn_square_button_raw(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    label: &str,
    bg: Color,
) -> Entity {
    let button = commands
        .spawn((
            Node {
                width: Val::Px(32.0),
                height: Val::Px(26.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(bg),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_label(commands, button, ui, language, label, 12.0, theme::TEXT);

    let hover = if bg == theme::PRIMARY_BG {
        theme::PRIMARY_BG_HOVER
    } else {
        theme::BUTTON_BG_HOVER
    };
    theme::add_hover(commands, button, bg, hover);

    button
}

fn spawn_skill_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    skill: u8,
    label: &str,
) {
    let button = commands
        .spawn((
            Node {
                width: Val::Px(34.0),
                height: Val::Px(26.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(theme::BUTTON_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            SkillButton(skill),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_label(commands, button, ui, language, label, 12.0, theme::TEXT);

    commands.entity(button).observe(
        move |mut click: On<PointerClick>, mut combat_ui: ResMut<CombatUiState>| {
            click.propagate(false);
            combat_ui.armed_skill = skill;
        },
    );
}

// Live updates

pub(crate) fn update_combat_status(
    ui: UiContext,
    settings: Res<Persistent<SettingsConfig>>,
    cycle: Res<CombatCycle>,
    mut status: Query<(&mut Text, &mut TextColor), With<CombatStatusText>>,
) {
    let language = settings.language;
    let (message, color) = match &cycle.phase {
        CombatPhase::PlayerPlanning => (
            theme::resolve(&ui, language, "combat_planning"),
            theme::GOLD_TEXT,
        ),
        CombatPhase::Planning | CombatPhase::Resolving(_) => (
            theme::resolve(&ui, language, "combat_resolving"),
            theme::ENEMY,
        ),
        CombatPhase::Idle => (theme::resolve(&ui, language, "combat_idle"), theme::MUTED),
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

/// Keeps the armed skill visually highlighted.
pub(crate) fn update_skill_highlight(
    combat_ui: Res<CombatUiState>,
    mut buttons: Query<(&SkillButton, &mut BackgroundColor, &mut BorderColor)>,
) {
    for (skill, mut background, mut border) in &mut buttons {
        let armed = skill.0 == combat_ui.armed_skill;
        let wanted = if armed {
            theme::HIGHLIGHT_BG
        } else {
            theme::BUTTON_BG
        };
        if background.0 != wanted {
            background.0 = wanted;
        }
        let wanted_border = if armed {
            theme::BORDER
        } else {
            theme::BORDER_SOFT
        };
        if border.top != wanted_border {
            *border = BorderColor::all(wanted_border);
        }
    }
}

// Helpers

fn collect_own_positions(
    units_map: &UnitsMap,
    ids: &Query<&Id>,
    combat_units: &Query<&crate::world::game_resources::data::unit::UnitLevel>,
    owner: Id,
) -> Vec<IVec2> {
    let size = units_map.size;
    let mut positions = Vec::new();

    for (index, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok(id) = ids.get(entity) else { continue };
        // Builders cannot receive combat orders, so navigation skips them too.
        if *id != owner || combat_units.get(entity).is_err() {
            continue;
        }
        positions.push(IVec2::new((index / size) as i32, (index % size) as i32));
    }

    positions
}

fn tile_content_at(
    pos: IVec2,
    units_map: &UnitsMap,
    buildings_map: &BuildingMap,
    resources_map: &ResourceMap,
    fog: &FogMap,
) -> TileContent {
    if pos.x < 0
        || pos.y < 0
        || (pos.x as usize) >= units_map.size
        || (pos.y as usize) >= units_map.size
    {
        return TileContent::default();
    }
    let (x, y) = (pos.x as usize, pos.y as usize);
    TileContent::from_world(
        true,
        units_map.is_some(x, y),
        buildings_map.is_some(x, y),
        resources_map.is_some(x, y),
        !fog.is_general_visible_player(x, y),
    )
}
