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
    /// Computes whether this [Float] is approximately equal to another [Float] using an epsilon
    fn approximately(&self, other: &Self, epsilon: Float) -> bool {
        let a = Float::abs(*self);
        let b = Float::abs(*other);
        let difference = Float::abs(a - b);

        if self == other {
            return true;
        } else if *self == 0.0 || *self == 0.0 || a + b < Self::MIN_POSITIVE {
            return difference < (epsilon * Self::MIN_POSITIVE)
        } else {
            return difference / Self::min(a + b, Self::MAX) < epsilon
        }
    }
}
