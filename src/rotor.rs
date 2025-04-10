//!
//! Rotor
//!

use crate::bivec::Bivector;
use crate::traits::FloatExt;
use crate::Approximately;
use crate::Float;
use crate::Vector;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Mul;

// the "wild rotations" you mention has a very simple solution employed by every engine I've worked
// with. Basically, you just constrain the real part to be positive which fixes your interpolation
// on one half of the Lie-manifold which ensures the arc taken is as short as possible.

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Rotor {
    pub b: Bivector,
    pub s: Float,
}

impl Default for Rotor {
    fn default() -> Self {
        Self::identity()
    }
}

impl Rotor {
    /// Constructs a new [Rotor] from a [Bivector] and scalar [Float]
    pub fn new(bivector: Bivector, scalar: Float) -> Self {
        Self {
            b: bivector,
            s: scalar,
        }
    }

    pub fn identity() -> Self {
        Self::new(Bivector::default(), Float::ONE)
    }

    /// Rotate a [Vector] by the rotation represented by this [Rotor]
    ///
    /// ```
    /// # use integrator::{ traits::{ Approximately, FloatExt }, Float, vec::Vector, rotor::Rotor, bivec::Bivector };
    /// let mut from = Vector::new(4.0, 5.0, 3.0).normalized();
    /// let to = Vector::new(2.0, 5.0, 2.0).normalized();
    /// let rotor = Rotor::from_rotation_between_vectors(from, to);
    /// rotor.rotate_vector(&mut from);
    /// assert!(to.approximately(from, Float::EPSILON));
    /// ```
    pub fn rotate_vector(&self, vector: &mut Vector) {
        #[cfg(not(feature = "fixed_precision"))]
        {
            floating::rotate_vector(self, vector);
        }
        #[cfg(feature = "fixed_precision")]
        {
            fixed::rotate_vector(&self, vector);
        }
    }

    /// Rotate this [Rotor] by another [Rotor]
    pub fn rotate(&mut self, other: &Rotor) {
        *self = (*self) * *other * (self.reversed())
    }

    /// Return a new [Rotor] rotated by another [Rotor]
    pub fn rotated(&self, other: &Rotor) -> Self {
        let mut rotated = *self;
        rotated.rotate(other);
        rotated
    }

    /// Returns a new `Rotor` that rotates one unit vector to another unit vector
    #[inline]
    pub fn from_rotation_between_vectors(from: Vector, to: Vector) -> Self {
        let to = to.normalized();
        let from = from.normalized();

        let (b, s) = if from == -to {
            (
                Bivector::from_axis_vector(from.orthogonal().normalized()),
                Float::ZERO,
            )
        } else {
            (
                Bivector::from_wedge(to, from),
                Float::ONE + Vector::dot(&to, &from),
            )
        };

        Rotor::new(b, s).normalized()
    }

    /// Returns a new `Rotor` from an angle and a plane, the plane must be normalized
    #[inline]
    pub fn from_angle_and_plane<F: Into<Float>>(angle: F, plane: Bivector) -> Self {
        #[cfg(not(feature = "fixed_precision"))]
        {
            let angle = angle.into();

            let sina = (angle / Float::from(2.0)).sin();
            let cosa = (angle / Float::from(2.0)).cos();
            let bv = Bivector {
                xy: -sina * plane.xy,
                xz: -sina * plane.xz,
                yz: -sina * plane.yz,
            };
            Rotor::new(bv, cosa).normalized()
        }
        #[cfg(feature = "fixed_precision")]
        {
            return fixed::from_angle_and_plane(angle.into(), plane);
        }
    }

    /// Computes and returns the geometric product of two [Rotor]'s
    #[inline]
    pub fn product(&self, other: &Self) -> Self {
        let p = self;
        let q = other;

        let mut r = Rotor { ..Rotor::default() };

        r.s = p.s * q.s - p.b.xy * q.b.xy - p.b.xz * q.b.xz - p.b.yz * q.b.yz;
        r.b.xy = p.b.xy * q.s + p.s * q.b.xy + p.b.yz * q.b.xz - p.b.xz * q.b.yz;
        r.b.xz = p.b.xz * q.s + p.s * q.b.xz - p.b.yz * q.b.xy + p.b.xy * q.b.yz;
        r.b.yz = p.b.yz * q.s + p.s * q.b.yz + p.b.xz * q.b.xy - p.b.xy * q.b.xz;
        r.normalize();
        r
    }

    /// Computes and returns a normalized version of this [Rotor]
    #[inline]
    pub fn normalized(&self) -> Self {
        let mut normalized = *self;
        normalized.normalize();
        normalized
    }

