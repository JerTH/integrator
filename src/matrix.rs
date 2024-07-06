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

    pub fn identity() -> Self {
        Self::one()
    }

    #[inline]
    pub fn element(&self, row: usize, col: usize) -> Float {
        debug_assert!(row < (MATRIX_4X4 - 1));
        debug_assert!(col < (MATRIX_4X4 - 1));
        self.row(row)[col]
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

    /// Right handed
    pub fn look_at(eye: Point, target: Point, up: Vector) -> Self {
        Self::look_toward(eye, target - eye, up)
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
        let lhs = self;
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
