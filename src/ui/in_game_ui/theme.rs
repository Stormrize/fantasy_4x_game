//! Dark-fantasy palette and small UI building blocks shared by the in-game HUD.
//!
//! Everything visual is defined here so the HUD has a single source of truth
//! for its look. Panels are semi-transparent so the map stays readable behind
//! them, and every interactive surface carries [`Pickable`] so clicks on the
//! HUD never leak into the world (see `click_to_world`).

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::colors::{COLOR_ACCENT_GOLD, COLOR_TEXT_MUTED, COLOR_TEXT_PRIMARY};
use crate::ui::{TranslationKey, UiContext};

// ---------------------------------------------------------------------------
// Palette
// ---------------------------------------------------------------------------

pub const PANEL_BG: Color = Color::srgba(0.071, 0.059, 0.051, 0.95);
pub const PANEL_BG_SOFT: Color = Color::srgba(0.114, 0.098, 0.086, 0.94);
pub const SLOT_BG: Color = Color::srgba(0.157, 0.129, 0.110, 0.85);
pub const SLOT_BG_HOVER: Color = Color::srgba(0.235, 0.192, 0.145, 0.95);
pub const HIGHLIGHT_BG: Color = Color::srgba(0.478, 0.353, 0.149, 0.55);
pub const OVERLAY_BG: Color = Color::srgba(0.020, 0.016, 0.012, 0.82);

pub const BORDER: Color = Color::srgba(0.769, 0.604, 0.290, 0.55);
pub const BORDER_SOFT: Color = Color::srgba(0.769, 0.604, 0.290, 0.22);

pub const TEXT: Color = COLOR_TEXT_PRIMARY;
pub const MUTED: Color = COLOR_TEXT_MUTED;
pub const GOLD_TEXT: Color = COLOR_ACCENT_GOLD;

pub const BUTTON_BG: Color = Color::srgba(0.173, 0.141, 0.118, 0.95);
pub const BUTTON_BG_HOVER: Color = Color::srgba(0.267, 0.216, 0.169, 0.98);
pub const PRIMARY_BG: Color = Color::srgba(0.302, 0.098, 0.098, 0.98);
pub const PRIMARY_BG_HOVER: Color = Color::srgba(0.435, 0.145, 0.145, 1.0);
pub const DANGER_BG: Color = Color::srgba(0.267, 0.078, 0.078, 0.98);
pub const DANGER_BG_HOVER: Color = Color::srgba(0.400, 0.118, 0.118, 1.0);

pub const MANA: Color = Color::srgb(0.235, 0.478, 0.902);
pub const MANA_TEXT: Color = Color::srgb(0.612, 0.769, 0.988);
pub const MANA_TRACK: Color = Color::srgba(0.043, 0.078, 0.153, 0.95);

pub const RES_GOLD: Color = Color::srgb(0.886, 0.714, 0.259);
pub const RES_FOOD: Color = Color::srgb(0.427, 0.702, 0.271);
pub const RES_SCIENCE: Color = Color::srgb(0.463, 0.612, 0.929);

pub const HP_GOOD: Color = Color::srgb(0.404, 0.729, 0.365);
pub const HP_WARN: Color = Color::srgb(0.855, 0.690, 0.235);
pub const HP_LOW: Color = Color::srgb(0.831, 0.290, 0.290);

pub const MOVE_POINT: Color = Color::srgb(0.639, 0.812, 0.925);
pub const ENEMY: Color = Color::srgb(0.878, 0.420, 0.353);

/// Accent colour used for race-tinted dots and icons.
pub fn race_color(race: &str) -> Color {
    match race {
        "fire_mages" => Color::srgb(0.855, 0.420, 0.235),
        "water_mages" => Color::srgb(0.310, 0.561, 0.835),
        "stone_mages" => Color::srgb(0.525, 0.627, 0.310),
        _ => Color::srgb(0.600, 0.400, 0.400),
    }
}

/// `fire_mages` -> `Fire Mages`.
pub fn race_display_name(race: &str) -> String {
    race.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ---------------------------------------------------------------------------
// Layout helpers
// ---------------------------------------------------------------------------

pub fn column(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        row_gap: Val::Px(gap),
        ..default()
    }
}

/// Column that takes the remaining row width, used for list item text.
pub fn grow_column(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        flex_grow: 1.0,
        row_gap: Val::Px(gap),
        ..default()
    }
}

pub fn row(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(gap),
        ..default()
    }
}

