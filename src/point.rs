//!
//! Points in 3D space
//! 

use serde::{Serialize, Deserialize};

use crate::{Float, Vector};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    v: Vector,
}

impl Point {
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            v: Vector::new(x, y, z),
        }
    }

    #[inline]
    pub const fn origin() -> Self {
        Self {
            v: Vector::zero()
        }
    }
    
    #[inline(always)]
    pub fn as_vector(&self) -> &Vector {
        &self.v
    }

    /// Calculates the distance from this point to `rhs`
    /// X = |V_1 - V|
    pub fn distance_to(&self, rhs: &Self) -> f64 {
        let delta = rhs.as_vector() - self.as_vector();
        delta.length()
    }

    /// Calculates the squared distance from this point to `rhs`
    /// Can be faster than `Point::distance_to()`
    pub fn distance_to_sq(&self, rhs: &Self) -> f64 {
        let delta = rhs.as_vector() - self.as_vector();
        delta.length_sq()
    }

    /// Returns a new [Point] with each component snapped to the nearest
    /// multiple of the corresponding component of `step`
    pub fn snapped<V>(&self, step: V) -> Self
    where
        V: Into<Vector>,
    {
        let step_vector: Vector = step.into();
        Point::from(Vector {
            x: Float::round(self.v.x / step_vector.x) * step_vector.x,
            y: Float::round(self.v.y / step_vector.y) * step_vector.y,
            z: Float::round(self.v.z / step_vector.z) * step_vector.z,
        })
    }
}

impl From<Vector> for Point {
    fn from(value: Vector) -> Self {
        Point { v: value }
    }
}

impl std::ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.v + rhs)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::Add<&Vector> for Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.v - rhs)
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Sub<&Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        &self - rhs
    }
}

impl std::ops::Sub<Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        self - &rhs
    }
}

impl std::ops::AddAssign<&Vector> for Point {
    fn add_assign(&mut self, rhs: &Vector) {
        self.v.x = self.v.x + rhs.x;
        self.v.y = self.v.y + rhs.y;
        self.v.z = self.v.z + rhs.z;
    }
}

impl std::ops::AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.add_assign(&rhs)
    }
}

impl std::ops::SubAssign<&Vector> for Point {
    fn sub_assign(&mut self, rhs: &Vector) {
        self.v.x = self.v.x - rhs.x;
        self.v.y = self.v.y - rhs.y;
        self.v.z = self.v.z - rhs.z;
    }
}

impl std::ops::SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.sub_assign(&rhs)
    }
}
