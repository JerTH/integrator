//!
//! Matrices
//! 

use crate::*;
pub const MATRIX_4X4: usize = 4usize;

/// A 4x4 Matrix
pub struct Matrix {
    elements: [[Float; MATRIX_4X4]; MATRIX_4X4],
}

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