pub fn spacer() -> Node {
    Node {
        flex_grow: 1.0,
        ..default()
    }
}

/// Bordered panel used by the sidebar detail cards.
pub fn card(bg: Color) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(3.0),
            padding: UiRect::all(Val::Px(7.0)),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(BORDER_SOFT),
        Pickable::IGNORE,
    )
}

/// Card header: the name on the left, a secondary value on the right.
pub fn title_row() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable::IGNORE,
    )
}

/// Background of a [`list_row`], matching its selected state.
pub fn list_row_bg(selected: bool) -> Color {
    if selected { HIGHLIGHT_BG } else { SLOT_BG }
}

/// Full-width clickable row of a sidebar list. Callers attach their own click
/// observer and hover colours.
pub fn list_row(selected: bool) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(7.0),
            padding: UiRect::axes(Val::Px(7.0), Val::Px(4.0)),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(list_row_bg(selected)),
        BorderColor::all(if selected { BORDER } else { BORDER_SOFT }),
        Pickable::default(),
    )
}

// Text

/// Resolves `value` through [`UiContext::get`]. Locale keys render their
/// translation; literals such as `">"`, `"0 / 0"` or already formatted values
/// render unchanged because `get` falls back to the key itself.
pub fn resolve(ui: &UiContext, language: Language, value: &str) -> String {
    ui.get(language, value).to_string()
}

/// Text bundle rendered verbatim. Prefer [`label`] so locale keys are resolved.
fn label_raw(
    ui: &UiContext,
    language: Language,
    value: impl Into<String>,
    size: f32,
    color: Color,
) -> impl Bundle {
    (
        Text::new(value),
        TextFont {
            font: FontSource::Handle(ui.font(language)),
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(color),
    )
}

/// Text bundle whose content is resolved through the localization tables.
pub fn label(
    ui: &UiContext,
    language: Language,
    value: impl Into<String>,
    size: f32,
    color: Color,
) -> impl Bundle {
    let value = value.into();
    let text = resolve(ui, language, &value);
    label_raw(ui, language, text, size, color)
}

/// [`label`] tagged with [`TranslationKey`] so it follows language changes.
fn keyed_label(
    ui: &UiContext,
    language: Language,
    key: &'static str,
    size: f32,
    color: Color,
) -> impl Bundle {
    (label(ui, language, key, size, color), TranslationKey(key))
}

/// Spawns a text bundle as a child of `parent`.
fn spawn_text(commands: &mut Commands, parent: Entity, text: impl Bundle) -> Entity {
    commands.spawn((text, ChildOf(parent))).id()
}

/// Spawns a full-width word-wrapped text bundle as a child of `parent`.
fn spawn_wrapped(commands: &mut Commands, parent: Entity, text: impl Bundle) -> Entity {
    commands
        .spawn((
            text,
            TextLayout::new(Justify::Left, LineBreak::WordBoundary),
            Node {
                width: Val::Percent(100.0),
                ..default()
            },
            ChildOf(parent),
        ))
        .id()
}

pub fn spawn_label(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    value: impl Into<String>,
    size: f32,
    color: Color,
) -> Entity {
    spawn_text(commands, parent, label(ui, language, value, size, color))
}

/// Spawns a label bound to a static locale key; it is re-resolved when the
/// language changes.
pub fn spawn_translation(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    size: f32,
    color: Color,
) -> Entity {
    spawn_text(
        commands,
        parent,
        keyed_label(ui, language, key, size, color),
    )
}

pub fn spawn_wrapped_text(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    value: impl Into<String>,
    size: f32,
    color: Color,
) -> Entity {
    spawn_wrapped(commands, parent, label(ui, language, value, size, color))
}

/// [`spawn_wrapped_text`] bound to a static locale key.
pub fn spawn_wrapped_translation(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    size: f32,
    color: Color,
) -> Entity {
    spawn_wrapped(
        commands,
        parent,
        keyed_label(ui, language, key, size, color),
    )
}

/// Uppercase section header with a hairline under it.
pub fn spawn_section_header(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
) -> Entity {
    let header = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(3.0),
                padding: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

    commands.spawn((
        label_raw(
            ui,
            language,
            resolve(ui, language, key).to_uppercase(),
            11.0,
            GOLD_TEXT,
        ),
        TranslationKey(key),
        ChildOf(header),
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            ..default()
        },
        BackgroundColor(BORDER_SOFT),
        ChildOf(header),
    ));

    header
}

