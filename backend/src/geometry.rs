//! Geometric primitives
use std::ops::{Add, Sub};

/// A point in the 2-dimensional XY plane.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    /// Create a new point with given coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        assert!(!x.is_nan(), "x NaN");
        assert!(!y.is_nan(), "y NaN");
        
        Point2D {
            x: x,
            y: y,
        }
    }
    
    /// Get the x-coordinate of this point.
    pub fn x(&self) -> f32 {
        self.x
    }
    
    /// Get the y-coordinate of this point.
    pub fn y(&self) -> f32 {
        self.y
    }
    
    /// Compute the distance between two points.
    pub fn distance(a: Self, b: Self) -> f32 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }
}

impl Add<Point2D> for Point2D {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Self;
    
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// A triangle is a polygon connecting 3 points.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Triangle {
    a: Point2D,
    b: Point2D,
    c: Point2D,
}

/// A circle is all points that have a fixed distance, known as the radius, from a center point.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Circle {
    center: Point2D,
    radius: f32,
}

impl Circle {
    /// Create a new circle with given center and radius.
    pub fn new(center: Point2D, radius: f32) -> Self {
        assert!(!radius.is_nan(), "radius NaN");
        
        Circle {
            center: center,
            radius: radius,
        }
    }
    
    /// Get the radius of this circle.
    pub fn radius(&self) -> f32 { self.radius }
    
    /// Get the center point of this circle.
    pub fn center(&self) -> Point2D { self.center }
}