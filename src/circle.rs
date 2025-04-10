use crate::Float;
use crate::Point;
use crate::Vector;

/// An oriented circle in 3D space
pub struct Circle {
    center: Point,
    frame: Vector,
}

impl Circle {
    pub fn radius(&self) -> Float {
        self.frame.length()
    }
}
