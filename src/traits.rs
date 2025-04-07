use crate::Float;

pub trait FloatExt {
    const ONE: Self;
    const ZERO: Self;
    const EPSILON: Self;
}

pub trait FromLossy<U> {
    fn from_lossy(value: U) -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait Approximately<Rhs = Self> {
    fn approximately(&self, other: Rhs, epsilon: Float) -> bool;
}

pub trait Coincident<Rhs = Self> {
    fn coincident(&self, other: &Rhs) -> bool;
}

pub trait Distance<Rhs = Self> {
    /// Compute the squared distance between two items
    /// 
    /// It is usually enough to just implement this method and use the
    /// auto implementation of [Distance::distance_to]
    fn distance_to_sq(&self, other: &Rhs) -> Float;
    
    /// Compute the real distance between two items
    /// 
    /// Generally speaking this is more expensive than [Distance::distance_to_sq] and
    /// usually involves at least one square root operation
    /// 
    /// By default this first calls [Distance::distance_to_sq] and then finds the
    /// square root of the result
    fn distance_to(&self, other: &Rhs) -> Float {
        Float::sqrt(self.distance_to_sq(other))
    }
}

pub trait Parallel<Rhs = Self> {
    fn parallel(&self, other: &Rhs) -> bool;
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
