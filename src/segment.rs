//! Line Segment

use crate::{line::Line, Float, Point};

/// A Line Segment
/// 
/// Nearly identical to a [crate::line::Line], the difference being how they are used.
/// 
/// Where a [crate::line::Line] is canonically defined by an origin point and a direction
/// vector where the origin is any point along the line and the line extends to infinity,
/// a line segment is defined by start and end points and has a length. [LineSegment]'s
/// can be converted to and from [crate::line::Line]'s seamlessly.
/// 
/// [LineSegment]'s and [crate::line::Line]'s are similar to the distinction between
/// [crate::point::Point] and [crate::vec::Vector] where the two types are nearly
/// functionally identical but differentiating the two is still useful
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

