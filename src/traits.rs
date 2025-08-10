use crate::Float;
use std::convert::Infallible;
use std::num::NonZeroI32;
use std::num::NonZeroI64;
use std::num::NonZeroU32;
use std::num::NonZeroU64;

pub trait FloatExt {
    const ONE: Self;
    const ZERO: Self;
    const EPSILON: Self;
}

pub trait FromLossy<U> {
    /// Perform a potentially lossy conversion from one type to another
    ///
    /// E.g., when converting a u64 into an f32. The exact conversion is
    /// not specified for most types, except that the "as" keyword is
    /// used for this conversion when possible, the only guarantee is that
    /// some values may incur a loss of information or precision when
    /// converted using this trait
    fn from_lossy(value: U) -> Self;
}

pub trait Zero {
    /// Returns a zero-representation of the item
    fn zero() -> Self;
}

pub trait Approximately<Rhs = Self> {
    /// Test whether one item is approximately equivalent to another item
    fn approximately(&self, other: Rhs, epsilon: Float) -> bool;
}

pub trait Coincident<Rhs = Self> {
    /// Test whether two items are coincident with one another
    fn coincident(&self, other: &Rhs) -> bool;
}

pub trait Distance<Rhs = Self> {
    /// Compute the squared distance between two items
    ///
    /// It is usually enough to just implement this method and use the
    /// auto implementation of [Distance::distance_to]
    fn distance_to_sq(&self, other: &Rhs) -> Float;

    /// Compute the real distance between two items
    ///
    /// Generally speaking this is more expensive than [Distance::distance_to_sq] and
    /// usually involves at least one square root operation
    ///
    /// By default this first calls [Distance::distance_to_sq] and then finds the
    /// square root of the result
    fn distance_to(&self, other: &Rhs) -> Float {
        Float::sqrt(self.distance_to_sq(other))
    }
}

pub trait Parallel<Rhs = Self> {
    /// Test whether two items are parallel to one another
    fn parallel(&self, other: &Rhs) -> bool;
}

pub trait Intersects<Rhs = Self> {
    /// The resulting intersection shape
    ///
    /// For two planes, this would be an [Option<Line>]
    type Intersection;

    /// Test whether two items intersect, without finding the intersection
    fn interesects(&self, other: &Rhs) -> bool;

    /// Compute the resulting intersection of two items.
    ///
    /// In cases where an intersection is not guaranteed, an [Option] or a [Result] is to be returned
    fn intersection(&self, other: &Rhs) -> Self::Intersection;
}

#[allow(private_bounds)]
pub trait Numeric: sealed::SealedNumeric {
    type Error;
    fn into_float(self) -> Float;
    fn try_into_float(&self) -> Result<Float, Self::Error>;

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized;
}

impl Numeric for f64 {
    type Error = Infallible;

    fn into_float(self) -> Float {
        self
    }

    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(*self)
    }

    fn from_float(value: Float) -> Option<Self> {
        Some(value)
    }
}

impl Numeric for std::num::NonZeroU32 {
    type Error = Infallible;

    fn into_float(self) -> Float {
        self.get() as Float
    }

    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(self.get() as Float)
    }

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized,
    {
        if value > u32::MAX as f64 {
            return None;
        }

        NonZeroU32::new(value as u32)
    }
}

impl Numeric for std::num::NonZeroU64 {
    type Error = Infallible;

    /// Lossy conversion
    fn into_float(self) -> Float {
        self.get() as Float
    }

    /// Lossy conversion
    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(f64::from_lossy(self.get()))
    }

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized,
    {
        if value > u64::MAX as f64 {
            return None;
        }

        NonZeroU64::new(value as u64)
    }
}

impl Numeric for std::num::NonZeroI32 {
    type Error = Infallible;

    fn into_float(self) -> Float {
        self.get() as Float
    }

    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(self.get() as Float)
    }

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized,
    {
        if value.abs() > i32::MAX as f64 {
            return None;
        }

        NonZeroI32::new(value as i32)
    }
}

impl Numeric for std::num::NonZeroI64 {
    type Error = Infallible;

    /// Lossy conversion
    fn into_float(self) -> Float {
        self.get() as Float
    }

    /// Lossy conversion
    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(f64::from_lossy(self.get()))
    }

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized,
    {
        if value.abs() > i64::MAX as f64 {
            return None;
        }

        NonZeroI64::new(value as i64)
    }
}

impl Numeric for i8 {
    type Error = Infallible;

    fn into_float(self) -> Float {
        self as Float
    }

    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(f64::from_lossy(*self))
    }

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized,
    {
        if value.abs() > i8::MAX as f64 {
            return None;
        }

        Some(value as i8)
    }
}

impl Numeric for i32 {
    type Error = Infallible;

    fn into_float(self) -> Float {
        self as Float
    }

    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(f64::from_lossy(*self))
    }

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized,
    {
        if value.abs() > i32::MAX as f64 {
            return None;
        }

        Some(value as i32)
    }
}

impl Numeric for u8 {
    type Error = Infallible;

    fn into_float(self) -> Float {
        self as Float
    }

    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(f64::from_lossy(*self))
    }

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized,
    {
        if value.abs() > u8::MAX as f64 {
            return None;
        }

        Some(value as u8)
    }
}

impl Numeric for u32 {
    type Error = Infallible;

    fn into_float(self) -> Float {
        self as Float
    }

    fn try_into_float(&self) -> Result<Float, Self::Error> {
        Ok(f64::from_lossy(*self))
    }

    fn from_float(value: Float) -> Option<Self>
    where
        Self: Sized,
    {
        if value.abs() > i32::MAX as f64 {
            return None;
        }

        Some(value as u32)
    }
}

mod sealed {
    pub(super) trait SealedNumeric {}

    impl SealedNumeric for f64 {}
    impl SealedNumeric for f32 {}

    impl SealedNumeric for u64 {}
    impl SealedNumeric for u32 {}
    impl SealedNumeric for u8 {}

    impl SealedNumeric for i64 {}
    impl SealedNumeric for i32 {}
    impl SealedNumeric for i8 {}

    impl SealedNumeric for std::num::NonZeroU32 {}
    impl SealedNumeric for std::num::NonZeroU64 {}
    impl SealedNumeric for std::num::NonZeroI32 {}
    impl SealedNumeric for std::num::NonZeroI64 {}
}
