use crate::{ Vector, Point };
use crate::FType as Float;

pub struct Plane {
    norm: Vector,
    dist: Float,
}

impl Plane {
    pub fn new(normal: Vector, distance: Float) -> Self {
        Self {
            norm: normal,
            dist: distance,
        }
    }

    pub fn invert(&mut self) {
        *self = self.inverted();
    }

    pub fn inverted(&self) -> Self {
        Self {
            norm: &self.norm * -1.0,
            dist: &self.dist * -1.0,
        }
    }
    
    pub fn distance_to(&self, point: Point) -> Float {
        self.norm.dot(point.as_vector()) * self.dist
    }
}
