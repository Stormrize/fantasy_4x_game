use crate::world::components::{Id, PlayerRace};

mod fps_mainland_generation;
mod ideal_grid_positioning;
pub mod voronoi_diagram;

#[derive(Debug, Clone, Copy)]
pub struct CapitalCoord(pub usize, pub usize);

pub struct CapitalsPos {
    pub data: Vec<((Id, PlayerRace), CapitalCoord)>,
}
