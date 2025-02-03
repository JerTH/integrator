//!
//! Matrices
//! 

use std::ops::{Index, IndexMut, Mul};

use rotor::Rotor;

use crate::*;
pub const MATRIX_4X4: usize = 4usize;

/// A 4x4 Matrix
/// 
/// Translation matrices will have the following layout:
/// 
///     [1, 0, 0, 0]
///     [0, 1, 0, 0]
///     [0, 0, 1, 0]
///     [X, Y, Z, 1]
/// 
#[derive(Debug, Clone)]
pub struct Matrix {
    elements: [[Float; MATRIX_4X4]; MATRIX_4X4],
}

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
    /// 
    ///     [ ..X.. , 0]
    ///     [ ..Y.. , 0]
    ///     [ ..Z.. , 0]
    ///     [0, 0, 0, 1]
    /// 
    #[inline]
    pub fn from_orientation(orientation: Rotor) -> Self {
        let a = Vector::unit_x().rotated_by(&orientation);
        let b = Vector::unit_y().rotated_by(&orientation);
        let c = Vector::unit_z().rotated_by(&orientation);
        
        Self {
            elements: [
                [a.x, a.y, a.z, 0.0],
                [b.x, b.y, b.y, 0.0],
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
    /// 
    ///     [ ..X.. , 0]
    ///     [ ..Y.. , 0]
    ///     [ ..Z.. , 0]
    ///     [X, Y, Z, 1]
    /// 
    #[inline]
    pub fn from_translation_and_orientation(translation: Point, orientation: Rotor) -> Self {
        let t = translation;
        let mut matrix = Self::from_orientation(orientation);

        matrix[3][0] = t.v.x;
        matrix[3][1] = t.v.y;
        matrix[3][2] = t.v.z;
        matrix[3][3] = 1.0;
        matrix
    }
    
    /// Construct a new translation [Matrix] from a [Vector]
    ///
    /// The resulting [Matrix] has the form:
    /// 
    ///     [0, 0, 0, 0]
    ///     [0, 0, 0, 0]
    ///     [0, 0, 0, 0]
    ///     [X, Y, Z, 1]
    /// 
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
        debug_assert!(row < (MATRIX_4X4 - 1));
        debug_assert!(col < (MATRIX_4X4 - 1));
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
    
    pub fn perspective(fovy: Float, aspect: Float, near: Float, far: Option<Float>) -> Self {
        match far {
            Some(far) => {
                let q = 1.0 / aspect * Float::tan(fovy * 0.5);
                let qq = 1.0 / Float::tan(fovy * 0.5);
                let qqq = far / far - near;
                let ppp = (-far * near) / (far - near);

                return Self {
                    elements: [
                        [q ,  0.0, 0.0, 0.0],
                        [0.0, qq , 0.0, 0.0],
                        [0.0, 0.0, qqq, ppp],
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

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    /// Multiply a [Matrix] by a [Vector] (p' = Mp)
    fn mul(self, rhs: &Vector) -> Self::Output {
        let lhs = self;
        let w = 0.0;
        Vector {
            x: rhs.x * lhs[0][0] + rhs.y * lhs[0][1] + rhs.z * lhs[0][2] + w * lhs[0][3],
            y: rhs.x * lhs[1][0] + rhs.y * lhs[1][1] + rhs.z * lhs[1][2] + w * lhs[1][3],
            z: rhs.x * lhs[2][0] + rhs.y * lhs[2][1] + rhs.z * lhs[2][2] + w * lhs[2][3],
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
    type Output = Vector;

    /// Multiply a [Matrix] by a [Point] (p' = Mp)
    fn mul(self, rhs: &Point) -> Self::Output {
        let rhs = rhs.as_vector();
        let lhs = self;
        let w = 1.0;
        Vector {
            x: rhs.x * lhs[0][0] + rhs.y * lhs[0][1] + rhs.z * lhs[0][2] + w * lhs[0][3],
            y: rhs.x * lhs[1][0] + rhs.y * lhs[1][1] + rhs.z * lhs[1][2] + w * lhs[1][3],
            z: rhs.x * lhs[2][0] + rhs.y * lhs[2][1] + rhs.z * lhs[2][2] + w * lhs[2][3],
        }
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:+.3}, {:+.3}, {:+.3}, {:+.3}]\n", self[0][0], self[0][1], self[0][2], self[0][3])?;
        write!(f, "[{:+.3}, {:+.3}, {:+.3}, {:+.3}]\n", self[1][0], self[1][1], self[1][2], self[1][3])?;
        write!(f, "[{:+.3}, {:+.3}, {:+.3}, {:+.3}]\n", self[2][0], self[2][1], self[2][2], self[2][3])?;
        write!(f, "[{:+.3}, {:+.3}, {:+.3}, {:+.3}]\n", self[3][0], self[3][1], self[3][2], self[3][3])
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::{rotor::Rotor, Approximately, Float, Point, Vector};

    use super::Matrix;

    #[test]
    fn test_point_translation() {
        let position = Point::new(1.0, 2.0, 3.0);
        let orientation = Rotor::from_rotation_between_vectors(Vector::unit_y(), Vector::unit_y());
        let translation_matrix = Matrix::from_translation_and_orientation(position, orientation);
        let point_to_translate = Point::new(2.0, 4.0, 6.0);
        let left_hand = &point_to_translate * &translation_matrix;
        let right_hand = &translation_matrix * &point_to_translate;

        println!("");
        println!("{translation_matrix}");
        println!("P = {point_to_translate}");
        println!("L = {left_hand}");
        println!("R = {right_hand}");
    }

    /// Test functionality similar to creating a camera view matrix for 3D rendering
    #[test]
    fn build_view_matrix() {
        // The position of the camera
        let position = Point::new(0.0, 0.0, 0.0);
        
        // The position of the target
        let target = Point::new(1.0, 1.0, 0.0);
        
        // The vector between the target and the camera
        let target_vector = target - position;

        // An orientation that "points to" the target from the cameras position
        let orientation = Rotor::from_rotation_between_vectors(Vector::unit_y(), target_vector);
        
        // Construct the matrix
        let matrix = Matrix::from_translation_and_orientation(position, orientation);
        let rotated = Vector::unit_y() * &matrix;

        let moved =  target.as_vector() * &matrix.transposed();
        
        println!("");
        println!("position:    {position}");
        println!("target:      {target}");
        println!("vector:      {target_vector}");
        println!("orientation: {orientation}");
        println!("matrix:\n{matrix}");
        println!("r1:          {rotated}");

        println!("constructed: {}", (position + (target_vector.length() * rotated)));
        println!("target:      {}", &target);
        println!("moved:       {}", &moved);
        assert!((position + (target_vector.length() * rotated)).approximately(&target, Float::EPSILON));
    }
}
