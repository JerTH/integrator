//!
//! Infinite lines in 3D space
//! 

use crate::segment::LineSegment;
use crate::traits::Coincident;
use crate::traits::Distance;
use crate::traits::Parallel;
use crate::traits::Zero;
use crate::Approximately;
use crate::Float;
use crate::Point;
use crate::Vector;

#[cfg(feature = "fixed_precision")]
use crate::traits::Numeric;

use serde::Deserialize;
use serde::Serialize;

const EPSILON: Float = Float::EPSILON;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Line {
    pub origin: Point,     // A
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
        // Check if both lines are degenerate (zero direction vectors)
        let self_dir_zero = self.direction.approximately(Vector::zero(), EPSILON);
        let other_dir_zero = other.direction.approximately(Vector::zero(), EPSILON);

        if self_dir_zero && other_dir_zero {
            // Both lines are points; check if their origins are the same
            return self.origin.approximately(&other.origin, EPSILON);
        } else if self_dir_zero || other_dir_zero {
            // One is a line and the other is a point; can't be coincident
            return false;
        }

        // Check if the direction vectors are parallel
        if !self.direction.parallel(&other.direction) {
            return false;
        }

        // Calculate the vector between the origins of the two lines
        let ab = other.origin - self.origin;

        // If the origins are the same, the lines are coincident
        if ab.approximately(Vector::zero(), EPSILON) {
            return true;
        }

        // Check if the vector between origins is parallel to the direction of the first line
        ab.parallel(&self.direction)
    }
}

impl Distance<Point> for &Line {
    // |BC| = |AB x v| / |v|
    fn distance_to(&self, other: &Point) -> Float {
        Float::sqrt(self.distance_to_sq(other))
    }
    
    fn distance_to_sq(&self, other: &Point) -> Float {
        (self.origin - other)
            .cross(&self.direction)
            .length_sq() 
        / self.direction.length_sq()
    }
}

#[cfg(test)]
mod line_tests {
    use super::*;

    #[test]
    fn lines_parallel() {
        let a = Line::new(Point::origin(), Vector::up());
        let b = Line::new(Point::origin() + Vector::new(1.0, 0.0, 0.0), Vector::up());
        let c = Line::new(Point::origin(), Vector::forward());

        assert!(a.parallel(&b));
        assert!(!a.parallel(&c));
        assert!(!b.parallel(&c));
    }

    #[test]
    fn lines_coincident() {
        let a = Line::new(Point::origin(), Vector::up());
        let b = Line::new(Point::origin() + Vector::up(), Vector::up());
        let c = Line::new(Point::origin() + Vector::new(1.0, 0.0, 0.0), Vector::up());

        assert!(a.coincident(&b));
        assert!(!a.coincident(&c));
        assert!(!b.coincident(&c));
    }
}
