use crate::world::enums::units::{Level, Unit};
use crate::world::terrain_config::TerrainConfig;
use crate::world::unit_map::UnitsMap;

impl UnitsMap {
    pub fn new(config: &TerrainConfig) -> Self {
        let terrain_size = config.terrain_size;
        let data = vec![(Unit::default(), Level::default()); terrain_size * terrain_size];



        Self {
            units: data,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Unit {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        let index = row * self.size + col;
        self.units[index].0
    }
}