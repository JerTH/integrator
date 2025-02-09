use crate::{line::Line, Float};

pub trait Coincident<Rhs = Self> {
    fn coincident(&self, other: Rhs) -> bool;
}

pub trait Parallel<Rhs = Self> {
    fn parallel(self, other: Rhs) -> bool;
}

pub trait Distance<Rhs = Self> {
    fn distance_to(self, other: Rhs) -> Float;
}

pub trait RayIntersection {
    fn ray_intersection(&self, ray: Line) -> Option<Float>;
}

pub trait Intersects<Rhs = Self> {
    /// The resulting intersection shape
    /// For two planes, this would be an Option<Line>
    type Intersection;

    /// Test whether two items intersect, without finding the intersection 
    fn interesects(&self, other: &Rhs) -> bool;
    
    /// Compute the resulting intersection of two items, if there is one
    fn intersection(&self, other: &Rhs) -> Self::Intersection;
}
