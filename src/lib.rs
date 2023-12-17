/// A math library

pub type FType = f64;
pub type IType = i64;

pub mod constant;
pub mod vec;
pub mod plane;
pub mod line;

pub use vec::Vector;
pub use vec::Point;
pub use plane::Plane;
pub use line::LineSegment;
