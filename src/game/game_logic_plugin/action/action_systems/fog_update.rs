use crate::world::components::{CAPITAL_CONTROL_RADIUS, Id};
use crate::world::game_configs::GameLogicConfig;
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::unit::FieldOfView;
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::unit_map::UnitsMap;
use bevy::prelude::{DetectChanges, Query, Res, ResMut};

/// Пересчитывает туман войны по текущему положению юнитов и зданий:
/// юниты дают полную видимость (с юнитами) в радиусе своего обзора,
/// здания — общую видимость (без юнитов) в радиусе контроля.
/// Запускается после любых изменений карт юнитов/зданий.
pub fn update_fog(
    units_map: Res<UnitsMap>,
    buildings_map: Res<BuildingMap>,
    mut fog_map: ResMut<FogMap>,
    game_logic_config: Res<GameLogicConfig>,
    units_query: Query<(&Id, &FieldOfView)>,
    buildings_query: Query<&Id>,
) {
    if !units_map.is_changed() && !buildings_map.is_changed() {
        return;
    }

    let local_player_id = game_logic_config.player.id;
    let size = fog_map.size;
    for fog in fog_map.map.iter_mut() {
        fog.clear_visibility();
    }

    for (index, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok((id, field_of_view)) = units_query.get(entity) else {
            continue;
        };
        let (row, col) = (index / size, index % size);
        reveal_area(
            &mut fog_map,
            *id,
            local_player_id,
            row,
            col,
            field_of_view.0 as usize,
            true,
        );
    }

    for (index, slot) in buildings_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok(id) = buildings_query.get(entity) else {
            continue;
        };
        let (row, col) = (index / size, index % size);
        reveal_area(
            &mut fog_map,
            *id,
            local_player_id,
            row,
            col,
            CAPITAL_CONTROL_RADIUS,
            true,
        );
    }

    // debug: весь туман раскрыт локальному игроку
    if fog_map.debug_full_visibility {
        for row in 0..size {
            for col in 0..size {
                fog_map.set_visibility(row, col, local_player_id, local_player_id, true, true);
            }
        }
    }
}

fn reveal_area(
    fog_map: &mut FogMap,
    id: Id,
    local_player_id: Id,
    row: usize,
    col: usize,
    radius: usize,
    full: bool,
) {
    let size = fog_map.size;
    let radius = radius as isize;
    for dr in -radius..=radius {
        for dc in -radius..=radius {
            let r = row as isize + dr;
            let c = col as isize + dc;
            if r < 0 || c < 0 || r as usize >= size || c as usize >= size {
                continue;
            }
            fog_map.set_visibility(r as usize, c as usize, id, local_player_id, true, full);
        }
    }
}
