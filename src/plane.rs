//! 
//! Infinite planes in 3D space
//! 

use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;

use crate::line::Line;
use crate::traits::FloatExt;
use crate::traits::Intersects;
use crate::traits::Parallel;
use crate::Approximately;
use crate::Float;
use crate::Point;
use crate::Vector;

const EPSILON: Float = Float::EPSILON;

pub const PLANE_XY: Plane = Plane {
    norm: Vector::unit_z(),
    dist: Float::ZERO,
};
pub const PLANE_XZ: Plane = Plane {
    norm: Vector::unit_y(),
    dist: Float::ZERO,
};
pub const PLANE_YZ: Plane = Plane {
    norm: Vector::unit_x(),
    dist: Float::ZERO,
};

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
            norm: &self.norm * -Float::ONE,
            dist: &self.dist * -Float::ONE,
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
        self.distance_to(point).signum() > Float::ZERO
    }

    /// Test whether the plane is perpendicular to another plane
    pub fn perpendicular_to(&self, other: &Plane) -> bool {
        self.norm.dot(&other.norm).abs().approximately(0.0, EPSILON)
    }

    pub fn line_of_intersection(&self, other: &Plane) -> Option<Line> {
        if self.parallel(other) {
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

impl<'a, P> From<&'a [P; 3]> for Plane
where
    P: Deref<Target = Point>,
{
    fn from(points: &'a [P; 3]) -> Self {
        let (a, b, c) = (&points[0], &points[1], &points[2]);
        let ab = b.as_vector() - a.as_vector();
        let ac = c.as_vector() - a.as_vector();
        let norm = (ab).cross(&ac);
        let dist = norm.dot(&a.as_vector());

        Plane { norm, dist }
    }
}

impl<P> From<(P, P, P)> for Plane
where
    P: Deref<Target = Point>,
{
    fn from(points: (P, P, P)) -> Self {
        Plane::from(&[points.0, points.1, points.2])
    }
}

impl Parallel for Plane {
    fn parallel(&self, other: &Plane) -> bool {
        self.norm
            .cross(&other.norm)
            .length_sq()
            .approximately(0.0, EPSILON)
    }
}

impl Parallel<Line> for Plane {
    fn parallel(&self, other: &Line) -> bool {
        self.norm
            .dot(&other.direction)
            .abs()
            .approximately(0.0, EPSILON)
    }
}

impl Intersects for Plane {
    type Intersection = Option<Line>;

    fn interesects(&self, other: &Self) -> bool {
        !self.parallel(other)
    }

    fn intersection(&self, other: &Self) -> Self::Intersection {
        if self.parallel(other) {
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
}

impl Intersects<Line> for Plane {
    type Intersection = Option<Point>;

    fn interesects(&self, other: &Line) -> bool {
        !self.parallel(other)
    }

    fn intersection(&self, other: &Line) -> Self::Intersection {
        let denom = self.norm.dot(&other.direction);
        if denom.abs().approximately(0.0, EPSILON) {
            None // Ray is parallel to the plane
        } else {
            let t = (self.dist - self.norm.dot(&other.origin.as_vector())) / denom;
            (t >= Float::ZERO).then_some(other.origin + (other.direction.normalized() * t))
        }
    }
}

#[cfg(test)]
mod plane_tests {
    use crate::vec::X_AXIS;
    use crate::vec::Y_AXIS;
    use crate::vec::Z_AXIS;

    use super::*;

    fn approx_eq(a: Float, b: Float) -> bool {
        a.approximately(b, EPSILON)
    }

    #[test]
    fn test_line_intersection_xy_xz() {
        let plane_xy = PLANE_XY;
        let plane_xz = PLANE_XZ;
        let expected = X_AXIS;
        let intersect = plane_xy
            .intersection(&plane_xz)
            .expect("Expected an intercept between XY and XZ planes");

        assert!(plane_xy.interesects(&plane_xz));

        // The intersection point should lie on both planes.
        assert!(
            approx_eq(plane_xy.distance_to(intersect.origin), Float::ZERO),
            "Intersection point is not on PLANE_XY."
        );
        assert!(
            approx_eq(plane_xz.distance_to(intersect.origin), Float::ZERO),
            "Intersection point is not on PLANE_XZ."
        );

        assert!(
            &intersect.direction.parallel(&expected),
            "Intersection direction is not parallel to the x-axis."
        );
    }

    #[test]
    fn test_line_intersection_xy_rotated() {
        let plane_xy = PLANE_XY;
        let normal_rotated = Vector::new(0.70710678, 0.0, 0.70710678);
        let plane_rotated = Plane::new(normal_rotated, Float::ZERO);
        let intersect = plane_xy
            .intersection(&plane_rotated)
            .expect("Expected an intersect between XY and rotated plane");
        let expected = Y_AXIS;

        assert!(plane_xy.interesects(&plane_rotated));

        // The intersection point must lie on both planes.
        assert!(
            approx_eq(plane_xy.distance_to(intersect.origin), Float::ZERO),
            "Intersection point is not on the XY plane."
        );
        assert!(
            approx_eq(plane_rotated.distance_to(intersect.origin), Float::ZERO),
            "Intersection point is not on the rotated plane."
        );

        assert!(
            &intersect.direction.parallel(&expected),
            "Intersection direction is not parallel to the y-axis."
        );
    }

    #[test]
    fn test_parallel_planes() {
        // Create two parallel planes: the XY plane and a shifted XY plane (z = 1).
        let plane1 = Plane::new(Z_AXIS, Float::ZERO);
        let plane2 = Plane::new(Z_AXIS, Float::ONE);
        let intersect = plane1.intersection(&plane2);

        assert!(!plane1.interesects(&plane2));
        assert!(
            intersect.is_none(),
            "Expected no intersection line between parallel planes."
        );
    }

    #[test]
    fn test_inverted_planes_intersection() {
        // Inverting a plane should not change the intersection line.
        let plane1 = PLANE_XY;
        let mut plane2 = PLANE_XZ;

        let intersect = plane1
            .intersection(&plane2)
            .expect("Expected an intersection between XY and XZ planes");

        plane2.invert();
        let intersect_inverted = plane1
            .intersection(&plane2)
            .expect("Expected an intersection between inverted planes");

        assert!(plane1.interesects(&plane2));

        // Both intersection points must lie on plane1.
        assert!(
            approx_eq(plane1.distance_to(intersect.origin), Float::ZERO),
            "Intersection point pt1 is not on PLANE_XY."
        );
        assert!(
            approx_eq(plane1.distance_to(intersect_inverted.origin), Float::ZERO),
            "Intersection point pt2 is not on PLANE_XY."
        );

        // The directions returned should be parallel (or anti‐parallel) even if one plane was inverted.
        assert!(
            &intersect.direction.parallel(&intersect_inverted.direction),
            "Intersection directions differ between the original and inverted plane cases."
        );
    }
}
