//!
//! Matrices
//!
//! This Matrix code takes a Vulkan-centric opinion. Graphics-related methods like perspective(...)
//! are designed to work with the Vulkan graphics library and may not work as expected with other libraries

use std::ops::{Index, IndexMut, Mul};

use rotor::Rotor;

use crate::*;
pub const MATRIX_4X4: usize = 4usize;

/// A 4x4 Matrix
#[derive(Debug, Clone)]
pub struct Matrix {
    elements: [[Float; MATRIX_4X4]; MATRIX_4X4],
}

// rustfmt::skip to retain some deliberate alignment for more clarity
#[rustfmt::skip]
impl Matrix {
    /// Construct a new [Matrix] from raw elements
    pub fn new(elements: &[[Float; MATRIX_4X4]; MATRIX_4X4]) -> Self {
        Self {
            elements: *elements
        }
    }
    
    /// Construct a new identity [Matrix] where the diagonal elements are [Float::one()]
    /// ```
    /// # use integrator::matrix::*;
    /// let identity = Matrix::identity();
    /// assert_eq!(identity.elements(), &[
    ///     [1.0, 0.0, 0.0, 0.0],
    ///     [0.0, 1.0, 0.0, 0.0],
    ///     [0.0, 0.0, 1.0, 0.0],
    ///     [0.0, 0.0, 0.0, 1.0],
    /// ])
    /// ```
    pub fn identity() -> Self {
        Self::one()
    }
    
