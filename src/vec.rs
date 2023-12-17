use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, DivAssign, Div, Deref};
use crate::FType as Float;

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector {
    x: Float,
    y: Float,
    z: Float,
}

impl Vector {
    /// Create a new [Vector] from x, y, and z components
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
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

    /// Compute the cross product of this and `rhs`
    pub fn cross<V>(&self, rhs: V) -> Self where V: Deref<Target = Self> {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    /// Calculate the length of the [Vector]
    /// L = |V|
    pub fn length(&self) -> Float {
        Float::sqrt(self.length_sq())
    }

    /// Calculate the squared length of the [Vector]
    /// Faster than Vector::length()
    pub fn length_sq(&self) -> Float {
        &self.x * &self.x + &self.y * &self.y + &self.z * &self.z
    }

    /// Computes a new [Vector] preserving this vectors direction, with
    /// its length limited to `length`
    pub fn limit_length(&self, length: Float) -> Self {
        if self.length_sq() > (length * length) {
            let normalized = self.normalized();
            normalized * length
        } else {
            self.clone()
        }
    }

    /// Calculates the resulting [Vector] from the linear interpolation
    /// of `a` to `b`, by the amount of `weight`
    /// # Examples
    /// 
    /// ```
    /// use integrator::Vector;
    /// 
    /// let a = Vector::new(1.0, 1.0, 1.0);
    /// let b = Vector::new(-1.0, -1.0, -1.0);
    /// 
    /// assert_eq!(Vector::new(0.5, 0.5, 0.5), a.lerp(&b, 0.25));
    /// assert_eq!(Vector::new(0.0, 0.0, 0.0), a.lerp(&b, 0.5));
    /// assert_eq!(Vector::new(-0.5, -0.5, -0.5), a.lerp(&b, 0.75));
    /// ```
    pub fn lerp(&self, to: &Self, weight: Float) -> Self {
        // a + (b - a) * t 
        Self {
            x: self.x + (to.x - self.x) * weight,
            y: self.y + (to.y - self.y) * weight,
            z: self.z + (to.z - self.z) * weight,
        }
    }

    /// Calculate a normalized copy of the [Vector]
    /// V = V/|V|
    pub fn normalized(&self) -> Self {
        let len = self.length();
        Vector::new(self.x / len, self.y / len, self.z / len)
    }

    /// Computes a new [Vector] with components clamped between the components
    /// of `min` and `max`
    pub fn clamp<V>(&self, min: V, max: V) -> Self where V: Deref<Target = Self> {
        Self {
            x: Float::clamp(self.x, min.x, max.x),
            y: Float::clamp(self.y, min.y, max.y),
            z: Float::clamp(self.z, min.z, max.z),
        }
    }

    /// Returns a new [Vector] with each component set to either 1.0 or -1.0,
    /// corresponding to the sign of each component of `self` 
    pub fn sign(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
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

impl Mul<Float> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: Float) -> Self::Output {
        self * &Vector::from(rhs)
    }
}

impl Mul<Float> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Float) -> Self::Output {
        &self * rhs
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

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    v: Vector,
}

impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Point { v: Vector::new(x, y, z) }
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
    pub fn snapped<V>(&self, step: V) -> Self where V: Into<Vector> {
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
