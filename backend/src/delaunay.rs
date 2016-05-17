//! Delaunay triangulation in the 2D plane.
use std::collections::HashSet;

use geometry::{Point2D, Triangle};

/// Computes the delaunay triangulation of the given set of points
/// using the Bowyer-Watson algorithm.
pub fn delaunay(points: &[Point2D]) -> HashSet<Triangle> {
    let mut triangulation = HashSet::new();
    let super_triangle = super_triangle(points);
    triangulation.insert(super_triangle.clone());
    
    for point in points {
        let mut invalidated = HashSet::new();
        
        for triangle in triangulation.iter() {
            let circum = triangle.circumcircle();
            if circum.contains(point.clone()) {
                invalidated.insert(triangle.clone());
            }
        }
        
        let mut polygon = Vec::new();
        for triangle in invalidated.iter() {
            'a:
            for edge in triangle.edges().iter() {
                for t in invalidated.iter() {
                    if t != triangle {
                        for e in t.edges().iter() {
                            if edge == e {
                                continue 'a;
                            }
                        }
                    }
                }         
                polygon.push(edge.clone());
            }
        }
        
        for triangle in invalidated {
            triangulation.remove(&triangle);
        }
        
        for edge in polygon {
            let (a, b) = edge.endpoints();
            triangulation.insert(Triangle::new(*point, a, b));
        }
        
    }
    
    let super_edges = super_triangle.edges();
    let mut super_related = Vec::new();
    for triangle in triangulation.iter() {
        if super_edges.iter().zip(triangle.edges().iter()).any(|(a, b)| a == b) {
            super_related.push(triangle.clone());
        }
    }
    
    for triangle in super_related {
        triangulation.remove(&triangle);
    }
    
    triangulation
}

// Yields a triangle which encompasses all points given.
fn super_triangle(points: &[Point2D]) -> Triangle {  
    // A triangle with area 1 that has a portion in each quadrant.  
    let mut top = Point2D::new(0.0, 0.5);
    let mut left = Point2D::new(-1.0, -0.5);
    let mut right = Point2D::new(1.0, 0.5);
    
    for point in points {
        loop {
            let tri = Triangle::new(top, left, right);
            if tri.contains(*point) {
                break;
            }
            
            top = top * 2.0;
            left = left * 2.0;
            right = right * 2.0;
        }
    }
    
    Triangle::new(top, left, right)
}