//! Shows/hides phase-specific HUD pieces and the overlays.

use bevy::prelude::*;

use crate::ui::in_game_ui::state::{HudPart, HudUiState};
use crate::world::turn_manager::{CombatPhase, TurnManager};

pub(crate) fn sync_hud_visibility(
    turn_manager: Res<TurnManager>,
    hud: Res<HudUiState>,
    mut parts: Query<(&HudPart, &mut Node)>,
) {
    let in_combat = !matches!(turn_manager.combat_phase, CombatPhase::Idle);

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

/// Only writes when the value actually changes, so UI layout is not re-run.
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
