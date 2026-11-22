//! "Buildings" section: own cities with inline details.

use bevy::prelude::*;

use crate::game::game_logic_plugin::components::TileLayer;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::SidebarSection;
use crate::ui::in_game_ui::theme;
use crate::world::components::Id;

use super::super::data::BuildingView;
use super::super::format::{
    building_key, building_summary, building_title, level_roman, owner_label,
};
use super::super::interactions::attach_select;
use super::super::model::SidebarModel;
use super::super::widgets::{
    ListRow, spawn_collapsible_header, spawn_list_row, spawn_row_subtitle, spawn_row_title,
};

pub(crate) fn spawn_buildings_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    model: &SidebarModel,
) {
    let view = model.view;
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();

    spawn_collapsible_header(
        commands,
        section,
        ui,
        language,
        "buildings",
        view.buildings_open,
        SidebarSection::Buildings,
    );

    if !view.buildings_open {
        return;
    }

    let own_cities: Vec<&BuildingView> = model.own_buildings().collect();

    if own_cities.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_buildings",
            10.0,
            theme::MUTED,
        );
        return;
    }

    for city in own_cities {
        let is_selected = model.is_selected(city.pos, TileLayer::Building);

        let ListRow { row, text } =
            spawn_list_row(commands, section, theme::GOLD_TEXT, is_selected, true);
        spawn_row_title(commands, text, ui, language, building_title(ui, language, city));
        spawn_row_subtitle(commands, text, ui, language, building_summary(city));

        attach_select(commands, row, city.pos, TileLayer::Building);

        if is_selected {
            spawn_building_details(commands, section, ui, language, city, model.local_id);
        }
    }
}

fn spawn_building_details(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    city: &BuildingView,
    local_id: Id,
) {
    let card = commands
        .spawn((theme::card(theme::SLOT_BG), ChildOf(parent)))
        .id();

    let title = commands.spawn((theme::title_row(), ChildOf(card))).id();
    theme::spawn_translation(
        commands,
        title,
        ui,
        language,
        building_key(city.kind),
        13.0,
        theme::GOLD_TEXT,
    );
    theme::spawn_label(
        commands,
        title,
        ui,
        language,
        level_roman(city.level),
        10.0,
        theme::TEXT,
    );

    let owner = owner_label(ui, language, city.owner, local_id);
    theme::spawn_stat_row(commands, card, ui, language, "owner", owner, theme::TEXT);
    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "yields",
        format!("{}g / {}f / {}s", city.gold, city.food, city.science),
        theme::TEXT,
    );
    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "maintenance",
        format!("{}g", city.maintenance),
        theme::MUTED,
    );
    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "housing_spawn",
        format!("{} / {}", city.housing, city.spawn_capacity),
        theme::TEXT,
    );
}
