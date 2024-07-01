//!
//! Bivector
//! 

use serde::{Serialize, Deserialize};

use crate::{Float, Vector};

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
    pub fn new(xy: Float, xz: Float, yz: Float) -> Self {
        Self {
            xy, xz, yz,
        }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            xy: 0.0,
            xz: 0.0,
            yz: 0.0,
        }
    }

    #[inline]
    pub fn unit_xy() -> Self {
        Self {
            xy: 1.0,
            xz: 0.0,
            yz: 0.0,
        }
    }

    #[inline]
    pub fn unit_xz() -> Self {
        Self {
            xy: 0.0,
            xz: 1.0,
            yz: 0.0,
        }
    }

    #[inline]
    pub fn unit_yz() -> Self {
        Self {
            xy: 0.0,
            xz: 0.0,
            yz: 1.0,
        }
    }

    #[inline]
    pub fn from_axis_vector(axis: Vector) -> Self {
        Self::new(axis.z, -axis.y, axis.x)
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
