/// A math library
#[cfg(all(feature = "low_precision", feature = "high_precision"))]
compile_error!(
    "feature \"low_precision\" and feature \"high_precision\" cannot be enabled at the same time"
);

#[cfg(not(feature = "fixed_precision"))]
mod precision {
    use types::FType;

    use crate::traits::{Approximately, FloatExt, FromLossy};

    #[cfg(feature = "low_precision")]
    pub(crate) mod types {
        pub type FType = f32;
        pub type IType = i32;
        pub type UType = u32;
    }

    #[cfg(feature = "high_precision")]
    pub(crate) mod types {
        pub type FType = f64;
        pub type IType = i64;
        pub type UType = u64;
    }

    impl FloatExt for FType {
        const ONE: Self = 1.0;
        const ZERO: Self = 0.0;
    }

    impl FromLossy<i32> for FType {
        fn from_lossy(value: i32) -> Self {
            value as Self
        }
    }

    impl FromLossy<i64> for FType {
        fn from_lossy(value: i64) -> Self {
            value as Self
        }
    }

    impl FromLossy<f32> for FType {
        fn from_lossy(value: f32) -> Self {
            value as Self
        }
    }

    impl FromLossy<f64> for FType {
        fn from_lossy(value: f64) -> Self {
            value as Self
        }
    }

    impl Approximately for FType {
        fn approximately(&self, other: Self, epsilon: FType) -> bool {
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

    impl Approximately for &FType {
        fn approximately(&self, other: Self, epsilon: FType) -> bool {
            FType::approximately(*self, *other, epsilon)
        }
    }

    impl Approximately for &mut FType {
        fn approximately(&self, other: Self, epsilon: FType) -> bool {
            FType::approximately(*self, *other, epsilon)
        }
    }
}

#[cfg(feature = "fixed_precision")]
mod precision {
    use types::FType;

    pub(crate) mod types {
        pub type FType = crate::fixed::Fixed;
        pub type IType = i64;
        pub type UType = u64;
    }

    impl FType {
        pub const ONE: Self = FType::from_const(1.0);
        pub const ZERO: Self = FType::from_const(0.0);
    }
}

pub type Float = precision::types::FType;
pub type Int = precision::types::IType;
pub type Unsigned = precision::types::UType;

pub mod bivec;
pub mod constant;
pub mod fixed;
pub mod line;
pub mod matrix;
pub mod percent;
pub mod plane;
pub mod point;
pub mod rotor;
pub mod segment;
pub mod sphere;
pub mod traits;
pub mod vec;

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

#[cfg(test)]
mod equality_tests {
    use super::*;
    use std::f64::{INFINITY, NAN, NEG_INFINITY};

    #[test]
    fn exact_equality() {
        let a: Float = 1.0.into();
        let b: Float = 1.0.into();
        assert!(a.approximately(b, Float::from(0.0)));
        assert!(a.approximately(b, Float::EPSILON));
    }

    #[cfg(feature = "fixed_precision")]
    #[ignore]
    #[test]
    fn difference_equals_epsilon() {
        let a = Float::from(1.0);
        let b = Float::from(1.0 + 0.5);
        assert!(a.approximately(b, Float::from(0.5)));
        assert!(b.approximately(a, Float::from(0.5)));
    }

    #[test]
    fn difference_exceeds_epsilon() {
        let a = Float::from(1.0);
        let b = Float::from(1.0 + 0.5 + f64::EPSILON);
        assert!(!a.approximately(b, Float::from(0.5)));
    }

    #[cfg(feature = "fixed_precision")]
    #[ignore]
    #[test]
    fn zero_edge_cases() {
        assert!(Float::from(0.0).approximately(Float::from(0.0), Float::from(0.0)));
        assert!(Float::from(0.0).approximately(Float::from(1e-10), Float::from(1e-5)));
        assert!(!Float::from(0.0).approximately(Float::from(1e-5), Float::from(1e-6)));
    }

    #[cfg(feature = "fixed_precision")]
    #[ignore]
    #[test]
    fn opposite_signs() {
        assert!(!Float::from(5.0).approximately(Float::from(-5.0), Float::from(9.9)));
        assert!(Float::from(5.0).approximately(Float::from(-5.0), Float::from(10.1)));
    }

    #[test]
    fn subnormal_numbers() {
        let min = Float::from(f64::MIN_POSITIVE);
        let a = Float::from(min);
        let b = Float::from(min + min / Float::from(2.0));
        assert!(a.approximately(b, min));
    }

    #[cfg(feature = "fixed_precision")]
    #[ignore]
    #[test]
    fn nan_handling() {
        assert!(!Float::from(NAN).approximately(Float::from(NAN), Float::from(f64::MAX)));
        assert!(!Float::from(NAN).approximately(Float::from(1.0), Float::from(f64::MAX)));
        assert!(!Float::from(1.0).approximately(Float::from(NAN), Float::from(f64::MAX)));
    }

    #[test]
    fn infinity_handling() {
        assert!(Float::from(INFINITY).approximately(Float::from(INFINITY), Float::from(0.0)));
        assert!(
            !Float::from(INFINITY).approximately(Float::from(NEG_INFINITY), Float::from(f64::MAX))
        );
        assert!(!Float::from(INFINITY).approximately(Float::from(1.0), Float::from(f64::MAX)));
        assert!(!Float::from(1.0).approximately(Float::from(INFINITY), Float::from(f64::MAX)));
    }

    #[cfg(feature = "fixed_precision")]
    #[ignore]
    #[test]
    fn large_numbers() {
        let a = Float::from(1e20);
        let b = Float::from(1e20 + 1e15);
        assert!(a.approximately(b, Float::from(1e16)));
        assert!(!a.approximately(b, Float::from(1e14)));
    }

    #[cfg(feature = "fixed_precision")]
    #[ignore]
    #[test]
    fn tiny_epsilon() {
        let a = Float::from(1.0 + 2.0 * f64::EPSILON);
        let b = Float::from(1.0);
        assert!(!a.approximately(b, Float::from(f64::EPSILON)));
        assert!(a.approximately(b, Float::from(3.0 * f64::EPSILON)));
    }

    #[test]
    fn symmetry_property() {
        let a = Float::from(1.0);
        let b = Float::from(1.000_000_1);
        let eps = Float::from(0.000_000_2);
        assert_eq!(a.approximately(b, eps), b.approximately(a, eps));
    }

    #[test]
    fn transitive_property() {
        let a = Float::from(1.0);
        let b = Float::from(1.000_000_05);
        let c = Float::from(1.000_000_1);
        let eps = Float::from(0.000_000_2);
        assert!(a.approximately(b, eps));
        assert!(b.approximately(c, eps));
        assert!(a.approximately(c, eps));
    }
}
