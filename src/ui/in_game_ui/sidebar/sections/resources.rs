//! "Resources" section: discovered resource kinds with inline details.

use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::SidebarSection;
use crate::ui::in_game_ui::theme;
use crate::world::components::Id;
use crate::world::game_resources::data::resource::ResourceType;

use super::super::format::{common_kind_key, owner_label, rare_description_key, rare_kind_key};
use super::super::model::SidebarModel;
use super::super::widgets::{
    ListRow, spawn_collapsible_header, spawn_list_row, spawn_row_subtitle,
};

pub(crate) fn spawn_resources_section(
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
        "resources",
        view.resources_open,
        SidebarSection::Resources,
    );

    if !view.resources_open {
        return;
    }

    if model.discovery.resources.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_resources",
            10.0,
            theme::MUTED,
        );
        return;
    }

    let selected_resource = model.selected_resource();

    for entry in &model.discovery.resources {
        // The row of the selected tile's resource kind is highlighted and
        // expanded with that tile's details.
        let selected_here = selected_resource.filter(|resource| entry.matches(resource));

        // Rows are informational only: no hover, no click.
        let ListRow { text, .. } = spawn_list_row(
            commands,
            section,
            entry.color(),
            selected_here.is_some(),
            false,
        );
        theme::spawn_translation(commands, text, ui, language, entry.key(), 12.0, theme::TEXT);
        spawn_row_subtitle(
            commands,
            text,
            ui,
            language,
            format!(
                "{} {}  ·  {}",
                entry.tiles(),
                ui.get(language, "tiles"),
                entry.detail(ui, language)
            ),
        );

        if let Some(resource) = selected_here {
            spawn_resource_details(commands, section, ui, language, resource, model.local_id);
        }
    }
}

fn spawn_resource_details(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    resource: &ResourceType,
    local_id: Id,
) {
    let card = commands
        .spawn((theme::card(theme::PANEL_BG_SOFT), ChildOf(parent)))
        .id();

    match resource {
        ResourceType::Common(common) => {
            theme::spawn_translation(
                commands,
                card,
                ui,
                language,
                common_kind_key(common.kind),
                13.0,
                theme::TEXT,
            );
            theme::spawn_stat_row(
                commands,
                card,
                ui,
                language,
                "amount",
                common.amount.to_string(),
                theme::TEXT,
            );
            if let Some(owner) = common.owner_id {
                let claimed_by = owner_label(ui, language, owner, local_id);
                theme::spawn_stat_row(
                    commands,
                    card,
                    ui,
                    language,
                    "claimed_by",
                    claimed_by,
                    theme::TEXT,
                );
            }
        }
        ResourceType::Rare(rare) => {
            theme::spawn_translation(
                commands,
                card,
                ui,
                language,
                rare_kind_key(rare.kind),
                13.0,
                theme::GOLD_TEXT,
            );
            theme::spawn_stat_row(
                commands,
                card,
                ui,
                language,
                "power",
                rare.power.to_string(),
                theme::TEXT,
            );
            theme::spawn_wrapped_translation(
                commands,
                card,
                ui,
                language,
                rare_description_key(rare.kind),
                10.0,
                theme::MUTED,
            );
        }
    }
}
