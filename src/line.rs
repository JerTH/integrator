use crate::{Approximately, Point, Vector, EPSILON};

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

    pub fn parallel_to(&self, other: &Self) -> bool {
        self.direction.parallel_to(&other.direction)
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