    /// Normalizes this [Rotor] in place
    #[inline]
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.s /= magnitude;
        self.b.xy /= magnitude;
        self.b.xz /= magnitude;
        self.b.yz /= magnitude;
    }

    /// Computes the magnitude (sometimes called length) of this [Rotor]
    #[inline]
    pub fn magnitude(&self) -> Float {
        self.magnitude_sq().sqrt()
    }

    /// Computes and returns the squared magnitude of this [Rotor]
    ///
    /// Slightly faster than [Rotor::magnitude()]
    #[inline]
    pub fn magnitude_sq(&self) -> Float {
        self.b.xy * self.b.xy + self.b.xz * self.b.xz + self.b.yz * self.b.yz + self.s * self.s
    }

    /// Returns a new [Rotor] that is the reverse (conjugate) of this [Rotor]
    #[inline]
    pub fn reversed(&self) -> Self {
        let mut reversed = *self;
        reversed.reverse();
        reversed
    }

    /// Reverses this [Rotor] in place
    pub fn reverse(&mut self) {
        self.b.xy = -self.b.xy;
        self.b.xz = -self.b.xz;
        self.b.yz = -self.b.yz;
    }
}

impl Mul<&Rotor> for &Rotor {
    type Output = Rotor;

    fn mul(self, rhs: &Rotor) -> Self::Output {
        self.product(rhs)
    }
}

impl Mul<Rotor> for &Rotor {
    type Output = Rotor;

    fn mul(self, rhs: Rotor) -> Self::Output {
        self.product(&rhs)
    }
}

impl Mul<&Rotor> for Rotor {
    type Output = Rotor;

    fn mul(self, rhs: &Rotor) -> Self::Output {
        self.product(rhs)
    }
}

impl Mul for Rotor {
    type Output = Rotor;

    fn mul(self, rhs: Rotor) -> Self::Output {
        self.product(&rhs)
    }
}

impl std::fmt::Display for Rotor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:+.3}, ({:+.3}, {:+.3}, {:+.3})]",
            self.s, self.b.xy, self.b.xz, self.b.yz
        )
    }
}

impl Approximately for Rotor {
    fn approximately(&self, other: Self, epsilon: Float) -> bool {
        self.s.approximately(other.s, epsilon) && self.b.approximately(other.b, epsilon)
    }
}

pub(crate) mod floating {
    use super::Rotor;
    use super::Vector;

    pub fn rotate_vector(rotor: &Rotor, vector: &mut Vector) {
        let r = rotor;
        let v = vector;
        let q = Vector::new(
            r.s * v.x + v.y * r.b.xy + v.z * r.b.xz,
            r.s * v.y - v.x * r.b.xy + v.z * r.b.yz,
            r.s * v.z - v.x * r.b.xz - v.y * r.b.yz,
        );

        let t = v.x * r.b.yz - v.y * r.b.xz + v.z * r.b.xy;

        v.x = r.s * q.x + q.y * r.b.xy + q.z * r.b.xz + t * r.b.yz;
        v.y = r.s * q.y - q.x * r.b.xy - t * r.b.xz + q.z * r.b.yz;
        v.z = r.s * q.z + t * r.b.xy - q.x * r.b.xz - q.y * r.b.yz;
    }
}

#[allow(dead_code)]
#[cfg(feature = "fixed_precision")]
pub(crate) mod fixed {
    use super::Bivector;
    use super::Rotor;
    use super::Vector;
    use crate::fixed::Fixed;
    use crate::fixed::FullFixed;
    use crate::fixed::FIXED_DECIMAL;

    /// Returns a new `Rotor` from an angle and a plane, the plane must be normalized
    #[inline]
    pub fn from_angle_and_plane(angle: Fixed, plane: Bivector) -> Rotor {
        let angle = FullFixed::from(angle);

        let pxy = FullFixed::from(plane.xy);
        let pxz = FullFixed::from(plane.xz);
        let pyz = FullFixed::from(plane.yz);

        let sina = FullFixed(angle.0 / 2).sin();
        let cosa = FullFixed(angle.0 / 2).cos();

        let bv = Bivector {
            xy: Fixed::from(-sina * pxy),
            xz: Fixed::from(-sina * pxz),
            yz: Fixed::from(-sina * pyz),
        };
        Rotor::new(bv, Fixed::from(cosa)).normalized()
    }

    #[inline]
    pub fn rotate_vector(rotor: &Rotor, vector: &mut Vector) {
        let rots = FullFixed::from(rotor.s);
        let rbxy = FullFixed::from(rotor.b.xy);
        let rbyz = FullFixed::from(rotor.b.yz);
        let rbxz = FullFixed::from(rotor.b.xz);

        let mut vx = FullFixed::from(vector.x);
        let mut vy = FullFixed::from(vector.y);
        let mut vz = FullFixed::from(vector.z);

        let (qx, qy, qz) = (
            rots * vx + vy * rbxy + vz * rbxz,
            rots * vy - vx * rbxy + vz * rbyz,
            rots * vz - vx * rbxz - vy * rbyz,
        );

        let t = vx * rbyz - vy * rbxz + vz * rbxy;

        vx = rots * qx + qy * rbxy + qz * rbxz + t * rbyz;
        vy = rots * qy - qx * rbxy - t * rbxz + qz * rbyz;
        vz = rots * qz + t * rbxy - qx * rbxz - qy * rbyz;

        *vector = Vector::new(FullFixed(vx.0), FullFixed(vy.0), FullFixed(vz.0));
    }
}

