//!
//! Floating point number extensions and helpers 
//! 

use crate::Float;

pub trait FloatExt {
    fn approximately(self, other: Self, epsilon: Self) -> bool;
}

impl FloatExt for Float {
    /// Computes whether this [Float] is approximately equal to another [Float] using an epsilon
    fn approximately(self, other: Self, epsilon: Self) -> bool {
        let a = Float::abs(self);
        let b = Float::abs(other);
        let difference = Float::abs(a - b);

        if self == other {
            return true;
        } else if self == 0.0 || self == 0.0 || a + b < Self::MIN_POSITIVE {
            return difference < (epsilon * Self::MIN_POSITIVE)
        } else {
            return difference / Self::min(a + b, Self::MAX) < epsilon
        }
    }
}
