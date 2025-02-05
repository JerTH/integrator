use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::line::Line;
use crate::{Approximately, Float, EPSILON};
use crate::{ Vector, Point };

pub const PLANE_XY: Plane = Plane { norm: Vector::new(0.0, 0.0, 1.0), dist: 0.0 };
pub const PLANE_XZ: Plane = Plane { norm: Vector::new(0.0, 1.0, 0.0), dist: 0.0 };
pub const PLANE_YZ: Plane = Plane { norm: Vector::new(1.0, 0.0, 0.0), dist: 0.0 };

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub norm: Vector,
    pub dist: Float,
}

impl Plane {
    pub fn new(normal: Vector, distance: Float) -> Self {
        Self {
            norm: normal.normalized(),
            dist: distance,
        }
    }

    pub fn invert(&mut self) {
        *self = self.inverted();
    }

    pub fn inverted(&self) -> Self {
        Self {
            norm: &self.norm * -1.0,
            dist: &self.dist * -1.0,
        }
    }

    pub fn distance_to(&self, point: Point) -> Float {
        self.norm.dot(&point.as_vector()) - self.dist
    }

    /// Project a point to the closest point on the plane
    pub fn project_point(&self, point: Point) -> Point {
        let d = self.distance_to(point);
        point - self.norm * d
    }

    /// Test whether the point is on the positive half of the plane
    pub fn point_on_positive_half(&self, point: Point) -> bool {
        self.distance_to(point).signum() > 0.0
    }

    /// Test whether the plane is parallel with another plane
    pub fn is_parallel(&self, other: &Plane) -> bool {
        self.norm.cross(&other.norm).length_sq().approximately(0.0, EPSILON)
    }
    
    /// Test whether the plane is perpendicular to another plane
    pub fn is_perpendicular(&self, other: &Plane) -> bool {
        self.norm.dot(&other.norm).abs().approximately(0.0, EPSILON)
    }

    /// Compute the intersection `t` along a ray where it hits the plane
    pub fn ray_intersection(&self, origin: Point, direction: Vector) -> Option<Float> {
        let denom = self.norm.dot(&direction);
        if denom.abs().approximately(0.0, EPSILON) {
            None // Ray is parallel to the plane
        } else {
            let t = (self.dist - self.norm.dot(&origin.as_vector())) / denom;
            (t >= 0.0).then_some(t)
        }
    }
    
    pub fn line_of_intersection(&self, other: &Plane) -> Option<Line> {
        if self.is_parallel(other) {
            return None;
        }

        let direction = self.norm.cross(&other.norm);

        let (norm1, dist1) = (&self.norm, &self.dist);
        let (norm2, dist2) = (&other.norm, &other.dist);
        
        let num = (dist1 * norm2.cross(&direction)) + (dist2 * norm1.cross(&direction));
        let den = direction.dot(&direction);

        if den.abs().approximately(0.0, EPSILON) {
            return None;
        }

        let origin = (num / den).into();
        return Some(Line { origin, direction });
    }

    pub fn angle_between(&self, other: &Plane) -> Float {
        self.norm.dot(&other.norm).abs().acos()
    }
}

impl<'a, P> From<&'a [P; 3]> for Plane where P: Deref<Target = Point> {
    fn from(points: &'a [P; 3]) -> Self {
        let (a, b, c) = (&points[0], &points[1], &points[2]);
        let ab = b.as_vector() - a.as_vector();
        let ac = c.as_vector() - a.as_vector();
        let norm = (ab).cross(&ac);
        let dist = norm.dot(&a.as_vector());

        Plane { norm, dist }
    }
}

impl<P> From<(P, P, P)> for Plane where P: Deref<Target = Point> {
    fn from(points: (P, P, P)) -> Self {
        Plane::from(&[points.0, points.1, points.2])
    }
}

#[cfg(test)]
mod plane_tests {
    use crate::vec::{X_AXIS, Y_AXIS, Z_AXIS};

    use super::*;

    fn approx_eq(a: Float, b: Float) -> bool {
        a.approximately(b, EPSILON)
    }

    #[test]
    fn test_line_intersection_xy_xz() {
        let plane_xy = PLANE_XY;
        let plane_xz = PLANE_XZ;
        let expected = X_AXIS;
        let intersect = plane_xy.line_of_intersection(&plane_xz).expect("Expected an intercept between XY and XZ planes");
        
        // The intersection point should lie on both planes.
        assert!(approx_eq(plane_xy.distance_to(intersect.origin), 0.0),
            "Intersection point is not on PLANE_XY.");
        assert!(approx_eq(plane_xz.distance_to(intersect.origin), 0.0),
            "Intersection point is not on PLANE_XZ.");

        assert!(&intersect.direction.parallel_to(&expected),
            "Intersection direction is not parallel to the x-axis.");
    }

    #[test]
    fn test_line_intersection_xy_rotated() {
        let plane_xy = PLANE_XY;
        let normal_rotated = Vector::new(0.70710678, 0.0, 0.70710678);
        let plane_rotated = Plane::new(normal_rotated, 0.0);
        let intersect = plane_xy.line_of_intersection(&plane_rotated).expect("Expected an intersect between XY and rotated plane");
        let expected = Y_AXIS;

        // The intersection point must lie on both planes.
        assert!(approx_eq(plane_xy.distance_to(intersect.origin), 0.0),
            "Intersection point is not on the XY plane.");
        assert!(approx_eq(plane_rotated.distance_to(intersect.origin), 0.0),
            "Intersection point is not on the rotated plane.");

        assert!(&intersect.direction.parallel_to(&expected),
            "Intersection direction is not parallel to the y-axis.");
    }

    #[test]
    fn test_parallel_planes() {
        // Create two parallel planes: the XY plane and a shifted XY plane (z = 1).
        let plane1 = Plane::new(Z_AXIS, 0.0);
        let plane2 = Plane::new(Z_AXIS, 1.0);
        let intersect = plane1.line_of_intersection(&plane2);
        assert!(intersect.is_none(), "Expected no intersection line between parallel planes.");
    }

    #[test]
    fn test_inverted_planes_intersection() {
        // Inverting a plane should not change the intersection line.
        let plane1 = PLANE_XY;
        let mut plane2 = PLANE_XZ;
        
        let intersect = plane1.line_of_intersection(&plane2).expect("Expected an intersection between XY and XZ planes");

        plane2.invert();
        let intersect_inverted = plane1.line_of_intersection(&plane2).expect("Expected an intersection between inverted planes");


        // Both intersection points must lie on plane1.
        assert!(approx_eq(plane1.distance_to(intersect.origin), 0.0),
            "Intersection point pt1 is not on PLANE_XY.");
        assert!(approx_eq(plane1.distance_to(intersect_inverted.origin), 0.0),
            "Intersection point pt2 is not on PLANE_XY.");

        // The directions returned should be parallel (or anti‐parallel) even if one plane was inverted.
        assert!(&intersect.direction.parallel_to(&intersect_inverted.direction),
            "Intersection directions differ between the original and inverted plane cases.");
    }
}
