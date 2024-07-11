//!
//! Matrices
//! 

use std::ops::{Index, IndexMut, Mul};

use crate::*;
pub const MATRIX_4X4: usize = 4usize;

/// A 4x4 Matrix
pub struct Matrix {
    elements: [[Float; MATRIX_4X4]; MATRIX_4X4],
}

#[rustfmt::skip]
impl Matrix {
    pub fn new(elements: &[[Float; MATRIX_4X4]; MATRIX_4X4]) -> Self {
        Self {
            elements: *elements
        }
    }

    /// Returns the identity matrix where the diagonal elements are [Float::one()]
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

    #[inline]
    pub fn element(&self, row: usize, col: usize) -> Float {
        debug_assert!(row < (MATRIX_4X4 - 1));
        debug_assert!(col < (MATRIX_4X4 - 1));
        self.row(row)[col]
    }

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

    /// Right handed
    pub fn look_at(eye: Point, target: Point, up: Vector) -> Self {
        Self::look_toward(eye, target - eye, up)
    }

    /// Right handed
    pub fn look_toward(eye: Point, direction: Vector, up: Vector) -> Self {
        let f = direction.normalized();
        let s = f.cross(&up).normalized();
        let u = s.cross(&f);

        let eds = -eye.as_vector().dot(&s);
        let edu = -eye.as_vector().dot(&u);
        let edf = eye.as_vector().dot(&f);
        
        Matrix::new(&[
            [s.x, u.x, -f.x, Float::zero()],
            [s.y, u.y, -f.y, Float::zero()],  
            [s.z, u.z, -f.z, Float::zero()],  
            [eds, edu,  edf, Float::one() ],
        ])
    }

    //mat4 MakeProjectionMatrix(float fovy_rads, float s, float near, float far)
    //{
    //    float g = 1.0f / tan(fovy_rads * 0.5);
    //    float k = far / (far - near);
    //
    //    return mat4(g / s,  0.0f,   0.0f,   0.0f,
    //                 0.0f,  g,      0.0f,   0.0f,
    //                 0.0f,  0.0f,   k,      -near * k,
    //                 0.0f,  0.0f,   1.0f,   0.0f);
    //}
    
    pub fn perspective(fovy: Float, aspect: Float, near: Float, far: Option<Float>) -> Self {
        match far {
            Some(far) => {
                let g = 1.0 / Float::tan(fovy * 0.5);
                let k = far / (far - near);
                let nk = -near * k;
                let gs = g / aspect;

                return Self {
                    elements: [
                        [gs , 0.0, 0.0, 0.0],
                        [0.0, g  , 0.0, 0.0],
                        [0.0, 0.0, k  , nk ],
                        [0.0, 0.0, 1.0, 0.0],
                    ]
                }
            },
            None => {
                let g = 1.0 / Float::tan(fovy * 0.5);
                let gs = g / aspect;
                let a = 0.0;
                let b = near;
                return Self {
                    elements: [
                        [gs , 0.0, 0.0, 0.0],
                        [0.0, g  , 0.0, 0.0],
                        [0.0, 0.0, a  , b  ],
                        [0.0, 0.0, 1.0, 0.0],
                    ]
                }
            },
        }
    }

    pub fn cgmath_perspective() -> Self {
        let fovy = 45.0;
        let aspect = 800.0 / 600.0;
        let mat = cgmath::perspective(cgmath::Deg(fovy), aspect, 0.1, 100.0);
        Self::convert_matrix(&mat)
    }

    pub fn look_toward_cgmath(eye: Point, direction: Vector, up: Vector) -> Self {
        let eye = cgmath::Point3 { x: eye.as_vector().x, y: eye.as_vector().y, z: eye.as_vector().z };
        let dir = cgmath::Vector3 { x: direction.x, y: direction.y, z: direction.z };
        let up = cgmath::Vector3 { x: up.x, y: up.y, z: up.z };
        
        let mat = cgmath::Matrix4::look_to_rh(eye, dir, up);
        Self::convert_matrix(&mat)
    }

    fn convert_matrix(mat: &cgmath::Matrix4<Float>) -> Matrix {
        Matrix {
            elements: [
                [mat[0][0], mat[0][1], mat[0][2], mat[0][3]],
                [mat[1][0], mat[1][1], mat[1][2], mat[1][3]],
                [mat[2][0], mat[2][1], mat[2][2], mat[2][3]],
                [mat[3][0], mat[3][1], mat[3][2], mat[3][3]],
            ],
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

impl From<Float> for Matrix {
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
