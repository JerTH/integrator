//! Fixed-point arithmetic
//!
//! This is not designed to be a fully general purpose fixed point arithmetic system
//! Instead, the goal is to be generally useful for 3D graphics and interactive simulations
//! while being easy to use
//!
//! Fixed point numbers are represented by signed 64 bit integers. During all basic
//! operations they are promoted to signed 128 bit integers before the operation and then
//! truncated back down to 64 bits.
//!
//! Fixed point arithmetic is slower. The primary benefit is a consistent precision across
//! the entire numerical range. The default setting for this implementation offers
//! a precision of 1.0×10^-5 over ±9.223372037×10^13. This is adequate enough to uniformly
//! represent positions of 10μm within a radius of 616AU

use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use f64 as Float;

use serde::Deserialize;
use serde::Serialize;

use crate::traits::Approximately;
use crate::traits::FromLossy;

type Int = i64;
type FullInt = i128;

const FULL_FIXED_PRECISION_MULTIPLIER: FullInt = 10;
pub const FIXED_DECIMAL: FullInt = 100000;
pub const FULL_FIXED_DECIMAL: FullInt = FIXED_DECIMAL * FULL_FIXED_PRECISION_MULTIPLIER;

#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed(pub Int);

impl Fixed {
    #[cfg(feature = "fixed_precision")]
    #[inline(always)]
    pub(crate) const fn from_const(value: Float) -> Self {
        let rounded = const_round_to_decimal_point(value * FIXED_DECIMAL as Float);
        Self(rounded as Int)
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub fn sqrt(self) -> Self {
        let f = Float::from(self);
        Self::from(Float::sqrt(f))
    }

    pub fn powi(self, exp: i32) -> Self {
        Self(Int::pow(self.0, exp.unsigned_abs()))
    }

    pub fn powf(self, exp: Self) -> Self {
        let f = Float::from(self);
        let e = Float::from(exp);
        Self::from(Float::powf(f, e))
    }

    #[inline(always)]
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

impl From<FullFixed> for Fixed {
    fn from(value: FullFixed) -> Self {
        Fixed((value.0 / FULL_FIXED_PRECISION_MULTIPLIER) as Int)
    }
}

impl From<f64> for Fixed {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self((value * FIXED_DECIMAL as f64).round() as Int)
    }
}

impl From<f32> for Fixed {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self((value * FIXED_DECIMAL as f32).round() as Int)
    }
}

impl From<i64> for Fixed {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self((value as FullInt * FIXED_DECIMAL) as Int)
    }
}

impl From<i32> for Fixed {
    #[inline(always)]
    fn from(value: i32) -> Self {
        Self((value as FullInt * FIXED_DECIMAL) as Int)
    }
}

impl FromLossy<i64> for Fixed {
    #[inline(always)]
    fn from_lossy(value: i64) -> Self {
        Self::from(value)
    }
}

impl FromLossy<i32> for Fixed {
    #[inline(always)]
    fn from_lossy(value: i32) -> Self {
        Self::from(value)
    }
}

impl FromLossy<f64> for Fixed {
    #[inline(always)]
    fn from_lossy(value: f64) -> Self {
        Self::from(value)
    }
}

impl FromLossy<f32> for Fixed {
    #[inline(always)]
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

impl<F> Approximately<F> for Fixed
where
    F: Into<Self>,
{
    fn approximately(&self, other: F, epsilon: crate::Float) -> bool {
        let e = Fixed::from(epsilon).0;
        i64::abs(self.0 - other.into().0) <= e
    }
}

impl PartialEq<Float> for Fixed {
    fn eq(&self, other: &Float) -> bool {
        Fixed::from(*other) == *self
    }
}

macro_rules! fixed_binop {
    ($lhs:ty, $rhs:ty, $func:ident, $trait:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = Fixed;
            fn $func(self, other: $rhs) -> Self::Output {
                Fixed::from(FullFixed::$func(
                    FullFixed::from(self),
                    FullFixed::from(other),
                ))
            }
        }
    };
}

fixed_binop!(Fixed, Fixed, add, Add);
fixed_binop!(Fixed, &Fixed, add, Add);
fixed_binop!(&Fixed, Fixed, add, Add);
fixed_binop!(&Fixed, &Fixed, add, Add);
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

