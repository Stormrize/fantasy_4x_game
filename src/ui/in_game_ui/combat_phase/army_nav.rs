//! Navigation buttons that walk through your combat units:
//! first, previous, next, last.

use bevy::prelude::*;

use crate::game::events::action_messages::NewTile;
use crate::game::game_logic_plugin::components::{
    AlreadySelectedTile, JustSelectedTile, TileContent,
};
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::ArmyNav;
use crate::world::components::Id;
use crate::world::game_configs::GameLogicConfig;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::unit::UnitLevel;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::unit_map::UnitsMap;

use super::buttons::{ButtonTone, spawn_square_button};

pub(super) fn spawn_army_nav_buttons(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
) {
    for (label, nav) in [
        ("|<", ArmyNav::First),
        ("<", ArmyNav::Previous),
        (">", ArmyNav::Next),
        (">|", ArmyNav::Last),
    ] {
        spawn_nav_button(commands, parent, ui, language, label, nav);
    }
}

fn spawn_nav_button(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    label: &str,
    nav: ArmyNav,
) {
    let button = spawn_square_button(commands, parent, ui, language, label, ButtonTone::Normal);

    commands.entity(button).observe(
        move |mut click: On<PointerClick>,
              selected: Res<AlreadySelectedTile>,
              units_map: Res<UnitsMap>,
              buildings_map: Res<BuildingMap>,
              resources_map: Res<ResourceMap>,
              fog: Res<FogMap>,
              ids: Query<&Id>,
              combat_levels: Query<&UnitLevel>,
              config: Res<GameLogicConfig>,
              mut new_tile: MessageWriter<NewTile>| {
            click.propagate(false);

            let own = own_combat_positions(&units_map, &ids, &combat_levels, config.player.id);
            let current = selected
                .pos
                .and_then(|pos| own.iter().position(|candidate| *candidate == pos));

            let Some(target) = navigate(nav, &own, current) else {
                return;
            };

            new_tile.write(NewTile(JustSelectedTile {
                pos: target,
                tile_content: tile_content_at(
                    target,
                    &units_map,
                    &buildings_map,
                    &resources_map,
                    &fog,
                ),
            }));
        },
    );
}

/// Picks the unit to select next. `current` is the index of the currently
/// selected unit inside `own`, if the selection is one of them. Wraps around.
fn navigate(nav: ArmyNav, own: &[IVec2], current: Option<usize>) -> Option<IVec2> {
    let len = own.len();
    if len == 0 {
        return None;
    }

    let index = match (nav, current) {
        (ArmyNav::First, _) | (ArmyNav::Next, None) => 0,
        (ArmyNav::Last, _) | (ArmyNav::Previous, None) => len - 1,
        (ArmyNav::Next, Some(index)) => (index + 1) % len,
        (ArmyNav::Previous, Some(index)) => (index + len - 1) % len,
    };
    own.get(index).copied()
}

/// Positions of the local player's combat units in map order.
/// Builders cannot receive combat orders, so navigation skips them too.
fn own_combat_positions(
    units_map: &UnitsMap,
    ids: &Query<&Id>,
    combat_levels: &Query<&UnitLevel>,
    owner: Id,
) -> Vec<IVec2> {
    let size = units_map.size;
    let mut positions = Vec::new();

    for (index, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok(id) = ids.get(entity) else { continue };
        if *id != owner || combat_levels.get(entity).is_err() {
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
