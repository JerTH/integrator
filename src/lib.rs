/// A math library
#[cfg(all(feature = "low_precision", feature = "high_precision"))]
compile_error!(
    "feature \"low_precision\" and feature \"high_precision\" cannot be enabled at the same time"
);

#[cfg(feature = "low_precision")]
mod types {
    pub type FType = f32;
    pub type IType = i32;
    pub type UType = u32;
}

#[cfg(feature = "high_precision")]
mod types {
    pub type FType = f64;
    pub type IType = i64;
    pub type UType = u64;
}

pub type Float = types::FType;
pub type Int = types::IType;
pub type Unsigned = types::UType;

pub mod traits;
pub mod fixed;
pub mod bivec;
pub mod constant;
pub mod line;
pub mod segment;
pub mod matrix;
pub mod percent;
pub mod plane;
pub mod point;
pub mod rotor;
pub mod vec;
pub mod sphere;

pub use matrix::Matrix;
pub use point::Point;
pub use rotor::Rotor;
pub use vec::Vector;

use traits::{Approximately, Zero};

impl Zero for Float {
    fn zero() -> Self {
        Float::from(0.0)
    }
}

trait One {
    fn one() -> Self;
}

impl One for Float {
    fn one() -> Self {
        Float::from(1.0)
    }
}

impl Approximately for Float {
    fn approximately(&self, other: Self, epsilon: f64) -> bool {
        // If either value is NaN, then they can not be equal
        if self.is_nan() || other.is_nan() {
            return false;
        }
        // If the two numbers are exactly equal (including infinities), they are approximately equal.
        if self == &other {
            return true;
        }
        // Compare the absolute difference to epsilon.
        (self - other).abs() <= epsilon
    }
}

impl Approximately for &Float {
    fn approximately(&self, other: Self, epsilon: Float) -> bool {
        Float::approximately(*self, *other, epsilon)
    }
}

impl Approximately for &mut Float {
    fn approximately(&self, other: Self, epsilon: Float) -> bool {
        Float::approximately(*self, *other, epsilon)
    }
}

#[cfg(test)]
mod equality_tests {
    use super::*;
    use std::f64::{INFINITY, NAN, NEG_INFINITY};

    #[test]
    fn exact_equality() {
        let a = 1.0_f64;
        let b = 1.0_f64;
        assert!(a.approximately(b, 0.0));
        assert!(a.approximately(b, f64::EPSILON));
    }

    #[test]
    fn difference_equals_epsilon() {
        let a = 1.0;
        let b = 1.0 + 0.5;
        assert!(a.approximately(b, 0.5));
        assert!(b.approximately(a, 0.5));
    }

    #[test]
    fn difference_exceeds_epsilon() {
        let a = 1.0;
        let b = 1.0 + 0.5 + f64::EPSILON;
        assert!(!a.approximately(b, 0.5));
    }

    #[test]
    fn zero_edge_cases() {
        assert!(0.0.approximately(0.0, 0.0));
        assert!(0.0.approximately(1e-10, 1e-5));
        assert!(!0.0.approximately(1e-5, 1e-6));
    }

    #[test]
    fn opposite_signs() {
        assert!(!5.0.approximately(-5.0, 9.9));
        assert!(5.0.approximately(-5.0, 10.1));
    }

    #[test]
    fn subnormal_numbers() {
        let min = f64::MIN_POSITIVE;
        let a = min;
        let b = min + min / 2.0;
        assert!(a.approximately(b, min));
    }

    #[test]
    fn nan_handling() {
        assert!(!NAN.approximately(NAN, f64::MAX));
        assert!(!NAN.approximately(1.0, f64::MAX));
        assert!(!1.0.approximately(NAN, f64::MAX));
    }

    #[test]
    fn infinity_handling() {
        assert!(INFINITY.approximately(INFINITY, 0.0));
        assert!(!INFINITY.approximately(NEG_INFINITY, f64::MAX));
        assert!(!INFINITY.approximately(1.0, f64::MAX));
        assert!(!1.0.approximately(INFINITY, f64::MAX));
    }

    #[test]
    fn large_numbers() {
        let a = 1e20;
        let b = a + 1e15;
        assert!(a.approximately(b, 1e16));
        assert!(!a.approximately(b, 1e14));
    }

    #[test]
    fn tiny_epsilon() {
        let a = 1.0 + 2.0 * f64::EPSILON;
        let b = 1.0;
        assert!(!a.approximately(b, f64::EPSILON));
        assert!(a.approximately(b, 3.0 * f64::EPSILON));
    }

    #[test]
    fn symmetry_property() {
        let a = 1.0;
        let b = 1.000_000_1;
        let eps = 0.000_000_2;
        assert_eq!(a.approximately(b, eps), b.approximately(a, eps));
    }

    #[test]
    fn transitive_property() {
        let a = 1.0;
        let b = 1.000_000_05;
        let c = 1.000_000_1;
        let eps = 0.000_000_2;
        assert!(a.approximately(b, eps));
        assert!(b.approximately(c, eps));
        assert!(a.approximately(c, eps));
    }
}