impl Debug for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let value: f64 = (*self).into();
        let value = self.0;
        write!(f, "{value:#?}")
    }
}

impl Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", Float::div(self.0 as Float, FIXED_DECIMAL as Float))
        write!(f, "{}", self.0)
    }
}

macro_rules! fixed_assignment_op {
    ($lhs:ty, $rhs:ty, $func:ident, $trait:ident) => {
        impl $trait<$rhs> for $lhs {
            fn $func(&mut self, other: $rhs) {
                let mut lhs = FullFixed::from(*self);
                let rhs = FullFixed::from(other);
                FullFixed::$func(&mut lhs, rhs);
                *self = Fixed::from(lhs);
            }
        }
    };
}

fixed_assignment_op!(Fixed, Fixed, add_assign, AddAssign);
fixed_assignment_op!(Fixed, Fixed, sub_assign, SubAssign);
fixed_assignment_op!(Fixed, Fixed, mul_assign, MulAssign);
fixed_assignment_op!(Fixed, Fixed, div_assign, DivAssign);

#[derive(Debug, Clone, Copy)]
pub(crate) struct FullFixed(pub FullInt);

#[cfg(feature = "fixed_precision")]
impl FullFixed {
    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub fn sqrt(self) -> Self {
        let f = Float::from(self);
        Self::from(Float::sqrt(f))
    }

    pub fn powi(self, exp: i32) -> Self {
        Self(FullInt::pow(self.0, exp.abs() as u32))
    }

    pub fn powf(self, exp: Self) -> Self {
        let f = Float::from(self);
        let e = Float::from(exp);
        Self::from(Float::powf(f, e))
    }

    #[inline(always)]
    pub fn signum(self) -> Self {
        if self.0 >= 0 {
            FullFixed(FIXED_DECIMAL as FullInt)
        } else {
            FullFixed(-FIXED_DECIMAL as FullInt)
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

impl From<&Fixed> for FullFixed {
    #[inline(always)]
    fn from(value: &Fixed) -> Self {
        FullFixed(value.0 as i128 * FULL_FIXED_PRECISION_MULTIPLIER)
    }
}

impl From<Fixed> for FullFixed {
    #[inline(always)]
    fn from(value: Fixed) -> Self {
        Self::from(&value)
    }
}

impl From<Float> for FullFixed {
    #[inline(always)]
    fn from(value: Float) -> Self {
        Self((value * FULL_FIXED_DECIMAL as f64).round() as FullInt)
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
            type Output = Self;
            fn $func(self, other: $rhs) -> Self::Output {
                Self(FullInt::$func(self.0, other.0))
                //Fixed(FullInt::$func(self.0, other.0) as Int)
            }
        }
    };
}

fullfixed_binop!(FullFixed, FullFixed, add, Add);
fullfixed_binop!(FullFixed, FullFixed, sub, Sub);

impl Mul for FullFixed {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(FullInt::div(
            FullInt::mul(self.0, rhs.0),
            FULL_FIXED_DECIMAL,
        ))
    }
}

impl Div for FullFixed {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(FullInt::div(
            FullInt::mul(self.0, FULL_FIXED_DECIMAL),
            rhs.0,
        ))
    }
}

macro_rules! fullfixed_assignment_op {
    ($lhs:ty, $rhs:ty, $func:ident, $trait:ident) => {
        impl $trait<$rhs> for $lhs {
            fn $func(&mut self, other: $rhs) {
                FullInt::$func(&mut self.0, other.0);
            }
        }
    };
}

fullfixed_assignment_op!(FullFixed, FullFixed, add_assign, AddAssign);
fullfixed_assignment_op!(FullFixed, FullFixed, sub_assign, SubAssign);

impl MulAssign for FullFixed {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl DivAssign for FullFixed {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
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

impl From<Fixed> for Float {
    fn from(value: Fixed) -> Self {
        (value.0 as Float) / (FIXED_DECIMAL as Float)
    }
}

impl From<FullFixed> for Float {
    #[inline(always)]
    fn from(value: FullFixed) -> Self {
        (value.0 as Float) / (FULL_FIXED_DECIMAL as Float)
    }
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
