//!
//! Vectors in 3D space
//!

use crate::{bivec::Bivector, matrix::Matrix, rotor::Rotor, Approximately, Float, EPSILON};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const X_AXIS: Vector = Vector { x: 1.0, y: 0.0, z: 0.0 };
pub const Y_AXIS: Vector = Vector { x: 0.0, y: 1.0, z: 0.0 };
pub const Z_AXIS: Vector = Vector { x: 0.0, y: 0.0, z: 1.0 };

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector {
    /// Create a new [Vector] from x, y, and z components
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_x() -> Self {
        X_AXIS
    }

    pub fn unit_y() -> Self {
        Y_AXIS
    }

    pub fn unit_z() -> Self {
        Z_AXIS
    }

    /// Constructs a new unit [Vector] pointing in the canonical up direction
    ///
    /// (0.0, 1.0, 0.0)
    ///
    pub fn up() -> Self {
        Self::unit_y()
    }

    /// Constructs a new unit [Vector] pointing in the canonical down direction
    ///
    /// (0.0, -1.0, 0.0)
    ///
    pub fn down() -> Self {
        -Self::unit_y()
    }

    /// Constructs a new unit [Vector] pointing in the canonical forward direction
    ///
    /// (0.0, 0.0, 1.0)
    ///
    pub fn forward() -> Self {
        Self::unit_z()
    }

    /// Constructs a new unit [Vector] pointing in the canonical backward direction
    ///
    /// (0.0, 0.0, -1.0)
    ///
    pub fn backward() -> Self {
        -Self::unit_z()
    }

    #[inline(always)]
    pub fn w(&self) -> Float {
        1.0
    }

    /// Calculate the dot product of this and `rhs`
    /// X = V.V_1
    pub fn dot(&self, rhs: &Self) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Compute the cross product of this and `rhs`
    pub fn cross<V>(&self, rhs: V) -> Self
    where
        V: Deref<Target = Self>,
    {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    #[inline]
    pub fn wedge<V>(self, v: V) -> Bivector
    where
        V: Into<Vector>,
    {
        let v: Vector = v.into();
        Bivector {
            xy: self.x * v.y - self.y * v.x,
            xz: self.x * v.z - self.z * v.x,
            yz: self.y * v.z - self.z * v.y,
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
    /// ```
    /// use integrator::vec::Vector;
    /// let vector = Vector::new(10.8, 5.4, 10.8);
    /// let limited = vector.limit_length(3.0);
    /// assert_eq!(Vector::new(2.0, 1.0, 2.0), limited);
    /// ```
    pub fn limit_length(&self, length: Float) -> Self {
        if self.length_sq() > (length * length) {
            let normalized = self.normalized();
            normalized * length
        } else {
            self.clone()
        }
    }

    pub fn distance_to(&self, other: &Self) -> Float {
        (other - self).length()
    }

    pub fn parallel_to(&self, other: &Self) -> bool {
        1.0.approximately(self.normalized().dot(&other.normalized()).abs(), EPSILON)
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
    pub fn clamp<V>(&self, min: V, max: V) -> Self
    where
        V: Deref<Target = Self>,
    {
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

    pub fn rotated_by(&self, rotation: &Rotor) -> Self {
        let mut rotated = *self;
        rotation.rotate_vector(&mut rotated);
        rotated
    }

    pub fn rotate(&mut self, rotation: &Rotor) {
        rotation.rotate_vector(self)
    }

    pub fn rotate_about_x(&self, radians: Float) -> Self {
        Vector {
            x: self.x,
            y: (self.y * radians.cos()) - (self.z * radians.sin()),
            z: (self.y * radians.sin()) + (self.z * radians.cos()),
        }
    }

    pub fn rotate_about_y(&self, radians: Float) -> Self {
        Vector {
            x: (self.x * radians.cos()) + (self.z * radians.sin()),
            y: self.y,
            z: (-self.x * radians.sin()) + (self.z * radians.cos()),
        }
    }

    pub fn rotate_about_z(&self, radians: Float) -> Self {
        Vector {
            x: (self.x * radians.cos()) - (self.y * radians.sin()),
            y: (self.x * radians.sin()) + (self.y * radians.cos()),
            z: self.z,
        }
    }
    
    /// Returns a new [Vector] with its X and Y components rotated 90 degrees clockwise about the Z axis
    /// ```
    /// use integrator::vec::Vector;
    /// let before = Vector::new(1.0, 0.0, 0.0);
    /// let rotated = before.rotate_90_xy_cw();
    /// assert_eq!(Vector::new(0.0, -1.0, 0.0), rotated);
    /// assert_eq!(Vector::new(-1.0, 0.0, 0.0), rotated.rotate_90_xy_cw());
    /// ```
    pub fn rotate_90_xy_cw(&self) -> Self {
        Vector {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    /// Returns a new [Vector] with its X and Y components rotated 90 degrees counter-clockwise about the Z axis
    /// ```
    /// use integrator::vec::Vector;
    /// let before = Vector::new(1.0, 0.0, 0.0);
    /// let rotated = before.rotate_90_xy_ccw();
    /// assert_eq!(Vector::new(0.0, 1.0, 0.0), rotated);
    /// ```
    pub fn rotate_90_xy_ccw(&self) -> Self {
        Vector {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }
}

impl Approximately for Vector {
    fn approximately(&self, other: Self, epsilon: Float) -> bool {
        self.x.approximately(other.x, epsilon)
            && self.y.approximately(other.y, epsilon)
            && self.z.approximately(other.z, epsilon)
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

macro_rules! vector_mul {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Vector;
            fn mul(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                }
            }
        }
    };
}

vector_mul!(Vector, Float);
vector_mul!(&Vector, Float);
vector_mul!(Vector, &Float);
vector_mul!(&Vector, &Float);
vector_mul!(&mut Vector, Float);

macro_rules! vector_mul_reversed {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Vector;
            fn mul(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                }
            }
        }
    };
}

vector_mul_reversed!(Float, Vector);
vector_mul_reversed!(&Float, Vector);
vector_mul_reversed!(Float, &Vector);
vector_mul_reversed!(&Float, &Vector);
vector_mul_reversed!(Float, &mut Vector);

macro_rules! vector_componentwise_binop {
    ($lhs:ty, $rhs:ty, $func:ident, $trait:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = Vector;
            fn $func(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: Float::$func(self.x, other.x),
                    y: Float::$func(self.y, other.y),
                    z: Float::$func(self.z, other.z),
                }
            }
        }
    };
}

vector_componentwise_binop!(Vector, Vector, mul, Mul);
vector_componentwise_binop!(&Vector, Vector, mul, Mul);
vector_componentwise_binop!(Vector, &Vector, mul, Mul);
vector_componentwise_binop!(&Vector, &Vector, mul, Mul);

vector_componentwise_binop!(Vector, Vector, div, Div);
vector_componentwise_binop!(&Vector, Vector, div, Div);
vector_componentwise_binop!(Vector, &Vector, div, Div);
vector_componentwise_binop!(&Vector, &Vector, div, Div);

vector_componentwise_binop!(Vector, Vector, sub, Sub);
vector_componentwise_binop!(&Vector, Vector, sub, Sub);
vector_componentwise_binop!(Vector, &Vector, sub, Sub);
vector_componentwise_binop!(&Vector, &Vector, sub, Sub);

vector_componentwise_binop!(Vector, Vector, add, Add);
vector_componentwise_binop!(&Vector, Vector, add, Add);
vector_componentwise_binop!(Vector, &Vector, add, Add);
vector_componentwise_binop!(&Vector, &Vector, add, Add);

macro_rules! vector_div {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = Vector;
            fn div(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                }
            }
        }
    };
}

