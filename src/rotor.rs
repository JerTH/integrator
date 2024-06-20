//!
//! Rotor
//!

use crate::bivec::Bivector;
use crate::Float;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Rotor {
    pub b: Bivector,
    pub s: Float,
}

impl Rotor {
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
