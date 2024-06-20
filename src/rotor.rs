//!
//! Rotor
//!

use std::ops::Mul;

use crate::bivec::Bivector;
use crate::{Float, Vector};

// Notes:
// the "wild rotations" you mention has a very simple solution employed by every engine I've worked
// with. Basically, you just constrain the real part to be positive which fixes your interpolation
// on one half of the Lie-manifold which ensures the arc taken is as short as possible.

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

    /// Returns a new `Rotor` that rotates one unit vector to another unit vector
    #[inline]
    pub fn from_rotation_between_vectors(from: Vector, to: Vector) -> Self {
        let rotor = Rotor {
            b: Bivector::from_wedge(to, from),
            s: 1.0 + to.dot(&from),
        }
        .normalized();
        rotor
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
        let magnitude = self.magnitude_sq();
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
        let ss = self.s * self.s;
        self.b.magnitude_sq() + ss
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
