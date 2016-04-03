//! Geometric primitives

/// A point in the 2-dimensional XY plane.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

/// A triangle is a polygon connecting 3 points.
pub type Triangle = [Point2D; 3];