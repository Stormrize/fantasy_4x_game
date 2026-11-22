//! "Units" section: combat / builder tabs, unit rows, inline details.

use bevy::prelude::*;

use crate::game::game_logic_plugin::components::TileLayer;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{SidebarSection, SidebarView, UnitTab};
use crate::ui::in_game_ui::theme;
use crate::world::components::Id;
use crate::world::game_resources::data::unit::UnitType;

use super::super::data::UnitView;
use super::super::format::{
    hp_color, owner_label, unit_color, unit_summary, unit_title,
};
use super::super::interactions::attach_select;
use super::super::model::SidebarModel;
use super::super::widgets::{
    ListRow, spawn_collapsible_header, spawn_list_row, spawn_row_subtitle, spawn_row_title,
};

pub(crate) fn spawn_units_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    model: &SidebarModel,
) {
    let view = model.view;
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();

    let header_key = if model.in_combat { "your_units" } else { "units" };
    spawn_collapsible_header(
        commands,
        section,
        ui,
        language,
        header_key,
        view.units_open,
        SidebarSection::Units,
    );

    if !view.units_open {
        return;
    }

    spawn_unit_tabs(commands, section, ui, language, view.unit_tab);

    let want_helper = view.unit_tab == UnitTab::Builder;
    let listed: Vec<&UnitView> = model
        .own_units()
        .filter(|unit| (unit.kind == UnitType::Helper) == want_helper)
        .collect();

    if listed.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_units",
            10.0,
            theme::MUTED,
        );
        return;
    }

    for unit in listed {
        let is_selected = model.is_selected(unit.pos, TileLayer::Unit);
        let row = spawn_unit_row(commands, section, ui, language, unit, is_selected);
        attach_select(commands, row, unit.pos, TileLayer::Unit);

        if is_selected {
            spawn_unit_details(commands, section, ui, language, unit, model.local_id);
        }
    }
}

/// One unit list row: colored dot, title and `HP · MP · position` line.
/// Also used by the enemies section.
pub(super) fn spawn_unit_row(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    unit: &UnitView,
    selected: bool,
) -> Entity {
    let ListRow { row, text } = spawn_list_row(commands, parent, unit_color(unit), selected, true);
    spawn_row_title(commands, text, ui, language, unit_title(ui, language, unit));
    spawn_row_subtitle(commands, text, ui, language, unit_summary(unit));
    row
}

// Tabs

fn spawn_unit_tabs(
    commands: &mut Commands,
    section: Entity,
    ui: &UiContext,
    language: Language,
    active: UnitTab,
) {
    let tabs = commands.spawn((theme::row(6.0), ChildOf(section))).id();

    for (key, tab) in [
        ("combat_tab", UnitTab::Combat),
        ("builder_tab", UnitTab::Builder),
    ] {
        spawn_unit_tab(commands, tabs, ui, language, key, tab, active == tab);
    }
}

fn spawn_unit_tab(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    tab: UnitTab,
    active: bool,
) {
    let bg = if active {
        theme::PRIMARY_BG
    } else {
        theme::BUTTON_BG
    };
    let button = theme::spawn_text_button(
        commands,
        parent,
        ui,
        language,
        key,
        bg,
        theme::BUTTON_BG_HOVER,
    );

    commands.entity(button).observe(
        move |mut click: On<PointerClick>, mut view: ResMut<SidebarView>| {
            click.propagate(false);
            if view.unit_tab != tab {
                view.unit_tab = tab;
            }
        },
    );
}

// Details card

fn spawn_unit_details(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    unit: &UnitView,
    local_id: Id,
) {
    let card = commands
        .spawn((theme::card(theme::SLOT_BG), ChildOf(parent)))
        .id();

    // Title: "Fire Mages Damage II" + race name.
    let title = commands.spawn((theme::title_row(), ChildOf(card))).id();
    theme::spawn_label(
        commands,
        title,
        ui,
        language,
        unit_title(ui, language, unit),
        13.0,
        unit_color(unit),
    );
    theme::spawn_label(
        commands,
        title,
        ui,
        language,
        theme::race_display_name(&unit.race.0),
        9.0,
        theme::MUTED,
    );

    // Common stats.
    let owner = owner_label(ui, language, unit.owner, local_id);
    theme::spawn_stat_row(commands, card, ui, language, "owner", owner, theme::TEXT);
    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "health",
        format!("{} / {}", unit.hp, unit.max_hp),
        hp_color(unit.hp, unit.max_hp),
    );
    theme::spawn_health_bar(commands, card, 160.0, unit.hp, unit.max_hp);
    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "moves",
        format!("{} / {}", unit.moves, unit.moves_max),
        theme::MOVE_POINT,
    );

    // Builder vs. combat stats.
    if unit.kind == UnitType::Helper {
        theme::spawn_stat_row(
            commands,
            card,
            ui,
            language,
            "build_charges",
            unit.build_points.unwrap_or(0).to_string(),
            theme::TEXT,
        );
    } else {
        theme::spawn_stat_row(
            commands,
            card,
            ui,
            language,
            "attack_defense",
            format!("{} / {}", unit.attack, unit.defense),
            theme::TEXT,
        );
        theme::spawn_stat_row(
            commands,
            card,
            ui,
            language,
            "range",
            unit.range.to_string(),
            theme::TEXT,
        );
    }

    spawn_ability_list(commands, card, ui, language, unit);
}

fn spawn_ability_list(
    commands: &mut Commands,
    card: Entity,
    ui: &UiContext,
    language: Language,
    unit: &UnitView,
) {
    if unit.abilities.is_empty() {
        return;
    }

    theme::spawn_translation(
        commands,
        card,
        ui,
        language,
        "abilities",
        9.0,
        theme::GOLD_TEXT,
    );

    for ability in &unit.abilities {
        let stats = format!(
            "{} {} · {} {} · {} {} · {} {} · {} {}",
            ui.get(language, "damage"),
            ability.damage,
            ui.get(language, "mana"),
            ability.mana,
            ui.get(language, "cooldown"),
            ability.cooldown,
            ui.get(language, "range"),
            ability.range,
            ui.get(language, "area"),
            ability.area,
        );
        theme::spawn_wrapped_text(
            commands,
            card,
            ui,
            language,
            format!("{}: {stats}", ability.name),
            10.0,
            theme::MUTED,
        );
    }
}
