//!
//! Bivector
//! 

use crate::{Float, Vector};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Bivector {
    pub xy: Float,
    pub xz: Float,
    pub yz: Float,
}

impl Bivector {
    #[inline]
    pub fn from_wedge<V>(u: V, v: V) -> Self where V: Into<Vector> {
        let u: Vector = u.into();
        let v: Vector = v.into();
        Self {
            xy: u.x * v.y - u.y * v.x,
            xz: u.x * v.z - u.z * v.x,
            yz: u.y * v.z - u.z * v.y,
        }
    }
    
    #[inline]
    pub fn magnitude_sq(&self) -> Float {
        self.xy * self.xy
    }

    #[inline]
    pub fn magnitude(&self) -> Float {
        self.xy
    }


    // Wedge product
inline Bivector3 Wedge( const Vector3& u, const Vector3& v )
{
	Bivector3 ret(
		u[0]*v[1] - u[1]*v[0], // XY
		u[0]*v[2] - u[2]*v[0], // XZ
		u[1]*v[2] - u[2]*v[1]  // YZ
	);
	return ret;
}

}
