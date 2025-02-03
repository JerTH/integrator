/// A math library
#[cfg(all(feature = "low_precision", feature = "high_precision"))]
compile_error!("feature \"low_precision\" and feature \"high_precision\" cannot be enabled at the same time");

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

pub mod constant;
pub mod vec;
pub mod matrix;
pub mod point;
pub mod bivec;
pub mod rotor;
pub mod plane;
pub mod line;
pub mod percent;

pub use constant::*;
pub use vec::Vector;
pub use point::Point;
pub use matrix::Matrix;
pub use rotor::Rotor;

trait Zero {
    fn zero() -> Self;
}

trait One {
    fn one() -> Self;
}

pub trait Approximately {
    fn approximately(&self, other: &Self, epsilon: Float) -> bool;
}

impl Zero for Float {
    fn zero() -> Self {
        Float::from(0.0)
    }
}

impl One for Float {
    fn one() -> Self {
        Float::from(1.0)
    }
}

impl Approximately for Float {
    fn approximately(&self, other: &Self, epsilon: Float) -> bool {
        if *self == *other { return true; }

        debug_assert!(Self::EPSILON <= epsilon);

        let diff = dbg!(Self::abs(*self - *other));

        if diff < epsilon { return true; }

        let norm = dbg!(Self::min(Self::abs(*self) + Self::abs(*other), Self::MAX));
        let epno = dbg!(epsilon * norm);

        return diff < Self::max(Self::MIN, epno);
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use std::f64::{INFINITY, NAN, NEG_INFINITY};

    #[test]
    fn exact_equality() {
        let a = 1.0_f64;
        let b = 1.0_f64;
        assert!(a.approximately(&b, 0.0));
        assert!(a.approximately(&b, f64::EPSILON));
    }

    #[test]
    fn difference_equals_epsilon() {
        let a = 1.0;
        let b = 1.0 + 0.5;
        assert!(a.approximately(&b, 0.5));
        assert!(b.approximately(&a, 0.5));
    }

    #[test]
    fn difference_exceeds_epsilon() {
        let a = 1.0;
        let b = 1.0 + 0.5 + f64::EPSILON;
        assert!(!a.approximately(&b, 0.5));
    }

    #[test]
    fn zero_edge_cases() {
        assert!(0.0.approximately(&0.0, 0.0));
        assert!(0.0.approximately(&1e-10, 1e-5));
        assert!(!0.0.approximately(&1e-5, 1e-6));
    }

    #[test]
    fn opposite_signs() {
        assert!(!5.0.approximately(&-5.0, 9.9));
        assert!(5.0.approximately(&-5.0, 10.1));
    }

    #[test]
    fn subnormal_numbers() {
        let min = f64::MIN_POSITIVE;
        let a = min;
        let b = min + min / 2.0;
        assert!(a.approximately(&b, min));
    }

    #[test]
    fn nan_handling() {
        assert!(!NAN.approximately(&NAN, f64::MAX));
        assert!(!NAN.approximately(&1.0, f64::MAX));
        assert!(!1.0.approximately(&NAN, f64::MAX));
    }

    #[test]
    fn infinity_handling() {
        assert!(INFINITY.approximately(&INFINITY, 0.0));
        assert!(!INFINITY.approximately(&NEG_INFINITY, f64::MAX));
        assert!(!INFINITY.approximately(&1.0, f64::MAX));
        assert!(!1.0.approximately(&INFINITY, f64::MAX));
    }

    #[test]
    fn large_numbers() {
        let a = 1e20;
        let b = a + 1e15;
        assert!(a.approximately(&b, 1e16));
        assert!(!a.approximately(&b, 1e14));
    }

    #[test]
    fn tiny_epsilon() {
        let a = 1.0 + 2.0 * f64::EPSILON;
        let b = 1.0;
        assert!(!a.approximately(&b, f64::EPSILON));
        assert!(a.approximately(&b, 3.0 * f64::EPSILON));
    }

    #[test]
    fn symmetry_property() {
        let a = 1.0;
        let b = 1.000_000_1;
        let eps = 0.000_000_2;
        assert_eq!(
            a.approximately(&b, eps),
            b.approximately(&a, eps)
        );
    }

    #[test]
    fn transitive_property() {
        let a = 1.0;
        let b = 1.000_000_05;
        let c = 1.000_000_1;
        let eps = 0.000_000_2;
        assert!(a.approximately(&b, eps));
        assert!(b.approximately(&c, eps));
        assert!(a.approximately(&c, eps));
    }
}