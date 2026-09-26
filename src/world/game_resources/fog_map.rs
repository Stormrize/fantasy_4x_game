use bevy::prelude::Resource;
#[derive(Resource)]
pub struct FogMap {
    pub map: Vec<bool>,
    pub size: usize,
}

impl FogMap {
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> bool {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        self.map[row * self.size + col]
    }
}
