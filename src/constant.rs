//! Various useful constants

/// Pi π, 3.141592653589793238462643383279502884197169399375105820974944592...
pub const PI: crate::Float = 3.141592653589793238462643383279502884197169399375105820974944592;

/// e, Euler's Number, 2.718281828459045235360287471352662497757247093699959574966967627...
pub const E: crate::Float = 2.718281828459045235360287471352662497757247093699959574966967627;

/// Phi φ, The Golden Ratio, 1.618033988749894848204586834365638117720309179805762862135448623...
// constfn sqrt would allow this to be: (crate::Float::sqrt(5.0) + 1.0) / 2.0
pub const PHI: crate::Float = 1.618033988749894848204586834365638117720309179805762862135448623;

/// π / 180, Degrees to Radians, 0.017453292519943295769236907684886127134428718885417254560971914...
pub const DEG2RAD: crate::Float = PI / 180.0;

/// 1 / ( π / 180 ), Radians to Degrees, 57.29577951308232087679815481410517033240547246656432154916024386...
pub const RAD2DEG: crate::Float = 1.0 / DEG2RAD;
