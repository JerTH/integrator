//! Line Segment

use crate::line::Line;
use crate::Float;
use crate::Point;

/// A Line Segment
///
/// Nearly identical to a [Line], the difference laying how they are used.
///
/// Where a [Line] is canonically defined by an origin point and a direction
/// vector where the origin is any point along the line and the line extends to infinity,
/// a line segment is defined by start and end points and has a length. [LineSegment]'s
/// can be converted to and from [Line]'s seamlessly.
///
/// [LineSegment] and [Line] are in a sense similar to the distinction between
/// [Point] and [crate::vec::Vector] where the two types are nearly
/// functionally identical but differentiating between them is still useful
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> Float {
        (self.end - self.start).length()
    }
}

impl From<Line> for LineSegment {
    fn from(line: Line) -> Self {
        Self::new(line.origin, line.direction.into())
    }
}
