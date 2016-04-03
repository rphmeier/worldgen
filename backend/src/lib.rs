pub mod delaunay;
pub mod geometry;
pub mod grid;

use grid::Grid;

pub struct World {
    grid: Grid<f32>,
}