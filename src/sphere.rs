//! 3D Spheres

use std::ops::Deref;

use crate::{line::Line, traits::Distance, Float, Point};

pub struct Sphere {
    pub center: Point,
    pub radius: Float,
}

impl Sphere {
    pub fn new(center: Point, radius: Float) -> Self {
        Self { center, radius }
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.center.distance_to(&point) < self.radius
    }

    pub fn minimum_bounding<P: Deref<Target = [Point]>>(points: P) -> Option<Self> {
        if points.len() < 4 {
            return None;
        }

        todo!()
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

impl Distance<&Sphere> for Sphere {
    fn distance_to(self, other: &Self) -> Float {
        let center_distance = self.center.distance_to(&other.center);
        center_distance - self.radius - other.radius
    }
}

impl Distance<&Point> for Sphere {
    fn distance_to(self, other: &Point) -> Float {
        self.center.distance_to(&other) - self.radius
    }
}

impl Distance<&Line> for Sphere {
    fn distance_to(self, other: &Line) -> Float {
        other.distance_to(&self.center) - self.radius
    }
}
