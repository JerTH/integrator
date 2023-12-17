use crate::{ Vector, Point };
use crate::FType as Float;

pub const PLANE_XY: Plane = Plane { norm: Vector::new(0.0, 0.0, 1.0), dist: 0.0 };
pub const PLANE_XZ: Plane = Plane { norm: Vector::new(0.0, 1.0, 0.0), dist: 0.0 };
pub const PLANE_YZ: Plane = Plane { norm: Vector::new(1.0, 0.0, 0.0), dist: 0.0 };

pub struct Plane {
    norm: Vector,
    dist: Float,
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
}

impl From<(Point, Point, Point)> for Plane {
    fn from(points: (Point, Point, Point)) -> Self {
        let (a, b, c) = (&points.0, &points.1, &points.2);
        //n = (p1 - p0).cross(p2 - p0);
        //n = normalize(n);
        //d = n.dot(p0);

        let ab = b.as_vector() - a.as_vector();
        let ac = c.as_vector() - a.as_vector();
        let norm = (ab).cross(&ac);
        let dist = norm.dot(a.as_vector());

        Plane { norm, dist }
    }
}