vector_div!(Vector, Float);
vector_div!(&Vector, Float);
vector_div!(Vector, &Float);
vector_div!(&Vector, &Float);

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

impl MulAssign<Float> for Vector {
    fn mul_assign(&mut self, rhs: Float) {
        *self = *self * rhs
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

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:+.3}, {:+.3}, {:+.3})", self.x, self.y, self.z)
    }
}

impl Mul<&Matrix> for &Vector {
    type Output = Vector;

    /// Multiply a [Vector] by a [Matrix] (p' = pM)
    fn mul(self, rhs: &Matrix) -> Self::Output {
        let lhs = self;
        let w = 0.0;

        // Here W is 0.0, we could erase the last term (it might be optimized out anyway)
        Vector {
            x: lhs.x * rhs[0][0] + lhs.y * rhs[1][0] + lhs.z * rhs[2][0] + w * rhs[3][0],
            y: lhs.x * rhs[0][1] + lhs.y * rhs[1][1] + lhs.z * rhs[2][1] + w * rhs[3][1],
            z: lhs.x * rhs[0][2] + lhs.y * rhs[1][2] + lhs.z * rhs[2][2] + w * rhs[3][2],
        }
    }
}

impl Mul<&Matrix> for Vector {
    type Output = Vector;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Matrix> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Matrix> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Matrix) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod vec_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_addition() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, Vector::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_subtraction() {
        let v1 = Vector::new(5.0, 5.0, 5.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1 - v2, Vector::new(3.0, 2.0, 1.0));
    }

    #[test]
    fn test_dot_product_perpendicular() {
        let v1 = Vector::unit_x();
        let v2 = Vector::unit_y();
        assert_eq!(v1.dot(&v2), 0.0);
    }

    #[test]
    fn test_cross_product() {
        let cross = Vector::unit_x().cross(&Vector::unit_y());
        assert_eq!(cross, Vector::unit_z());
    }

    #[test]
    fn test_length() {
        let v = Vector::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_normalization() {
        let v = Vector::new(3.0, 4.0, 0.0).normalized();
        assert!(v.length().approximately(1.0, 1e-6));
    }

    #[test]
    fn test_lerp() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        let lerped = a.lerp(&b, 0.5);
        assert_eq!(lerped, Vector::new(1.5, 2.5, 3.5));
    }

    #[test]
    fn test_clamp() {
        let v = Vector::new(5.0, -2.0, 10.0);
        let min = Vector::new(0.0, -1.0, 5.0);
        let max = Vector::new(4.0, 0.0, 8.0);
        let clamped = v.clamp(&min, &max);
        assert_eq!(clamped, Vector::new(4.0, -1.0, 8.0));
    }

    #[test]
    fn test_sign() {
        let v = Vector::new(-3.0, 4.0, 0.0);
        dbg!(v.sign());
        assert_eq!(v.sign(), Vector::new(-1.0, 1.0, 1.0));
    }

    #[test]
    fn test_rotation_about_z() {
        let v = Vector::unit_x();
        let rotated = v.rotate_about_z(PI / 2.0);
        assert!(rotated.approximately(Vector::unit_y(), 1e-6));
    }

    #[test]
    fn test_wedge_product() {
        let v1 = Vector::unit_x();
        let v2 = Vector::unit_y();
        let bivector = v1.wedge(v2);
        assert_eq!(bivector.xy, 1.0);
        assert_eq!(bivector.xz, 0.0);
        assert_eq!(bivector.yz, 0.0);
    }

    #[test]
    fn test_negation() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_cross_product_zero_vector() {
        let v1 = Vector::zero();
        let v2 = Vector::unit_x();
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vector::zero());
    }

    #[test]
    fn test_distance_to() {
        let v1 = Vector::new(0.0, 0.0, 0.0);
        let v2 = Vector::new(3.0, 4.0, 0.0);
        assert_eq!(v1.distance_to(&v2), 5.0);
    }

    #[test]
    fn test_assignment_operators() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        v += Vector::new(0.5, 1.0, 1.5);
        assert_eq!(v, Vector::new(1.5, 3.0, 4.5));
        v *= 2.0;
        assert_eq!(v, Vector::new(3.0, 6.0, 9.0));
        v -= Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vector::new(2.0, 4.0, 6.0));
        v /= Vector::new(2.0, 2.0, 2.0);
        assert_eq!(v, Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_componentwise_operations() {
        let a = Vector::new(2.0, 3.0, 4.0);
        let b = Vector::new(1.0, 2.0, 0.5);
        assert_eq!(a * b, Vector::new(2.0, 6.0, 2.0));
        assert_eq!(a / b, Vector::new(2.0, 1.5, 8.0));
    }

    #[test]
    fn test_rotate_90_xy() {
        let v = Vector::unit_x();
        let rotated_cw = v.rotate_90_xy_cw();
        assert_eq!(rotated_cw, Vector::new(0.0, -1.0, 0.0));
        let rotated_ccw = v.rotate_90_xy_ccw();
        assert_eq!(rotated_ccw, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_limit_length() {
        let v = Vector::new(6.0, 8.0, 0.0);
        let limited = v.limit_length(5.0);
        let expected = Vector::new(3.0, 4.0, 0.0);
        assert_eq!(limited, expected);
    }

    #[test]
    fn test_rotation_about_x() {
        let v = Vector::unit_y();
        let rotated = v.rotate_about_x(PI / 2.0);
        dbg!(rotated);
        dbg!(Vector::unit_z());
        assert!(rotated.approximately(Vector::unit_z(), 1e-6));
    }

    #[test]
    fn test_rotation_about_y() {
        let v = Vector::unit_z();
        let rotated = v.rotate_about_y(PI / 2.0);
        assert!(rotated.approximately(Vector::unit_x(), 1e-6));
    }

    #[test]
    fn test_from_tuple() {
        let tup = (1.0, 2.0, 3.0);
        let v: Vector = tup.into();
        assert_eq!(v, Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_approximately() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0 + 1e-7, 2.0 - 1e-7, 3.0 + 1e-7);
        assert!(v1.approximately(v2, 1e-6));
    }

    #[test]
    fn test_vector_mul() {
        let v1 = Vector::new(5.0, 3.0, 1.0);
        let mut v2 = Vector::new(2.0, 4.0, 6.0);
        let f1 = 3.0;

        let r1 = Vector::new(10.0, 12.0, 6.0);
        assert_eq!(r1, v1 * v2);
        assert_eq!(r1, &v1 * v2);
        assert_eq!(r1, v1 * &v2);
        assert_eq!(r1, &v1 * &v2);

        let r2 = Vector::new(15.0, 9.0, 3.0);
        assert_eq!(r2, v1 * f1);
        assert_eq!(r2, &v1 * f1);
        assert_eq!(r2, v1 * &f1);
        assert_eq!(r2, &v1 * &f1);

        let r3 = Vector::new(6.0, 12.0, 18.0);
        assert_eq!(r3, v2 * f1);
        assert_eq!(r3, &mut v2 * f1);
    }
}
