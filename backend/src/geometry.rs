//! Geometric primitives
use std::ops::{Add, Sub, Div, Neg};

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
    
    /// The origin point.
    pub fn origin() -> Self {
        Point2D::new(0.0, 0.0)
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

impl Div<f32> for Point2D {
    type Output = Self;
    
    #[inline]
    fn div(self, rhs: f32) -> Self {
        assert!(rhs != 0.0, "Error: division by 0");
        
        Point2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Point2D {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self {
        Point2D::origin() - self
    }
}

/// A triangle is a polygon connecting 3 points.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Triangle {
    a: Point2D,
    b: Point2D,
    c: Point2D,
}

impl Triangle {
    /// Create a new triangle with the given points.
    pub fn new(a: Point2D, b: Point2D, c: Point2D) -> Self {
        Triangle {
            a: a,
            b: b,
            c: c,
        }
    }
    
    /// Yields the points of the triangle as a tuple.
    pub fn points(&self) -> (Point2D, Point2D, Point2D) {
        (self.a, self.b, self.c)
    }
    
    /// Yields the circumcircle of the triangle.
    pub fn circumcircle(&self) -> Circle {
        // to simplify the calculation, we translate the triangle to the origin.
        // after this, we only need b and c.
        let b = self.b - self.a;
        let c = self.c - self.a;
        
        // find the circumcenter of the translated triangle.
        let d = 2.0 * ((b.x * c.y) - (c.x * b.y));
        let k =   c.y * (b.x.powi(2) + b.y.powi(2))
                - b.y * (c.x.powi(2) + c.y.powi(2));
                
        let x = k / d;
        let y = (-k) / d;
        
        // Translate the center back and find the distance to any of the triangle's 
        // vertices
        let center = Point2D::new(x, y) + self.a;
        let radius = Point2D::distance(center, self.a);
        
        Circle::new(center, radius)
    }
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