use crate::Point;


#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct LineSegment {
    a: Point,
    b: Point,
}

impl LineSegment {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            a,
            b,
        }
    }
}

