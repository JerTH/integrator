//! Fixed-point arithmetic
//! 
//! This is not designed to be a fully general purpose fixed point arithmetic system
//! Instead, the goal is to be generally useful for 3D graphics and interactive simulations

use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use f64 as Float;

impl From<Fixed> for Float {
    fn from(value: Fixed) -> Self {
        (value.0 as Float) / (FIXED_DECIMAL as Float)
    }
}

use serde::{Deserialize, Serialize};

use crate::traits::{Approximately, FromLossy};

type Int = i64;
type FullInt = i128;

pub const FIXED_DECIMAL: FullInt = 10000;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed(pub Int);

impl Fixed {
    pub const EPSILON: Self = Self(0);

    #[inline(always)]
    fn full(&self) -> FullFixed {
        FullFixed(self.0 as FullInt)
    }

    #[cfg(feature = "fixed_precision")]
    pub(crate) const fn from_const(value: Float) -> Self {
        let rounded = const_round_to_decimal_point(value * FIXED_DECIMAL as Float);
        Self(rounded as Int)
    }

    pub fn abs(self) -> Self {
        todo!()
    }

    pub fn sqrt(self) -> Self {
        let f = Float::from(self);
        Self::from(Float::sqrt(f))
    }

    pub fn powi(self, exp: i32) -> Self {
        todo!("{exp}")
    }

    pub fn powf(self, exp: Self) -> Self {
        let f = Float::from(self);
        let e = Float::from(exp);
        Self::from(Float::powf(f, e))
    }

    pub fn signum(self) -> Self {
        if self.0 >= 0 {
            Fixed(FIXED_DECIMAL as Int)
        } else {
            Fixed(-FIXED_DECIMAL as Int)
        }
    }

    pub fn sin(self) -> Self {
        let f = Float::from(self);
        Self::from(Float::sin(f))
    }

    pub fn cos(self) -> Self {
        let f = Float::from(self);
        Self::from(Float::cos(f))
    }

    pub fn tan(self) -> Self {
        let f = Float::from(self);
        Self::from(Float::tan(f))
    }

    pub fn acos(self) -> Self {
        let f = Float::from(self);
        Self::from(Float::acos(f))
    }

    pub fn round(self) -> Self {
        let f = Float::from(self);
        Self::from(Float::round(f))
    }
}

impl From<f64> for Fixed {
    fn from(value: f64) -> Self {
        Self((value * FIXED_DECIMAL as f64).round() as Int)
    }
}

impl From<f32> for Fixed {
    fn from(value: f32) -> Self {
        Self((value * FIXED_DECIMAL as f32).round() as Int)
    }
}

impl From<i64> for Fixed {
    fn from(value: i64) -> Self {
        Self((value as FullInt * FIXED_DECIMAL) as Int)
    }
}

impl From<i32> for Fixed {
    fn from(value: i32) -> Self {
        Self((value as FullInt * FIXED_DECIMAL) as Int)
    }
}

impl FromLossy<i64> for Fixed {
    fn from_lossy(value: i64) -> Self {
        Self::from(value)
    }
}

impl FromLossy<i32> for Fixed {
    fn from_lossy(value: i32) -> Self {
        Self::from(value)
    }
}

impl FromLossy<f64> for Fixed {
    fn from_lossy(value: f64) -> Self {
        Self::from(value)
    }
}

impl FromLossy<f32> for Fixed {
    fn from_lossy(value: f32) -> Self {
        Self::from(value)
    }
}

impl Neg for Fixed {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<F> Approximately<F> for Fixed where F: Into<Self> {
    fn approximately(&self, other: F, _: crate::Float) -> bool {
        //return i128::abs(self.full().0 - other.into().full().0) <= 2;
        self.0 == other.into().0
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
fixed_binop!(Fixed, &Fixed, add, Add);
fixed_binop!(&Fixed, Fixed, add, Add);
fixed_binop!(&Fixed, &Fixed, add, Add)
;
fixed_binop!(Fixed, Fixed, sub, Sub);
fixed_binop!(Fixed, &Fixed, sub, Sub);
fixed_binop!(&Fixed, Fixed, sub, Sub);
fixed_binop!(&Fixed, &Fixed, sub, Sub);

fixed_binop!(Fixed, Fixed, mul, Mul);
fixed_binop!(Fixed, &Fixed, mul, Mul);
fixed_binop!(&Fixed, Fixed, mul, Mul);
fixed_binop!(&Fixed, &Fixed, mul, Mul);

fixed_binop!(Fixed, Fixed, div, Div);
fixed_binop!(Fixed, &Fixed, div, Div);
fixed_binop!(&Fixed, Fixed, div, Div);
fixed_binop!(&Fixed, &Fixed, div, Div);

impl Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Float::div(self.0 as Float, FIXED_DECIMAL as Float))
    }
}

macro_rules! fixed_assignment_op {
    ($lhs:ty, $rhs:ty, $func:ident, $trait:ident) => {
        impl $trait<$rhs> for $lhs {
            fn $func(&mut self, other: $rhs) {
                let mut lhs = self.full();
                let rhs = other.full();
                FullFixed::$func(&mut lhs, rhs);
                *self = Fixed(lhs.0 as i64);
            }
        }
    };
}

fixed_assignment_op!(Fixed, Fixed, add_assign, AddAssign);
fixed_assignment_op!(Fixed, Fixed, sub_assign, SubAssign);
fixed_assignment_op!(Fixed, Fixed, mul_assign, MulAssign);
fixed_assignment_op!(Fixed, Fixed, div_assign, DivAssign);

#[derive(Debug, Clone, Copy)]
struct FullFixed(pub FullInt);

impl From<Fixed> for FullFixed {
    fn from(value: Fixed) -> Self {
        FullFixed(value.0 as i128)
    }
}

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

macro_rules! fullfixed_assignment_op {
    ($lhs:ty, $rhs:ty, $func:ident, $trait:ident) => {
        impl $trait<$rhs> for $lhs {
            fn $func(&mut self, other: $rhs) {
                dbg!(FullInt::$func(&mut self.0, other.0));
            }
        }
    };
}

fullfixed_assignment_op!(FullFixed, FullFixed, add_assign, AddAssign);
fullfixed_assignment_op!(FullFixed, FullFixed, sub_assign, SubAssign);

impl MulAssign for FullFixed {
    fn mul_assign(&mut self, rhs: Self) {
        *self = FullFixed::from(*self * rhs)
    }
}

impl DivAssign for FullFixed {
    fn div_assign(&mut self, rhs: Self) {
        *self = FullFixed::from(*self / rhs)
    }
}

#[cfg(feature = "fixed_precision")]
const fn const_round_to_decimal_point(x: Float) -> Float {
    let scaled = x * FIXED_DECIMAL as Float;
    let rounded = if scaled >= 0.0 {
        (scaled + 0.5) as i64
    } else {
        (scaled - 0.5) as i64
    };
    rounded as Float / FIXED_DECIMAL as Float
}

#[cfg(test)]
mod fixedpoint_tests {
    use super::*;

    #[test]
    fn from_float() {
        let float_val = 132.1139;
        let fixed_val = Fixed::from(float_val);
        let expected = (float_val * FIXED_DECIMAL as f64).round() as Int;

        debug_assert_eq!(expected, fixed_val.0)
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

