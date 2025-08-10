//!
//! Spheres in 3D space
//!

use std::ops::Deref;

use crate::circle::Circle;
use crate::line::Line;
use crate::traits::Distance;
use crate::Float;
use crate::Intersects;
use crate::Point;

pub struct Sphere {
    pub center: Point,
    pub radius: Float,
}

impl Sphere {
    pub fn new(center: Point, radius: Float) -> Self {
        Self { center, radius }
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.center.distance_to_sq(point) < (self.radius * self.radius)
    }

    pub fn minimum_bounding<P: Deref<Target = [Point]>>(_: P) -> Option<Self> {
        todo!("Not yet implemented: Turns out this is quite non-trivial - implementations are welcome")
    }
}

static MINIMUM_BOUNDING_SPHERE_EXPECTED_MESSAGE: &str =
    "expected valid point set when converting to bounding sphere. use member function instead to handle possible failure";

impl<P: Deref<Target = [Point]>> From<P> for Sphere {
    fn from(points: P) -> Self {
        debug_assert!(points.len() >= 4);
        Self::minimum_bounding(points).expect(MINIMUM_BOUNDING_SPHERE_EXPECTED_MESSAGE)
    }
}

impl Distance for Sphere {
    fn distance_to_sq(&self, other: &Self) -> Float {
        let center_distance_squared = self.center.distance_to_sq(&other.center);
        center_distance_squared - (self.radius * self.radius) - (other.radius * other.radius)
    }
}

impl Distance<Point> for Sphere {
    fn distance_to_sq(&self, other: &Point) -> Float {
        self.center.distance_to_sq(other) - (self.radius * self.radius)
    }
}

impl Distance<Line> for Sphere {
    fn distance_to_sq(&self, other: &Line) -> Float {
        other.distance_to_sq(&self.center) - (self.radius * self.radius)
    }
}

impl Intersects for Sphere {
    type Intersection = Option<Circle>;

    fn interesects(&self, other: &Self) -> bool {
        let d_sq = self.center.distance_to_sq(&other.center);
        let r_sum = self.radius + other.radius;
        let r_diff = (self.radius - other.radius).abs();
        let r_diff_sq = r_diff.powi(2);
        let r_sum_sq = r_sum.powi(2);
        d_sq <= r_sum_sq && d_sq >= r_diff_sq
    }

    fn intersection(&self, other: &Self) -> Self::Intersection {
        let direction = other.center - self.center;
        let d_sq = direction.length_sq();
        let d = d_sq.sqrt();

        // Check if spheres intersect
        let r_sum = self.radius + other.radius;
        let r_diff = (self.radius - other.radius).abs();
        if d > r_sum || d < r_diff {
            return None;
        }

        if d == 0.0 {
            if self.radius == other.radius {
                // Intersection is the entire sphere, not representable as a circle
                None
            } else {
                // One is entirely inside the other, no intersection circle
                None
            }
        } else {
            let h = (d_sq + self.radius.powi(2) - other.radius.powi(2)) / (2.0 * d);
            let intersection_radius = (self.radius.powi(2) - h.powi(2)).sqrt();
            let center = self.center + (direction * (h / d));
            let normal = direction.normalized();

            Some(Circle::new(center, normal, intersection_radius))
        }
    }
}
