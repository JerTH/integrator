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
pub use plane::Plane;
pub use line::LineSegment;

trait Zero {
    fn zero() -> Self;
}

trait One {
    fn one() -> Self;
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
