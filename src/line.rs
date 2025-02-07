use crate::{traits::Parallel, Approximately, Point, Vector, EPSILON};

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

    pub fn from_endpoints(start: Point, end: Point) -> Self {
        Self::new(start, end.as_vector())
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

impl Parallel<&Line> for &Line {
    fn parallel(self, other: &Line) -> bool {
        self.direction.parallel(&other.direction)

    }
}

impl Parallel<Line> for &Line {
    fn parallel(self, other: Line) -> bool {
        self.parallel(&other)
    }
}

impl Parallel<&Line> for Line {
    fn parallel(self, other: &Line) -> bool {
        (&self).parallel(other)
    }
}

impl Parallel for Line {
    fn parallel(self, other: Self) -> bool {
        (&self).parallel(&other)
    }
}
