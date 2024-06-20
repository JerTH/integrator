//!
//! Bivector
//! 

use crate::Float;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Bivector {
    pub xy: Float,
    pub xz: Float,
    pub yz: Float,
}

impl Bivector {
    #[inline]
    pub fn magnitude_sq(&self) -> Float {
        self.xy * self.xy
    }

    #[inline]
    pub fn magnitude(&self) -> Float {
        self.xy
    }
}
