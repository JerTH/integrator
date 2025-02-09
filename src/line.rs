use crate::{segment::LineSegment, traits::{Distance, Parallel}, Approximately, Point, Vector, EPSILON};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Line {
    pub origin: Point, // A
    pub direction: Vector, // B
}

impl Line {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn coincident(&self, other: &Self) -> bool {
        let ab = self.direction - self.origin.as_vector();
        let ac = other.origin.as_vector() - self.origin.as_vector();
        let ad = other.direction - self.origin.as_vector();
        let ab_ac = ab.cross(&ac);
        let ab_ad = ab.cross(&ad);
        ab_ac.approximately(ab_ad, EPSILON)
    }
}

impl From<LineSegment> for Line {
    fn from(segment: LineSegment) -> Self {
        Self::new(segment.start, segment.end.as_vector())
    }
}

impl Parallel<&Line> for &Line {
    fn parallel(self, other: &Line) -> bool {
        self.direction.parallel(&other.direction)

    }
}

impl Distance<&Point> for &Line {
    // |BC| = |AB x v| / |v|
    fn distance_to(self, other: &Point) -> crate::Float {
        (self.origin - other).cross(&self.direction).length() / self.direction.length()
    }
}
