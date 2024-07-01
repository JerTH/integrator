use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::{ Vector, Point };
use crate::Float;

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
        self.norm.dot(point.as_vector()) * self.dist
    }

    pub fn point_on_positive_half(&self, point: Point) -> bool {
        self.distance_to(point).signum() > 0.0
    }
}

impl<'a, P> From<&'a [P; 3]> for Plane where P: Deref<Target = Point> {
    fn from(points: &'a [P; 3]) -> Self {
        let (a, b, c) = (&points[0], &points[1], &points[2]);
        let ab = b.as_vector() - a.as_vector();
        let ac = c.as_vector() - a.as_vector();
        let norm = (ab).cross(&ac);
        let dist = norm.dot(a.as_vector());

        Plane { norm, dist }
    }
}

impl<P> From<(P, P, P)> for Plane where P: Deref<Target = Point> {
    fn from(points: (P, P, P)) -> Self {
        Plane::from(&[points.0, points.1, points.2])
    }
}