#[cfg(test)]
mod rotor_tests {
    use super::*;
    use crate::constant::PI;
    use crate::traits::Zero;
    use crate::Approximately;
    use crate::Vector;

    const EPSILON: Float = Float::EPSILON;

    fn test_vector() -> Vector {
        Vector::new(1.0, 0.0, 0.0)
    }

    #[test]
    fn identity_rotation() {
        let mut v = test_vector();
        let identity = Rotor::identity();
        identity.rotate_vector(&mut v);
        assert!(v.approximately(test_vector(), EPSILON));
    }

    #[test]
    fn half_turn_rotation() {
        let mut v = test_vector();
        let half_turn = Rotor::from_angle_and_plane(PI, Bivector::unit_xy());
        half_turn.rotate_vector(&mut v);
        assert!(v.approximately(Vector::new(-1.0, 0.0, 0.0), EPSILON));
    }

    #[test]
    fn quarter_turn_rotation() {
        let mut v = test_vector();
        let quarter_turn = Rotor::from_angle_and_plane(PI / Float::from(2.0), Bivector::unit_xy());
        quarter_turn.rotate_vector(&mut v);
        assert!(v.approximately(Vector::new(0.0, 1.0, 0.0), EPSILON));
    }

    #[test]
    fn vector_rotation_between_vectors() {
        let from = Vector::unit_x();
        let to = Vector::unit_y();
        let rotor = Rotor::from_rotation_between_vectors(from, to);
        let mut rotated = from;
        rotor.rotate_vector(&mut rotated);
        assert!(rotated.approximately(to, EPSILON));
    }

    #[test]
    fn rotor_composition() {
        let rot_x = Rotor::from_angle_and_plane(PI / Float::from(2.0), Bivector::unit_xz());
        let rot_y = Rotor::from_angle_and_plane(PI / Float::from(2.0), Bivector::unit_yz());

        let mut v = test_vector();
        let combined = rot_x * rot_y;

        combined.rotate_vector(&mut v);
        assert!(v.approximately(Vector::new(0.0, 0.0, 1.0), EPSILON));
    }

    #[test]
    fn normalization() {
        let unnormalized = Rotor::new(Bivector::new(3.0, 4.0, 0.0), Float::ZERO);
        let normalized = unnormalized.normalized();
        assert!(normalized.magnitude().approximately(1.0, EPSILON));
    }

    #[test]
    fn reverse_operation() {
        let original = Rotor::from_angle_and_plane(PI / Float::from(4.0), Bivector::unit_xy());
        let reversed = original.reversed();

        let mut v = test_vector();
        original.rotate_vector(&mut v);
        reversed.rotate_vector(&mut v);
        assert!(v.approximately(test_vector(), EPSILON));
    }

    #[test]
    fn zero_angle_rotation() {
        let rotor = Rotor::from_angle_and_plane(Float::ZERO, Bivector::unit_xy());
        let mut v = test_vector();
        rotor.rotate_vector(&mut v);
        assert!(v.approximately(test_vector(), EPSILON));
    }

    #[test]
    fn parallel_vector_rotation() {
        let v = Vector::unit_x();
        let rotor = Rotor::from_rotation_between_vectors(v, v);
        assert!(rotor.approximately(Rotor::default(), EPSILON));
    }

    #[test]
    fn opposite_vector_rotation() {
        let from = Vector::unit_x();
        let to = -Vector::unit_x();
        let rotor = Rotor::from_rotation_between_vectors(from, to);
        let mut rotated = from;
        rotor.rotate_vector(&mut rotated);
        assert!(rotated.approximately(to, EPSILON));
    }

    #[test]
    fn rotor_magnitude_properties() {
        let rotor = Rotor::from_angle_and_plane(PI / Float::from(3.0), Bivector::unit_xz());
        assert!(rotor
            .magnitude_sq()
            .approximately(rotor.magnitude().powf(Float::from(2.0)), EPSILON));
    }

    #[test]
    fn rotor_product_identity() {
        let id = Rotor::identity();
        let rotor = Rotor::from_angle_and_plane(PI / Float::from(4.0), Bivector::unit_xy());
        assert!((rotor * id).approximately(rotor, EPSILON));
        assert!((id * rotor).approximately(rotor, EPSILON));
    }

    #[test]
    fn rotor_inverse_property() {
        let rotor = Rotor::from_angle_and_plane(PI / Float::from(3.0), Bivector::unit_yz());
        let inverse = rotor.reversed();
        assert!((rotor * inverse).approximately(Rotor::default(), EPSILON));
    }

    #[test]
    fn rotor_interaction_with_zero_vector() {
        let mut v = Vector::zero();
        let rotor = Rotor::from_angle_and_plane(PI / Float::from(2.0), Bivector::unit_xy());
        rotor.rotate_vector(&mut v);
        assert_eq!(v, Vector::zero());
    }
}
