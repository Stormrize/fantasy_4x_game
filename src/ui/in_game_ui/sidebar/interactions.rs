//! Click observers attached to sidebar rows.

use bevy::prelude::*;

use crate::game::events::action_messages::NewTile;
use crate::game::game_logic_plugin::combat::integration::CombatOrders;
use crate::game::game_logic_plugin::combat::model::CombatCommand;
use crate::game::game_logic_plugin::components::{
    AlreadySelectedTile, JustSelectedTile, TileContent, TileLayer,
};
use crate::ui::in_game_ui::state::CombatUiState;
use crate::world::components::Id;
use crate::world::game_configs::GameLogicConfig;
use crate::world::game_resources::data::unit::UnitLevel;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::{CombatPhase, TurnManager};

use super::grid::cell_index;

/// Selects a tile by both position and layer. Rows know which layer they
/// represent, so selecting a building under a unit still shows the building.
pub(crate) fn attach_select(commands: &mut Commands, row: Entity, pos: IVec2, layer: TileLayer) {
    commands.entity(row).observe(
        move |mut click: On<PointerClick>, mut selected: ResMut<AlreadySelectedTile>| {
            click.propagate(false);
            selected.pos = Some(pos);
            selected.layer = Some(layer);
        },
    );
}

/// Click on an enemy row.
///
/// While planning and one of our combat units is selected, it queues an order
/// against `pos` (replacing the unit's previous order). Otherwise it just
/// selects the tile.
pub(crate) fn attach_queue_order(
    commands: &mut Commands,
    row: Entity,
    pos: IVec2,
    content: TileContent,
) {
    commands.entity(row).observe(
        move |mut click: On<PointerClick>,
              selected: Res<AlreadySelectedTile>,
              units_map: Res<UnitsMap>,
              ids: Query<&Id>,
              combat_levels: Query<&UnitLevel>,
              config: Res<GameLogicConfig>,
              turn_manager: Res<TurnManager>,
              combat_ui: Res<CombatUiState>,
              mut orders: ResMut<CombatOrders>,
              mut new_tile: MessageWriter<NewTile>| {
            click.propagate(false);

            if matches!(turn_manager.combat_phase, CombatPhase::Planning)
                && let Some(attacker_pos) = selected_own_combat_unit(
                    &selected,
                    &units_map,
                    &ids,
                    &combat_levels,
                    config.player.id,
                )
            {
                orders.0.retain(|order| order.attacker_pos != attacker_pos);
                orders.0.push(CombatCommand {
                    attacker_pos,
                    target_pos: pos,
                    skill_type: combat_ui.armed_skill,
                });
                return;
            }

            new_tile.write(NewTile(JustSelectedTile {
                pos,
                tile_content: content.clone(),
            }));
        },
    );
}

/// Position of the selected unit if it belongs to `local_id` and is a combat
/// unit (builders have no `UnitLevel` and cannot attack).
fn selected_own_combat_unit(
    selected: &AlreadySelectedTile,
    units_map: &UnitsMap,
    ids: &Query<&Id>,
    combat_levels: &Query<&UnitLevel>,
    local_id: Id,
) -> Option<IVec2> {
    let pos = selected
        .pos
        .filter(|_| selected.layer == Some(TileLayer::Unit))?;
    let index = cell_index(pos, units_map.size)?;
    let entity = units_map.map.get(index)?.as_ref()?;

    let is_own = ids.get(*entity).is_ok_and(|id| *id == local_id);
    let is_combat = combat_levels.get(*entity).is_ok();

    (is_own && is_combat).then_some(pos)
}
