//! Reusable UI pieces shared by several sections.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{SidebarSection, SidebarView};
use crate::ui::in_game_ui::theme;

// Section header

/// Clickable section header that folds/unfolds the tree section below it.
pub(crate) fn spawn_collapsible_header(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    open: bool,
    section: SidebarSection,
) {
    let header = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6.0),
                padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(theme::SLOT_BG),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_label(
        commands,
        header,
        ui,
        language,
        if open { "-" } else { "+" },
        12.0,
        theme::GOLD_TEXT,
    );
    theme::spawn_label(
        commands,
        header,
        ui,
        language,
        theme::resolve(ui, language, key).to_uppercase(),
        11.0,
        theme::GOLD_TEXT,
    );

    theme::add_hover(commands, header, theme::SLOT_BG, theme::SLOT_BG_HOVER);

    commands.entity(header).observe(
        move |mut click: On<PointerClick>, mut view: ResMut<SidebarView>| {
            click.propagate(false);
            view.toggle(section);
        },
    );
}

// Summary chip

/// `Units 3` style chip: muted caption followed by the value.
pub(crate) fn spawn_count_chip(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    value: usize,
) {
    let chip = commands.spawn((theme::row(4.0), ChildOf(parent))).id();
    theme::spawn_translation(commands, chip, ui, language, key, 9.0, theme::MUTED);
    theme::spawn_label(
        commands,
        chip,
        ui,
        language,
        value.to_string(),
        10.0,
        theme::TEXT,
    );
}

// List rows

/// A list row (`dot | text column`) and the column that holds its labels.
pub(crate) struct ListRow {
    pub row: Entity,
    pub text: Entity,
}

/// Spawns the shared skeleton of every list row. `clickable` rows get a hover
/// highlight; purely informational rows do not.
pub(crate) fn spawn_list_row(
    commands: &mut Commands,
    parent: Entity,
    dot: Color,
    selected: bool,
    clickable: bool,
) -> ListRow {
    let row = commands
        .spawn((theme::list_row(selected), ChildOf(parent)))
        .id();

    if clickable {
        theme::add_hover(
            commands,
            row,
            theme::list_row_bg(selected),
            theme::SLOT_BG_HOVER,
        );
    }

    theme::spawn_dot(commands, row, dot, 8.0);

    let text = commands.spawn((theme::grow_column(1.0), ChildOf(row))).id();
    ListRow { row, text }
}

pub(crate) fn spawn_row_title(
    commands: &mut Commands,
    text: Entity,
    ui: &UiContext,
    language: Language,
    title: String,
) {
    theme::spawn_label(commands, text, ui, language, title, 12.0, theme::TEXT);
}

pub(crate) fn spawn_row_subtitle(
    commands: &mut Commands,
    text: Entity,
    ui: &UiContext,
    language: Language,
    subtitle: String,
) {
    theme::spawn_label(commands, text, ui, language, subtitle, 9.0, theme::MUTED);
}

// Small "x" button

pub(crate) struct XButton {
    size: f32,
    radius: f32,
    font_size: f32,
    bg: Color,
    hover_bg: Color,
}

impl XButton {
    /// Closes the whole sidebar (player card).
    pub(crate) fn close() -> Self {
        Self {
            size: 22.0,
            radius: 4.0,
            font_size: 11.0,
            bg: theme::BUTTON_BG,
            hover_bg: theme::BUTTON_BG_HOVER,
        }
    }

    /// Removes one queued order.
    pub(crate) fn remove() -> Self {
        Self {
            size: 18.0,
            radius: 3.0,
            font_size: 10.0,
            bg: theme::DANGER_BG,
            hover_bg: theme::DANGER_BG_HOVER,
        }
    }
}

pub(crate) fn spawn_x_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    style: XButton,
) -> Entity {
    let button = commands
        .spawn((
            Node {
                width: Val::Px(style.size),
                height: Val::Px(style.size),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(style.radius)),
                ..default()
            },
            BackgroundColor(style.bg),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_label(commands, button, ui, language, "x", style.font_size, theme::TEXT);
    theme::add_hover(commands, button, style.bg, style.hover_bg);

    button
}
