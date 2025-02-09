use crate::{segment::LineSegment, traits::{Coincident, Distance, Parallel}, Approximately, Float, Point, Vector, EPSILON};

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
}

impl From<LineSegment> for Line {
    fn from(segment: LineSegment) -> Self {
        Self::new(segment.start, segment.end.as_vector())
    }
}

impl Parallel for Line {
    fn parallel(&self, other: &Self) -> bool {
        self.direction.parallel(&other.direction)

    }
}

impl Coincident for Line {
    fn coincident(&self, other: &Self) -> bool {
        let ab = self.direction - self.origin.as_vector();
        let ac = other.origin.as_vector() - self.origin.as_vector();
        let ad = other.direction - self.origin.as_vector();
        let ab_ac = ab.cross(&ac);
        let ab_ad = ab.cross(&ad);
        ab_ac.approximately(ab_ad, EPSILON)
    }
}

impl Distance<Point> for &Line {
    // |BC| = |AB x v| / |v|
    fn distance_to(&self, other: &Point) -> Float {
        (self.origin - other).cross(&self.direction).length() / self.direction.length()
    }
}

#[cfg(test)]
mod line_tests {
    use super::*;

    #[test]
    fn lines_parallel() {
        let a = Line::new(Point::origin(), Vector::up());
        let b = Line::new(Point::origin() + Vector::new(1.0, 0.0, 0.0), Vector::up());

        assert!(a.parallel(&b));
        assert!(!a.coincident(&b));
    }
}
