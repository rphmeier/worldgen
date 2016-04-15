//! Geometric primitives
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub, Div, Mul, Neg};

/// A point in the 2-dimensional XY plane.
/// 
/// The x and y components are guaranteed to be finite numbers.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    /// Create a new point with given coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        assert!(x.is_finite(), "x not finite");
        assert!(y.is_finite(), "y not finite");
        
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

impl Add for Point2D {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Point2D {
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
        assert!(rhs.is_finite(), "attempting to scale point by non-finite factor.");
        
        Point2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<f32> for Point2D {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        assert!(rhs.is_normal(), "attempting to scale point by non-finite factor");
        
        Point2D {
            x: self.x * rhs,
            y: self.y * rhs,
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

impl Hash for Point2D {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        use std::mem;
        
        assert!(self.x.is_finite());
        assert!(self.y.is_finite());
        
        unsafe {
            state.write_u32(mem::transmute(self.x));
            state.write_u32(mem::transmute(self.y));
        }
        
    }    
}

impl Eq for Point2D {}

/// A triangle is a polygon connecting 3 points.
#[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd)]
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
    
    /// The three edges of this triangle.
    pub fn edges(&self) -> [Segment; 3] {
        [
            Segment::new(self.a, self.b),
            Segment::new(self.b, self.c),
            Segment::new(self.c, self.a),
        ]
    }
    
    /// Whether this triangle contains the given point.
    // http://www.blackpawn.com/texts/pointinpoly/
    pub fn contains(&self, point: Point2D) -> bool {
        fn dot(p1: Point2D, p2: Point2D) -> f32 {
            p1.x * p2.x + p1.y * p2.y
        }
        
        let v0 = self.c - self.a;
        let v1 = self.b - self.a;
        let v2 = point - self.a;
        
        let dot00 = dot(v0, v0);
        let dot01 = dot(v0, v1);
        let dot02 = dot(v0, v2);
        let dot11 =  dot(v1, v1);
        let dot12 =  dot(v1, v2);
        
        let d = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * d;
        let v = (dot00 * dot12 - dot01 * dot02) * d;
        
        u >= 0.0 && v >= 0.0 && u + v < 1.0
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// A segment connecting two points.
///
/// The start is always the leftmost of the two points.
/// in cases where the x-coordinate is the same, the start
/// is the lower of the two.
pub struct Segment {
    start: Point2D,
    end: Point2D,
}

impl Segment {
    /// Create a new segment from given points.
    pub fn new(a: Point2D, b: Point2D) -> Self {
        if b < a {
            Segment {
                start: b,
                end: a,
            }
        } else {
            Segment {
                start: a,
                end: b,
            }
        }
    }
    
    /// Get the endpoints of this segment.
    pub fn endpoints(&self) -> (Point2D, Point2D) {
        (self.start, self.end)
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
    
    /// Whether this circle contains the given point.
    pub fn contains(&self, point: Point2D) -> bool {
        Point2D::distance(self.center, point) <= self.radius
    }
}