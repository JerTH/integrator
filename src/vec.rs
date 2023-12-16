use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, DivAssign, Div};

use crate::FType as Float;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Vector {
    x: Float,
    y: Float,
    z: Float,
}

impl Vector {
    /// Create a new [Vector] from x, y, and z components
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self {
            x, y, z
        }
    }
    
    /// Calculate the dot product of this and `rhs`
    /// X = V.V_1
    pub fn dot(&self, rhs: &Self) -> Float {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }

    /// Calculate the length of the vector
    /// L = |V|
    pub fn length(&self) -> Float {
        Float::sqrt(&self.x * &self.x + &self.y * &self.y + &self.z * &self.z)
    }

    /// Calculate a normalized copy of the vector
    /// V = V/|V|
    pub fn normalized(&self) -> Self {
        let len = self.length();
        Vector::new(self.x / len, self.y / len, self.z / len)
    }
}

impl From<Float> for Vector {
    fn from(value: Float) -> Self {
        Vector {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl From<(Float, Float, Float)> for Vector {
    fn from(value: (Float, Float, Float)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Mul for &Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<Float> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Float) -> Self::Output {
        &self * &Vector::from(rhs)
    }
}

impl Div for &Vector {
    type Output = Vector;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div for Vector {
    type Output = Vector;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl Div<Float> for Vector {
    type Output = Vector;

    fn div(self, rhs: Float) -> Self::Output {
        &self / &Vector::from(rhs)
    }
}


impl AddAssign<&Self> for Vector {
    fn add_assign(&mut self, rhs: &Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl SubAssign<&Self> for Vector {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs)
    }
}

impl MulAssign<&Self> for Vector {
    fn mul_assign(&mut self, rhs: &Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(&rhs)
    }
}

impl DivAssign<&Self> for Vector {
    fn div_assign(&mut self, rhs: &Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(&rhs)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Point {
    v: Vector,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point::from(Vector::new(x, y, z))
    }

    #[inline(always)]
    pub fn as_vector(&self) -> &Vector {
        &self.v
    }

    pub fn distance_to(&self, rhs: &Self) -> f64 {
        let delta = rhs.as_vector() - self.as_vector();
        delta.length()
    }
}

impl From<Vector> for Point {
    fn from(value: Vector) -> Self {
        Point { v: value }
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.v + rhs)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        &self + &rhs
    }
}

impl Add<&Vector> for Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        &self + rhs
    }
}

impl Add<Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        self + &rhs
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.v - rhs)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        &self - rhs
    }
}

impl Sub<Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        self - &rhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        const PRECISION: f64 = 1_000_000_000_000i64 as f64;

        let v = Vector::new(1.0, 2.0, 3.0);
        let p = Point::new(5.0, 5.0, 5.0);

        assert_eq!(Vector::new(2.0, 4.0, 6.0), &v + &v);
        assert_eq!(Point::new(6.0, 7.0, 8.0), &p + &v);

        let test_length = (3.7416573867739413 * PRECISION).round() / PRECISION;
        let real_length = (v.length() * PRECISION).round() / PRECISION;
        assert_eq!(test_length, real_length);

        let p2 = Point::new(3.0, 1.0, 4.0);
        let test_distance = (4.58257569495584 * PRECISION).round() / PRECISION;
        let real_distance = (p.distance_to(&p2) * PRECISION).round() / PRECISION;
        assert_eq!(test_distance, real_distance);
    }
}