    /// Construct a new [Matrix] from an orientation [Rotor]
    /// 
    /// The resulting [Matrix] has the form:
    ///     [ ..X.. , 0]
    ///     [ ..Y.. , 0]
    ///     [ ..Z.. , 0]
    ///     [0, 0, 0, 1]
    #[inline]
    pub fn from_orientation(orientation: Rotor) -> Self {
        let a = Vector::unit_x().rotated_by(&orientation);
        let b = Vector::unit_y().rotated_by(&orientation);
        let c = Vector::unit_z().rotated_by(&orientation);
        
        Self {
            elements: [
                [a.x, a.y, a.z, 0.0],
                [b.x, b.y, b.z, 0.0],
                [c.x, c.y, c.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    /// Construct a new [Matrix] from a position [Point] and an orientation [Rotor]
    /// 
    /// This [Matrix] encodes both a translation and a rotation
    /// 
    /// The resulting [Matrix] has the form:
    ///     [ ..X.. , 0]
    ///     [ ..Y.. , 0]
    ///     [ ..Z.. , 0]
    ///     [X, Y, Z, 1]
    #[inline]
    pub fn from_translation_and_orientation(translation: Point, orientation: Rotor) -> Self {
        let t = translation;
        let mut matrix = Self::from_orientation(orientation);

        matrix[3][0] = t.x;
        matrix[3][1] = t.y;
        matrix[3][2] = t.z;
        matrix[3][3] = 1.0;
        matrix
    }
    
    /// Construct a new translation [Matrix] from a [Vector]
    ///
    /// The resulting [Matrix] has the form:
    ///     [0, 0, 0, 0]
    ///     [0, 0, 0, 0]
    ///     [0, 0, 0, 0]
    ///     [X, Y, Z, 1]
    #[inline]
    pub fn from_translation(translation: Vector) -> Self {
        let t = translation;
        Self::new(&[
            [1.0, 0.0, 0.0, t.x],
            [0.0, 1.0, 0.0, t.y],
            [0.0, 0.0, 1.0, t.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[inline]
    pub fn element(&self, row: usize, col: usize) -> Float {
        debug_assert!(row < MATRIX_4X4);
        debug_assert!(col < MATRIX_4X4);
        self.row(row)[col]
    }

    #[inline]
    pub fn elements(&self) -> &[[Float; MATRIX_4X4]; MATRIX_4X4] {
        &self.elements
    }

    #[inline]
    pub fn row(&self, row: usize) -> [Float; MATRIX_4X4] {
        self.elements[row]
    }

    #[inline]
    pub fn col(&self, col: usize) -> [Float; MATRIX_4X4] {
        [
            self.row(0)[col],
            self.row(1)[col],
            self.row(2)[col],
            self.row(3)[col],
        ]
    }

    /// Tranpose this [Matrix] in place
    /// 
    /// ```
    /// # use integrator::Matrix;
    /// let mut m = Matrix::new(&[
    ///     [0.0, 1.0, 2.0, 3.0],
    ///     [0.0, 0.0, 4.0, 5.0],
    ///     [0.0, 0.0, 0.0, 6.0],
    ///     [0.0, 0.0, 0.0, 0.0],
    /// ]);
    /// m.transpose();
    /// assert_eq!(m.elements(), &[
    ///     [0.0, 0.0, 0.0, 0.0],
    ///     [1.0, 0.0, 0.0, 0.0],
    ///     [2.0, 4.0, 0.0, 0.0],
    ///     [3.0, 5.0, 6.0, 0.0],
    /// ]);
    /// ```
    #[inline]
    pub fn transpose(&mut self) {
        {
            // [[a, b, c, d]]
            // --------------
            // [[e, f, g, h],
            //  [i, j, k, l],
            //  [m, n, o, p]] 
            let (a, b) = self.elements.split_at_mut(1);
            std::mem::swap(&mut a[0][1], &mut b[0][0]);
            std::mem::swap(&mut a[0][2], &mut b[1][0]);
            std::mem::swap(&mut a[0][3], &mut b[2][0]);
        }
        {
            // [[a, b, c, d],
            //  [e, f, g, h]]
            // --------------
            // [[i, j, k, l],
            //  [m, n, o, p]]

            let (a, b) = self.elements.split_at_mut(2);
            std::mem::swap(&mut a[1][2], &mut b[0][1]);
            std::mem::swap(&mut a[1][3], &mut b[1][1]);
        }
        {
            // [[a, b, c, d],
            //  [e, f, g, h],
            //  [i, j, k, l]],
            // --------------
            // [[m, n, o, p]]
            let (a, b) = self.elements.split_at_mut(3);
            std::mem::swap(&mut a[2][3], &mut b[0][2]);
        }
    }

    /// Constructs and returns the transpose of this [Matrix]
    pub fn transposed(&self) -> Self {
        let mut transposed = self.clone();
        transposed.transpose();
        transposed
    }

    /// Right handed
    pub fn look_at(eye: Point, target: Point, up: Vector) -> Self {
        Self::look_toward(eye, target - eye, up)
    }
    
    /// Right handed
    pub fn look_toward(eye: Point, direction: Vector, up: Vector) -> Self {
        let f = direction.normalized();
        let s = f.cross(&up).normalized();
        let u = s.cross(&f).normalized();

        let eds = eye.as_vector().dot(&s);
        let edu = eye.as_vector().dot(&u);
        let edf = eye.as_vector().dot(&f);
        
        Matrix::new(&[
            [s.x, u.x, -f.x, Float::zero()],
            [s.y, u.y, -f.y, Float::zero()],  
            [s.z, u.z, -f.z, Float::zero()],  
            [eds, edu,  edf, Float::one() ],
        ])
    }
    
    /*
    Notes:
        Vulkan View Volume: 
            X: [-1.0, 1.0]
            Y: [-1.0, 1.0]
            Z: [ 0.0, 1.0]

            -Y = UP
            -X = LEFT
            Z = FORWARD

        Orthographic View Volume:
            e.g.:
                Left, Bottom, Rear
                Right, Top, Far
    */

    /// Construct a Vulkan orthographic projection matrix
    /// 
    /// We assume:
    ///     right = -left
    ///     top = -bottom
    #[rustfmt::skip]
    pub fn orthographic(fovy: Float, aspect: Float, near: Float, far: Float) -> Self {
        let f = far;
        let n = near;
        let h = 2.0 * n * Float::tan(fovy / 2.0); // Near plane height
        let w = aspect * h; // Near plane width
        let b = n * Float::tan(fovy / 2.0); // Near plane bottom
        let r = (n * w / h) * Float::tan(fovy / 2.0); // Near plane right
        
        Self {
            elements: [
                [1.0/r, 0.0,   0.0,       0.0     ],
                [0.0,   1.0/b, 0.0,       0.0     ],
                [0.0,   0.0,   1.0/(f-n), -n/(f-n)],
                [0.0,   0.0,   0.0,       1.0     ],
            ]
        }
    }

    /// Construct a Vulkan perspective projection matrix
    #[rustfmt::skip]
    pub fn perspective(near: Float, far: Float) -> Self {
        let n = near;
        let f = far;
        Self {
            elements: [
                [n,   0.0, 0.0, 0.0 ],
                [0.0, n,   0.0, 0.0 ],
                [0.0, 0.0, f+n, -f*n],
                [0.0, 0.0, 1.0, 0.0 ],
            ]
        }
    }

    /// Multiplies two matrices and returns the resulting [Matrix]
    /// 
    /// ```
    /// # use integrator::matrix::*;
    /// let a = Matrix::new(&[
    ///     [1.0, 0.0, 0.0, 1.0],
    ///     [0.0, 2.0, 2.0, 0.0],
    ///     [0.0, 3.0, 3.0, 0.0],
    ///     [4.0, 0.0, 0.0, 4.0],
    /// ]);
    /// let b = Matrix::new(&[
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [2.0, 0.0, 0.0, 0.0],
    ///     [3.0, 0.0, 0.0, 0.0],
    ///     [4.0, 0.0, 0.0, 0.0],
    /// ]);
    /// let result = a.product(&b);
    /// assert_eq!(result.elements(), &[
    ///     [5.0 , 2.0, 3.0 , 4.0 ],
    ///     [10.0, 0.0, 0.0 , 0.0 ],
    ///     [15.0, 0.0, 0.0 , 0.0 ],
    ///     [20.0, 8.0, 12.0, 16.0],
    /// ]);
    /// ```
    pub fn product(&self, other: &Self) -> Self {
        let lhs = self;
        let rhs = other;
        let mut out = Matrix::zero();
        for i in 0..MATRIX_4X4 {
            for j in 0..MATRIX_4X4 {
                for k in 0..MATRIX_4X4 {
                    out[i][j] += lhs[i][k] * rhs[k][j]
                }
            } 
        }
        out
    }
}

impl Approximately for Matrix {
    fn approximately(&self, other: Self, epsilon: Float) -> bool {
        for i in 0..MATRIX_4X4 {
            for j in 0..MATRIX_4X4 {
                if !self[i][j].approximately(other[i][j], epsilon) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl From<Float> for Matrix {
    #[rustfmt::skip]
    fn from(value: Float) -> Self {
        Self {
            elements: [
                [value, Float::zero(), Float::zero(), Float::zero()],
                [Float::zero(), value, Float::zero(), Float::zero()],
                [Float::zero(), Float::zero(), value, Float::zero()],
                [Float::zero(), Float::zero(), Float::zero(), value],
            ]
        }
    }
}

impl Zero for Matrix {
    fn zero() -> Self {
        Self::from(Float::zero())
    }
}

impl One for Matrix {
    fn one() -> Self {
        Self::from(Float::one())
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.product(rhs)
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Self) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    #[rustfmt::skip]
    fn mul(self, rhs: &Vector) -> Self::Output {
        // Calculate only 3 components (ignore translation)
        Vector {
            x: rhs.x * self[0][0] + rhs.y * self[0][1] + rhs.z * self[0][2],
            y: rhs.x * self[1][0] + rhs.y * self[1][1] + rhs.z * self[1][2],
            z: rhs.x * self[2][0] + rhs.y * self[2][1] + rhs.z * self[2][2],
        }
    }
}

impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Point> for &Matrix {
    type Output = Point;

    #[rustfmt::skip]
    fn mul(self, rhs: &Point) -> Self::Output {
        // Calculate all 4 components using full matrix
        let rhs = rhs.as_vector();
        let x = rhs.x * self[0][0] + rhs.y * self[0][1] + rhs.z * self[0][2] + self[0][3];
        let y = rhs.x * self[1][0] + rhs.y * self[1][1] + rhs.z * self[1][2] + self[1][3];
        let z = rhs.x * self[2][0] + rhs.y * self[2][1] + rhs.z * self[2][2] + self[2][3];
        let w = rhs.x * self[3][0] + rhs.y * self[3][1] + rhs.z * self[3][2] + self[3][3];

        // Perform perspective divide
        let inv_w = 1.0 / w;
        Point::new(x * inv_w, y * inv_w, z * inv_w)
    }
}

impl Index<usize> for Matrix {
    type Output = [Float; MATRIX_4X4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl Index<usize> for &Matrix {
    type Output = [Float; MATRIX_4X4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl Index<usize> for &mut Matrix {
    type Output = [Float; MATRIX_4X4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl IndexMut<usize> for &mut Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl std::fmt::Display for Matrix {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:+.3}, {:+.3}, {:+.3}, {:+.3}]\n", self[0][0], self[0][1], self[0][2], self[0][3])?;
        write!(f, "[{:+.3}, {:+.3}, {:+.3}, {:+.3}]\n", self[1][0], self[1][1], self[1][2], self[1][3])?;
        write!(f, "[{:+.3}, {:+.3}, {:+.3}, {:+.3}]\n", self[2][0], self[2][1], self[2][2], self[2][3])?;
        write!(f, "[{:+.3}, {:+.3}, {:+.3}, {:+.3}]\n", self[3][0], self[3][1], self[3][2], self[3][3])
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn test_look_at() {
        let eye = Point::origin();
        let target = Point::new(0.0, 0.0, -1.0);
        let up = Vector::unit_y();
        let view = Matrix::look_at(eye, target, up);
        let expected = Matrix::identity();

        assert!(view.approximately(expected, 1e-6));
    }

    #[test]
    fn perspective_aspect_ratio() {
        let fovy = std::f64::consts::FRAC_PI_2 as Float;
        let aspect = 2.0; // 2:1 aspect ratio
        let near = 0.1;
        let far = 100.0;

        let p = Matrix::perspective(near, far);
        let o = Matrix::orthographic(fovy, aspect, near, far);
        let m = p * o;

        let tan_half_fovy = (fovy / 2.0).tan();
        let expected_x = 1.0 / (aspect * tan_half_fovy);
        assert!(expected_x.approximately(m[0][0], 1e-6));
    }

    #[test]
    fn test_finite_perspective_matrix() {
        let near = 2.0;
        let far = 10.0;
        let fovy = PI / 2.0; // 90 degrees
        let aspect = 1.0;

        let o = Matrix::orthographic(fovy, aspect, near, far);
        let p = Matrix::perspective(near, far);
        let m = o * p;

        // Test far plane projects to 1.0
        let point_far = Point::new(0.0, 0.0, far);
        let z_ndc = (&m * &point_far).z;

        assert!(z_ndc.approximately(1.0, 1e-6));

        // Test near plane projects to 0.0
        let point_near = Point::new(0.0, 0.0, near);
        let z_ndc = (&m * &point_near).z;

        assert!(z_ndc.approximately(0.0, 1e-6));

        let tan_half_fov = (fovy / 2.0).tan();
        let expected_m00 = 1.0 / (aspect * tan_half_fov);
        let expected_m11 = 1.0 / tan_half_fov;

        // Check scaling factors
        assert!(m.elements[0][0].approximately(expected_m00, 1e-6));
        assert!(m.elements[1][1].approximately(expected_m11, 1e-6));

        // Check z and w rows
        let expected_m22 = -far / (near - far);
        let expected_m23 = (far * near) / (near - far);

        assert!(m.elements[2][2].approximately(expected_m22, 1e-6));
        assert!(m.elements[2][3].approximately(expected_m23, 1e-6));
        assert!(m.elements[3][2].approximately(1.0, 1e-6));
        assert!(m.elements[3][3].approximately(0.0, 1e-6));
        
        // Check other elements are zero where expected
        assert!(m.elements[0][1].approximately(0.0, 1e-6));
        assert!(m.elements[0][2].approximately(0.0, 1e-6));
        assert!(m.elements[0][3].approximately(0.0, 1e-6));
        assert!(m.elements[1][0].approximately(0.0, 1e-6));
        assert!(m.elements[1][2].approximately(0.0, 1e-6));
        assert!(m.elements[1][3].approximately(0.0, 1e-6));
        assert!(m.elements[2][0].approximately(0.0, 1e-6));
        assert!(m.elements[2][1].approximately(0.0, 1e-6));
        assert!(m.elements[3][0].approximately(0.0, 1e-6));
        assert!(m.elements[3][1].approximately(0.0, 1e-6));
        assert!(m.elements[3][3].approximately(0.0, 1e-6));
    }
}
