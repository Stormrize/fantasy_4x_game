//! "Orders" section: queued combat orders, only shown during combat.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::game::game_logic_plugin::combat::integration::CombatOrders;
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::theme;

use super::super::format::skill_name;
use super::super::widgets::{XButton, spawn_x_button};

pub(crate) fn spawn_orders_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    orders: &CombatOrders,
    armed_skill: u8,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    theme::spawn_section_header(commands, section, ui, language, "orders");

    theme::spawn_stat_row(
        commands,
        section,
        ui,
        language,
        "skill",
        skill_name(ui, language, armed_skill),
        theme::GOLD_TEXT,
    );

    if orders.0.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_orders",
            10.0,
            theme::MUTED,
        );
        return;
    }

    for (index, order) in orders.0.iter().enumerate() {
        let row = spawn_order_row(commands, section);

        theme::spawn_label(
            commands,
            row,
            ui,
            language,
            format!(
                "({}, {}) → ({}, {})  ·  {}",
                order.attacker_pos.x,
                order.attacker_pos.y,
                order.target_pos.x,
                order.target_pos.y,
                skill_name(ui, language, order.skill_type)
            ),
            10.0,
            theme::TEXT,
        );

        let remove = spawn_x_button(commands, row, ui, language, XButton::remove());
        commands.entity(remove).observe(
            move |mut click: On<PointerClick>, mut orders: ResMut<CombatOrders>| {
                click.propagate(false);
                if index < orders.0.len() {
                    orders.0.remove(index);
                }
            },
        );
    }
}

fn spawn_order_row(commands: &mut Commands, parent: Entity) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(6.0),
                padding: UiRect::axes(Val::Px(7.0), Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(theme::SLOT_BG),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id()
}
