use crate::Point;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct LineSegment {
    pub a: Point,
    pub b: Point,
}

impl LineSegment {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            a,
            b,
        }
    }
}

