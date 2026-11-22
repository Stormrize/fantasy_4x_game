//! Skill selection buttons and the armed-skill highlight.

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{CombatUiState, SkillButton};
use crate::ui::in_game_ui::theme;

/// `ATK`, `1`, `2`, `3`
pub(super) fn spawn_skill_buttons(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    for (skill, label) in [(0u8, "ATK"), (1, "1"), (2, "2"), (3, "3")] {
        spawn_skill_button(commands, parent, ui, language, skill, label);
    }
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
