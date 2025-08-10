use crate::Float;
use crate::Point;
use crate::Vector;

/// An oriented circle in 3D space
pub struct Circle {
    center: Point,
    frame: Vector,
}

impl Circle {
    pub fn new(center: Point, normal: Vector, radius: Float) -> Self {
        let frame = normal * radius;
        Self { center, frame }
    }

    pub fn radius(&self) -> Float {
        self.frame.length()
    }

    pub fn center(&self) -> Point {
        self.center
    }
}
