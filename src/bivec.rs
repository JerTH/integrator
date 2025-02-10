//!
//! Bivector
//! 

use serde::{Serialize, Deserialize};

use crate::{Approximately, Float, Vector, traits::FloatExt};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Bivector {
    pub xy: Float,
    pub xz: Float,
    pub yz: Float,
}

impl Bivector {
    #[inline]
    pub fn new<F: Into<Float>>(xy: F, xz: F, yz: F) -> Self {
        Self {
            xy: xy.into(), xz: xz.into(), yz: yz.into(),
        }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            xy: Float::ZERO,
            xz: Float::ZERO,
            yz: Float::ZERO,
        }
    }

    #[inline]
    pub fn unit_xy() -> Self {
        Self {
            xy: Float::ONE,
            xz: Float::ZERO,
            yz: Float::ZERO,
        }
    }

    #[inline]
    pub fn unit_xz() -> Self {
        Self {
            xy: Float::ZERO,
            xz: Float::ONE,
            yz: Float::ZERO,
        }
    }

    #[inline]
    pub fn unit_yz() -> Self {
        Self {
            xy: Float::ZERO,
            xz: Float::ZERO,
            yz: Float::ONE,
        }
    }
    
    #[inline]
    pub fn from_axis_vector(axis: Vector) -> Self {
        Self::new(axis.z, axis.y, axis.x)
    }
    
    #[inline]
    pub fn from_wedge<V>(u: V, v: V) -> Self where V: Into<Vector> {
        let u: Vector = u.into();
        let v: Vector = v.into();
        Self {
            xy: u.x * v.y - u.y * v.x,
            xz: u.x * v.z - u.z * v.x,
            yz: u.y * v.z - u.z * v.y,
        }
    }

    #[inline]
    pub fn magnitude_sq(&self) -> Float {
        self.xy * self.xy
    }

    #[inline]
    pub fn magnitude(&self) -> Float {
        self.xy
    }
}

impl Approximately for Bivector {
    fn approximately(&self, other: Self, epsilon: Float) -> bool {
        self.xy.approximately(other.xy, epsilon)
            && self.xz.approximately(other.xz, epsilon)
            && self.yz.approximately(other.yz, epsilon)
    }
}