/// Stat row: the label comes from a locale key, the value is caller-provided.
pub fn spawn_stat_row(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    value: impl Into<String>,
    value_color: Color,
) -> Entity {
    let row_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

    spawn_translation(commands, row_entity, ui, language, key, 12.0, MUTED);
    spawn_label(commands, row_entity, ui, language, value, 12.0, value_color);

    row_entity
}

// Buttons

pub fn button_frame(bg: Color) -> impl Bundle {
    (
        Node {
            padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(BORDER_SOFT),
        Pickable::default(),
    )
}

/// Creates a button with the given label bundle and hover colours. The
/// returned entity is clickable; callers attach their own click observer.
fn spawn_labeled_button(
    commands: &mut Commands,
    parent: Entity,
    label: impl Bundle,
    bg: Color,
    hover: Color,
) -> Entity {
    let button = commands.spawn((button_frame(bg), ChildOf(parent))).id();
    commands.spawn((label, ChildOf(button)));
    add_hover(commands, button, bg, hover);

    button
}

/// Small text button bound to a static locale key.
pub fn spawn_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    bg: Color,
    hover: Color,
) -> Entity {
    let label = (
        label_raw(
            ui,
            language,
            resolve(ui, language, key).to_uppercase(),
            11.0,
            TEXT,
        ),
        TranslationKey(key),
    );

    spawn_labeled_button(commands, parent, label, bg, hover)
}

/// Button with a free-form label, used for values with numbers inside such as
/// `Upgrade 40g`. The label is passed through localization, so a locale key
/// also works.
pub fn spawn_text_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    value: impl Into<String>,
    bg: Color,
    hover: Color,
) -> Entity {
    let value = value.into();
    let label = label_raw(
        ui,
        language,
        resolve(ui, language, &value).to_uppercase(),
        11.0,
        TEXT,
    );

    spawn_labeled_button(commands, parent, label, bg, hover)
}

/// Changes the background while the pointer is inside the entity's subtree.
pub fn add_hover(commands: &mut Commands, entity: Entity, base: Color, hover: Color) {
    commands.entity(entity).observe(
        move |mut over: On<PointerEnter>, mut backgrounds: Query<&mut BackgroundColor>| {
            over.propagate(false);
            if let Ok(mut background) = backgrounds.get_mut(entity) {
                background.0 = hover;
            }
        },
    );

    commands.entity(entity).observe(
        move |mut out: On<PointerLeave>, mut backgrounds: Query<&mut BackgroundColor>| {
            out.propagate(false);
            if let Ok(mut background) = backgrounds.get_mut(entity) {
                background.0 = base;
            }
        },
    );
}

/// Round icon-ish node used for race dots and resource dots.
pub fn spawn_dot(commands: &mut Commands, parent: Entity, color: Color, size: f32) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Px(size),
                height: Val::Px(size),
                min_width: Val::Px(size),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(color),
            Pickable::IGNORE,
            ChildOf(parent),
        ))
        .id()
}

pub fn spawn_race_icon(
    commands: &mut Commands,
    parent: Entity,
    image: Handle<Image>,
    size: f32,
) -> Entity {
    commands
        .spawn((
            ImageNode { image, ..default() },
            Node {
                width: Val::Px(size),
                height: Val::Px(size),
                min_width: Val::Px(size),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(size * 0.18)),
                ..default()
            },
            BorderColor::all(BORDER_SOFT),
            Pickable::IGNORE,
            ChildOf(parent),
        ))
        .id()
}

/// Thin horizontal health bar.
pub fn spawn_health_bar(
    commands: &mut Commands,
    parent: Entity,
    width: f32,
    current: u16,
    max: u16,
) -> Entity {
    let ratio = if max == 0 {
        0.0
    } else {
        (current as f32 / max as f32).clamp(0.0, 1.0)
    };
    let color = if ratio > 0.5 {
        HP_GOOD
    } else if ratio > 0.25 {
        HP_WARN
    } else {
        HP_LOW
    };

    let track = commands
        .spawn((
            Node {
                width: Val::Px(width),
                height: Val::Px(5.0),
                border_radius: BorderRadius::MAX,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            Pickable::IGNORE,
            ChildOf(parent),
        ))
        .id();

    commands.spawn((
        Node {
            width: Val::Percent(ratio * 100.0),
            height: Val::Percent(100.0),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BackgroundColor(color),
        Pickable::IGNORE,
        ChildOf(track),
    ));

    track
}
