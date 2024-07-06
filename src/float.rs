//!
//! Floating point number extensions and helpers 
//! 

use crate::Float;

pub trait FloatExt {
    fn approximately(self, other: Self, epsilon: Self) -> bool;
}
/*
public static boolean nearlyEqual(float a, float b, float epsilon) {
		final float absA = Math.abs(a);
		final float absB = Math.abs(b);
		final float diff = Math.abs(a - b);

		if (a == b) { // shortcut, handles infinities
			return true;
    } else if (a == 0 || b == 0 || (absA + absB < Float.MIN_NORMAL)) {
			// a or b is zero or both are extremely close to it
			// relative error is less meaningful here
			return diff < (epsilon * Float.MIN_NORMAL);
		} else { // use relative error
			return diff / Math.min((absA + absB), Float.MAX_VALUE) < epsilon;
		}
	}
*/
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
