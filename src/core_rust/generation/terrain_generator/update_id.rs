use crate::{
    core_rust::generation::races_generation::generate_cities::BuildingMapNoBevy,
    world::{components::CAPITAL_CONTROL_RADIUS, game_resources::terrain_map::TerrainMap},
};

impl TerrainMap {
    pub fn update_ids(&mut self, buildings: &BuildingMapNoBevy) {
        let size = self.size;
        if size == 0 {
            return;
        }

        let radius = CAPITAL_CONTROL_RADIUS as isize;

        for (index, building) in buildings.buildings.iter().enumerate() {
            let Some(building) = building else {
                continue;
            };
            if self.map[index].id.is_some() {
                continue;
            }

            let id = building.id;
            let center_row = (index / size) as isize;
            let center_col = (index % size) as isize;

            for row in center_row - radius..=center_row + radius {
                if row < 0 || row >= size as isize {
                    continue;
                }
                for col in center_col - radius..=center_col + radius {
                    if col < 0 || col >= size as isize {
                        continue;
                    }

                    let tile_index = row as usize * size + col as usize;
                    if self.map[tile_index].id.is_none() {
                        self.map[tile_index].id = Some(id);
                    }
                }
            }
        }
    }
}
