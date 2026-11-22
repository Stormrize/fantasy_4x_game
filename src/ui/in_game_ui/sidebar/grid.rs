//! Grid helpers: row-major cell math and an `O(1)` `pos -> item` lookup.

use bevy::prelude::IVec2;

/// `true` when `pos` lies inside a `size × size` map.
pub(crate) fn pos_in_bounds(pos: IVec2, size: usize) -> bool {
    pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < size && (pos.y as usize) < size
}

/// Index of `pos` in a map stored as `x * size + y`, or `None` when out of bounds.
pub(crate) fn cell_index(pos: IVec2, size: usize) -> Option<usize> {
    pos_in_bounds(pos, size).then(|| pos.x as usize * size + pos.y as usize)
}

/// Inverse of [`cell_index`].
pub(crate) fn cell_pos(index: usize, size: usize) -> IVec2 {
    IVec2::new((index / size) as i32, (index % size) as i32)
}

/// Items stored in map order plus a `pos -> item` lookup that is `O(1)`.
pub(crate) struct GridIndex<T> {
    items: Vec<T>,
    cells: Vec<Option<usize>>,
    size: usize,
}

impl<T> GridIndex<T> {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            items: Vec::new(),
            cells: vec![None; size * size],
            size,
        }
    }

    /// Stores `item` for the map cell with the given row-major `cell` index.
    pub(crate) fn insert(&mut self, cell: usize, item: T) {
        self.cells[cell] = Some(self.items.len());
        self.items.push(item);
    }

    pub(crate) fn at(&self, pos: IVec2) -> Option<&T> {
        let cell = cell_index(pos, self.size)?;
        let slot = self.cells.get(cell).copied().flatten()?;
        self.items.get(slot)
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}
