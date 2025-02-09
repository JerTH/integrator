//!
//! Points in 3D space
//!

use std::ops::Mul;

use serde::{Deserialize, Serialize};

use crate::{Approximately, Float, Matrix, Vector, Zero};

#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Point {
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline(always)]
    pub fn as_vector(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
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
            x: Float::round(self.x / step_vector.x) * step_vector.x,
            y: Float::round(self.y / step_vector.y) * step_vector.y,
            z: Float::round(self.z / step_vector.z) * step_vector.z,
        })
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Self::new(Float::zero(), Float::zero(), Float::zero())
    }
}

impl From<Vector> for Point {
    fn from(value: Vector) -> Self {
        Point::new(value.x, value.y, value.z)
    }
}

impl Approximately for Point {
    fn approximately(&self, other: Self, epsilon: Float) -> bool {
        self.x.approximately(other.x, epsilon)
            && self.y.approximately(other.y, epsilon)
            && self.z.approximately(other.z, epsilon)
    }
}

impl Approximately for &Point {
    fn approximately(&self, other: Self, epsilon: Float) -> bool {
        self.x.approximately(other.x, epsilon)
            && self.y.approximately(other.y, epsilon)
            && self.z.approximately(other.z, epsilon)
    }
}

impl std::ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.as_vector() + rhs)
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
        Point::from(&self.as_vector() - rhs)
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

impl std::ops::Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        self.as_vector() - rhs.as_vector()
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        self.as_vector() - rhs.as_vector()
    }
}

impl std::ops::Sub<Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        self.as_vector() - rhs.as_vector()
    }
}

impl std::ops::Sub<&Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        self.as_vector() - rhs.as_vector()
    }
}

impl std::ops::AddAssign<&Vector> for Point {
    fn add_assign(&mut self, rhs: &Vector) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl std::ops::AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.add_assign(&rhs)
    }
}

impl std::ops::SubAssign<&Vector> for Point {
    fn sub_assign(&mut self, rhs: &Vector) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl std::ops::SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.sub_assign(&rhs)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:+.3}, {:+.3}, {:+.3})", self.x, self.y, self.z)
    }
}

impl Mul<&Matrix> for &Point {
    type Output = Vector;

    /// Multiply a [Matrix] by a [Point] (p' = Mp)
    fn mul(self, rhs: &Matrix) -> Self::Output {
        let rhs = rhs;
        let lhs = self.as_vector();
        let w = 1.0;
        Vector {
            x: lhs.x * rhs[0][0] + lhs.y * rhs[0][1] + lhs.z * rhs[0][2] + w * rhs[0][3],
            y: lhs.x * rhs[1][0] + lhs.y * rhs[1][1] + lhs.z * rhs[1][2] + w * rhs[1][3],
            z: lhs.x * rhs[2][0] + lhs.y * rhs[2][1] + lhs.z * rhs[2][2] + w * rhs[2][3],
        }
    }
}
