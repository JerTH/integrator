//! Fixed-point arithmetic
//! 
//! This is not designed to be a fully general purpose fixed point arithmetic system
//! Instead, the goal is to be generally useful for 3D graphics and interactive simulations

use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}};

use crate::Float;

type Int = i64;
type FullInt = i128;

pub const FIXED_DECIMAL: FullInt = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed(pub Int);

impl Fixed {
    #[inline(always)]
    fn full(&self) -> FullFixed {
        FullFixed(self.0 as FullInt)
    }
}

impl From<Float> for Fixed {
    fn from(value: Float) -> Self {
        Self((value * FIXED_DECIMAL as Float).round() as Int)
    }
}

impl Neg for Fixed {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

macro_rules! fixed_binop {
    ($lhs:ty, $rhs:ty, $func:ident, $trait:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = Fixed;
            fn $func(self, other: $rhs) -> Self::Output {
                FullFixed::$func(self.full(), other.full())
            }
        }
    };
}

fixed_binop!(Fixed, Fixed, add, Add);
fixed_binop!(Fixed, Fixed, sub, Sub);
fixed_binop!(Fixed, Fixed, mul, Mul);
fixed_binop!(Fixed, Fixed, div, Div);

impl Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Float::div(self.0 as Float, FIXED_DECIMAL as Float))
    }
}

struct FullFixed(pub FullInt);

impl Neg for FullFixed {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

macro_rules! fullfixed_binop {
    ($lhs:ty, $rhs:ty, $func:ident, $trait:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = Fixed;
            fn $func(self, other: $rhs) -> Self::Output {
                Fixed(FullInt::$func(self.0, other.0) as Int)
            }
        }
    };
}

fullfixed_binop!(FullFixed, FullFixed, add, Add);
fullfixed_binop!(FullFixed, FullFixed, sub, Sub);

impl Mul for FullFixed {
    type Output = Fixed;

    fn mul(self, rhs: Self) -> Self::Output {
        Fixed(FullInt::div(FullInt::mul(self.0, rhs.0), FIXED_DECIMAL) as Int)
    }
}

impl Div for FullFixed {
    type Output = Fixed;

    fn div(self, rhs: Self) -> Self::Output {
        Fixed(FullInt::div(FullInt::mul(self.0, FIXED_DECIMAL), rhs.0) as Int)
    }
}

#[cfg(test)]
mod fixedpoint_tests {
    use super::*;

    #[test]
    fn from_float() {
        let float_val = 132.1139;
        let fixed_val = Fixed::from(float_val);

        debug_assert_eq!(132114, fixed_val.0)
    }

    #[test]
    fn addition() {
        let a = Fixed::from(14.0);
        let b = Fixed::from(16.0);
        let expected = Fixed::from(14.0 + 16.0);

        debug_assert_eq!(expected, a + b)
    }

    #[test]
    fn subtraction() {
        let a = Fixed::from(10.0);
        let b = Fixed::from(20.0);
        let expected = Fixed::from(10.0 - 20.0);

        debug_assert_eq!(expected, a - b)
    }

    #[test]
    fn multiplication() {
        let a = Fixed::from(1.5);
        let b = Fixed::from(30.0);
        let expected = Fixed::from(1.5 * 30.0);

        debug_assert_eq!(expected, a * b)
    }

    #[test]
    fn division() {
        let a = Fixed::from(333.255);
        let b = Fixed::from(-9.0);
        let expected = Fixed::from(333.255 / -9.0);

        debug_assert_eq!(expected, a / b)
    }
}
