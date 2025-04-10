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
    type Intersection = Circle;

    fn interesects(&self, other: &Self) -> bool {
        self.distance_to_sq(other) < 0.0
    }

    fn intersection(&self, _: &Self) -> Self::Intersection {
        todo!()
    }
}
