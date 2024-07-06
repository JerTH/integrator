//!
//! Rotor
//!

use std::ops::Mul;
use serde::{Serialize, Deserialize};
use crate::bivec::Bivector;
use crate::{Float, Vector};

// Notes:
// the "wild rotations" you mention has a very simple solution employed by every engine I've worked
// with. Basically, you just constrain the real part to be positive which fixes your interpolation
// on one half of the Lie-manifold which ensures the arc taken is as short as possible.

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Rotor {
    pub b: Bivector,
    pub s: Float,
}

impl Rotor {
    pub fn new(bivector: Bivector, scalar: Float) -> Self {
        Self {
            b: bivector,
            s: scalar,
        }
    }
    
    /// Rotate a [Vector] by the rotation represented by this [Rotor]
    /// 
    /// ```
    /// # use integrator::{ Float, vec::Vector, rotor::Rotor, bivec::Bivector };
    /// let mut from = Vector::new(4.0, 5.0, 3.0).normalized();
    /// let to = Vector::new(2.0, 5.0, 2.0).normalized();
    /// let rotor = Rotor::from_rotation_between_vectors(from, to);
    /// rotor.rotate_vector(&mut from);
    /// assert!(to.approximately(&from, Float::EPSILON));
    /// ```
    pub fn rotate_vector(&self, vector: &mut Vector) {
        let r = self;
        let v = vector;
        let q = Vector::new(
            r.s * v.x + v.y * r.b.xy + v.z * r.b.xz,
            r.s * v.y - v.x * r.b.xy + v.z * r.b.yz,
            r.s * v.z - v.x * r.b.xz - v.y * r.b.yz,
        );

        let t = v.x * r.b.yz - v.y * r.b.xz + v.z * r.b.xy;

        v.x = r.s * q.x + q.y * r.b.xy + q.z * r.b.xz + t   * r.b.yz;
        v.y = r.s * q.y - q.x * r.b.xy - t   * r.b.xz + q.z * r.b.yz;
        v.z = r.s * q.z + t   * r.b.xy - q.x * r.b.xz - q.y * r.b.yz;
    }
    
    /// Returns a new `Rotor` that rotates one unit vector to another unit vector
    #[inline]
    pub fn from_rotation_between_vectors(from: Vector, to: Vector) -> Self {
        Rotor {
            b: Bivector::from_wedge(to, from),
            s: 1.0 + Vector::dot(&to, &from),
        }.normalized()
    }

    /// Returns a new `Rotor` from an angle and a plane, the plane must be normalized
    #[inline]
    pub fn from_angle_and_plane(angle: Float, plane: Bivector) -> Self {
        let sina = (angle / 2.0).sin();
        let cosa = (angle / 2.0).cos();
        let bv = Bivector {
            xy: -sina * plane.xy,
            xz: -sina * plane.xz,
            yz: -sina * plane.yz,
        };
        Rotor::new(bv, cosa)
    }

    #[inline]
    pub fn product(&self, other: &Self) -> Self {
        let p = self;
        let q = other;
        let mut r = Self::default();
        r.s = p.s * q.s - p.b.xy * q.b.xy - p.b.xz * q.b.xz - p.b.yz * q.b.yz;
        r.b.xy = p.b.xy * q.s + p.s * q.b.xy + p.b.yz * q.b.xz - p.b.xz * q.b.yz;
        r.b.xz = p.b.xz * q.s + p.s * q.b.xz - p.b.yz * q.b.xy + p.b.xy * q.b.yz;
        r.b.yz = p.b.yz * q.s + p.s * q.b.yz + p.b.xz * q.b.xy - p.b.xy * q.b.xz;
        r
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let mut normalized = *self;
        normalized.normalize_in_place();
        normalized
    }

    #[inline(always)]
    pub fn normalize(&mut self) {
        self.normalize_in_place()
    }

    #[inline]
    fn normalize_in_place(&mut self) {
        let magnitude = self.magnitude();
        self.s /= magnitude;
        self.b.xy /= magnitude;
        self.b.xz /= magnitude;
        self.b.yz /= magnitude;
    }

    #[inline]
    pub fn magnitude(&self) -> Float {
        self.magnitude_sq().sqrt()
    }

    #[inline]
    pub fn magnitude_sq(&self) -> Float {
        self.b.xy * self.b.xy +
        self.b.xz * self.b.xz +
        self.b.yz * self.b.yz +
        self.s * self.s
    }
}

impl Mul for Rotor {
    type Output = Rotor;

    fn mul(self, rhs: Rotor) -> Self::Output {
        self.product(&rhs)
    }
}

impl Mul<Rotor> for &Rotor {
    type Output = Rotor;

    fn mul(self, rhs: Rotor) -> Self::Output {
        self.product(&rhs)
    }
}

impl Mul<&Rotor> for &Rotor {
    type Output = Rotor;

    fn mul(self, rhs: &Rotor) -> Self::Output {
        self.product(rhs)
    }
}
