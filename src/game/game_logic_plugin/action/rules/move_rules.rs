use crate::game::game_logic_plugin::action::rules::movement::{
    UNREACHABLE, biome_cost, movement_field,
};
use crate::world::components::Id;
use crate::world::game_configs::Player;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;
use bevy::math::IVec2;
use bevy::prelude::{Commands, DetectChanges, Res, ResMut, Resource};

/// Кэш стоимости передвижения по клеткам реальной карты (местность статична,
/// поэтому сетка строится один раз после генерации мира).
#[derive(Resource)]
pub struct MovementCostGrid {
    pub grid: Vec<u8>,
    pub size: usize,
}

impl MovementCostGrid {
    pub fn from_terrain(terrain: &TerrainMap) -> Self {
        let size = terrain.size;
        let mut grid = Vec::with_capacity(size * size);
        for row in 0..size {
            for col in 0..size {
                grid.push(biome_cost(terrain.get_biome_type(row, col)));
            }
        }
        Self { grid, size }
    }
}

/// Вставляет кэш при входе в игру (до первого кадра, где обрабатываются ходы).
pub fn insert_movement_cost_grid(mut commands: Commands, terrain: Res<TerrainMap>) {
    commands.insert_resource(MovementCostGrid::from_terrain(&terrain));
}

/// Пересобирает кэш, если карта была заменена (например, отладочная регенерация).
pub fn maintain_movement_cost_grid(
    terrain: Res<TerrainMap>,
    mut cost_grid: ResMut<MovementCostGrid>,
) {
    if terrain.is_changed() {
        *cost_grid = MovementCostGrid::from_terrain(&terrain);
    }
}

#[inline]
pub fn is_players_turn(player: Player, current_player: Player) -> bool {
    player == current_player
}

#[inline]
pub fn is_players_unit(unit_id: Id, player: Player) -> bool {
    unit_id == player.id
}

/// Проверяет ход юнита и возвращает стоимость пути в очках движения
/// (`None` — ход невозможен).
///
/// Учитывает: границы карты, занятость целевой клетки другим юнитом и
/// стоимость прохождения по биомам (с правилом «1 очка хватает на 1 шаг»).
pub fn check_move_validity(
    cost_grid: &MovementCostGrid,
    units_map: &UnitsMap,
    movement_points: u16,
    current_position: IVec2,
    target_position: IVec2,
) -> Option<u16> {
    let size = cost_grid.size;

    let in_bounds =
        |p: IVec2| p.x >= 0 && p.y >= 0 && (p.x as usize) < size && (p.y as usize) < size;

    if !in_bounds(current_position) || !in_bounds(target_position) {
        return None;
    }
    if current_position == target_position {
        return Some(0);
    }

    // На занятую другим юнитом клетку ходить нельзя.
    if units_map.is_some(target_position.x as usize, target_position.y as usize) {
        return None;
    }

    let dist = movement_field(
        &cost_grid.grid,
        size,
        (current_position.x as usize, current_position.y as usize),
        movement_points,
    );
    let cost = dist[target_position.x as usize * size + target_position.y as usize];

    if cost == UNREACHABLE || cost > movement_points {
        None
    } else {
        Some(cost)
    }
}
