use std::ops;
use crate::vectors::{Vec2, Vec3, Vec4};

// Define the 2 by 2 matrix struct
#[derive(Debug, Default, PartialEq)]
pub struct Mat2x2 {
    m: [[f64; 2]; 2],
}

// Define the 3 by 3 matrix struct
#[derive(Debug, Default, PartialEq)]
pub struct Mat3x3 {
    m: [[f64; 3]; 3],
}

// Define the 4 by 4 matrix struct
#[derive(Debug, Default, PartialEq)]
pub struct Mat4x4 {
    m: [[f64; 4]; 4],
}

// Implement constructor and method form Mat2x2
impl Mat2x2 {
    /// Create a new matrix initialized with 
    /// the elements in the float array
    pub fn new(matrix: [[f64; 2]; 2]) -> Self {
        Self { m: matrix }
    }

    /// Create a 3 by 3 identity matrix
    pub fn indentity() -> Self {
        Self::new([
            [1., 0.],
            [0., 1.],
        ])
    }

    /// Return the transposed matrix
    pub fn transpose(&self) -> Self {
        Self::new([
            [self[0][0], self[1][0]],
            [self[0][1], self[1][1]],
        ])
    }

    /// Perform the matrix multiplication between a Vec2 and a Mat2x2
    /// and return the resulting vector2
    pub fn dot_vec(v: &Vec2, m: &Mat2x2) -> Vec2 {
        Vec2::new(
            v.x*m[0][0] + v.y*m[1][0],
            v.x*m[0][1] + v.y*m[1][1],
        )
    }
    
    /// Perform the matrix multiplication of two matrices
    /// and return the reuslt as a new matrix
    pub fn dot(a: &Mat2x2, b: &Mat2x2) -> Self {
        Self::new([[
            a[0][0]*b[0][0] + a[0][1]*b[1][0],
            a[0][0]*b[0][1] + a[0][1]*b[1][1],
        ],[
            a[1][0]*b[0][0] + a[1][1]*b[1][0],
            a[1][0]*b[0][1] + a[1][1]*b[1][1],
        ]])
    }

    /// Perform the matrix multiplication of two matrices
    /// and save the result in a mutable reference to Mat2x2
    pub fn dot_ref(a: &Mat2x2, b: &Mat2x2, out: &mut Mat2x2) {
        out[0][0] = a[0][0]*b[0][0] + a[0][1]*b[1][0];
        out[0][1] = a[0][0]*b[0][1] + a[0][1]*b[1][1];

        out[1][0] = a[1][0]*b[0][0] + a[1][1]*b[1][0];
        out[1][1] = a[1][0]*b[0][1] + a[1][1]*b[1][1];
    }
}

// Implement constructor and method form Mat3x3
impl Mat3x3 {
    /// Create a new matrix initialized with 
    /// the elements in the float array
    pub fn new(matrix: [[f64; 3]; 3]) -> Self {
        Self { m: matrix }
    }

    /// Create a 3 by 3 identity matrix
    pub fn indentity() -> Self {
        Self::new([
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ])
    }
    
    /// Return the transposed matrix
    pub fn transpose(&self) -> Self {
        Self::new([
            [self[0][0], self[1][0], self[2][0]],
            [self[0][1], self[1][1], self[2][1]],
            [self[0][2], self[1][2], self[2][2]],
        ])
    }

    /// Perform the matrix multiplication between a Vec3 and a Mat3x3
    /// and return the resulting vector3
    pub fn dot_vec(v: &Vec3, m: &Mat3x3) -> Vec3 {
        Vec3::new(
            v.x*m[0][0] + v.y*m[1][0] + v.z*m[2][0],
            v.x*m[0][1] + v.y*m[1][1] + v.z*m[2][1],
            v.x*m[0][2] + v.y*m[1][2] + v.z*m[2][2],
        )
    }

    /// Perform the matrix multiplication of two matrices
    /// and return the reuslt as a new matrix
    pub fn dot(a: &Mat3x3, b: &Mat3x3) -> Self {
        Self::new([[
            a[0][0]*b[0][0] + a[0][1]*b[1][0] + a[0][2]*b[2][0],
            a[0][0]*b[0][1] + a[0][1]*b[1][1] + a[0][2]*b[2][1],
            a[0][0]*b[0][2] + a[0][1]*b[1][2] + a[0][2]*b[2][2],
        ],[
            a[1][0]*b[0][0] + a[1][1]*b[1][0] + a[1][2]*b[2][0],
            a[1][0]*b[0][1] + a[1][1]*b[1][1] + a[1][2]*b[2][1],
            a[1][0]*b[0][2] + a[1][1]*b[1][2] + a[1][2]*b[2][2],
        ],[
            a[2][0]*b[0][0] + a[2][1]*b[1][0] + a[2][2]*b[2][0],
            a[2][0]*b[0][1] + a[2][1]*b[1][1] + a[2][2]*b[2][1],
            a[2][0]*b[0][2] + a[2][1]*b[1][2] + a[2][2]*b[2][2],
        ]])
    }

    /// Perform the matrix multiplication of two matrices
    /// and save the result in a mutable reference to Mat3x3
    pub fn dot_ref(a: &Mat3x3, b: &Mat3x3, out: &mut Mat3x3) {
        out[0][0] = a[0][0]*b[0][0] + a[0][1]*b[1][0] + a[0][2]*b[2][0];
        out[0][1] = a[0][0]*b[0][1] + a[0][1]*b[1][1] + a[0][2]*b[2][1];
        out[0][2] = a[0][0]*b[0][2] + a[0][1]*b[1][2] + a[0][2]*b[2][2];

        out[1][0] = a[1][0]*b[0][0] + a[1][1]*b[1][0] + a[1][2]*b[2][0];
        out[1][1] = a[1][0]*b[0][1] + a[1][1]*b[1][1] + a[1][2]*b[2][1];
        out[1][2] = a[1][0]*b[0][2] + a[1][1]*b[1][2] + a[1][2]*b[2][2];

        out[2][0] = a[2][0]*b[0][0] + a[2][1]*b[1][0] + a[2][2]*b[2][0];
        out[2][1] = a[2][0]*b[0][1] + a[2][1]*b[1][1] + a[2][2]*b[2][1];
        out[2][2] = a[2][0]*b[0][2] + a[2][1]*b[1][2] + a[2][2]*b[2][2];
    }
}

// Implement constructor and method form Mat4x4
impl Mat4x4 {
    /// Create a new matrix initialized with 
    /// the elements in the float array
    pub fn new(matrix: [[f64; 4]; 4]) -> Self {
        Self { m: matrix }
    }

    /// Create a 3 by 3 identity matrix
    pub fn indentity() -> Self {
        Self::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }
        
    /// Return the transposed matrix
    pub fn transpose(&self) -> Self {
        Self::new([
            [self[0][0], self[1][0], self[2][0], self[3][0]],
            [self[0][1], self[1][1], self[2][1], self[3][1]],
            [self[0][2], self[1][2], self[2][2], self[3][2]],
            [self[0][3], self[1][3], self[2][3], self[3][3]],
        ])
    }

    /// Perform the matrix multiplication between a Vec4 and a Mat4x4
    /// and return the resulting vector4
    pub fn dot_vec(v: &Vec4, m: &Mat4x4) -> Vec4 {
        Vec4::new(
            v.x*m[0][0] + v.y*m[1][0] + v.z*m[2][0] + v.w*m[3][0],
            v.x*m[0][1] + v.y*m[1][1] + v.z*m[2][1] + v.w*m[3][1],
            v.x*m[0][2] + v.y*m[1][2] + v.z*m[2][2] + v.w*m[3][2],
            v.x*m[0][3] + v.y*m[1][3] + v.z*m[2][3] + v.w*m[3][3],
        )
    }

    /// Perform the matrix multiplication of two matrices
    /// and return the reuslt as a new matrix
    pub fn dot(a: &Mat4x4, b: &Mat4x4) -> Self {
        // Abomination but faster than a for loop
        Self::new([[
            a[0][0]*b[0][0] + a[0][1]*b[1][0] + a[0][2]*b[2][0] + a[0][3]*b[3][0],
            a[0][0]*b[0][1] + a[0][1]*b[1][1] + a[0][2]*b[2][1] + a[0][3]*b[3][1],
            a[0][0]*b[0][2] + a[0][1]*b[1][2] + a[0][2]*b[2][2] + a[0][3]*b[3][2],
            a[0][0]*b[0][3] + a[0][1]*b[1][3] + a[0][2]*b[2][3] + a[0][3]*b[3][3],
        ],[
            a[1][0]*b[0][0] + a[1][1]*b[1][0] + a[1][2]*b[2][0] + a[1][3]*b[3][0],
            a[1][0]*b[0][1] + a[1][1]*b[1][1] + a[1][2]*b[2][1] + a[1][3]*b[3][1],
            a[1][0]*b[0][2] + a[1][1]*b[1][2] + a[1][2]*b[2][2] + a[1][3]*b[3][2],
            a[1][0]*b[0][3] + a[1][1]*b[1][3] + a[1][2]*b[2][3] + a[1][3]*b[3][3],
        ],[
            a[2][0]*b[0][0] + a[2][1]*b[1][0] + a[2][2]*b[2][0] + a[2][3]*b[3][0],
            a[2][0]*b[0][1] + a[2][1]*b[1][1] + a[2][2]*b[2][1] + a[2][3]*b[3][1],
            a[2][0]*b[0][2] + a[2][1]*b[1][2] + a[2][2]*b[2][2] + a[2][3]*b[3][2],
            a[2][0]*b[0][3] + a[2][1]*b[1][3] + a[2][2]*b[2][3] + a[2][3]*b[3][3],
        ],[
            a[3][0]*b[0][0] + a[3][1]*b[1][0] + a[3][2]*b[2][0] + a[3][3]*b[3][0],
            a[3][0]*b[0][1] + a[3][1]*b[1][1] + a[3][2]*b[2][1] + a[3][3]*b[3][1],
            a[3][0]*b[0][2] + a[3][1]*b[1][2] + a[3][2]*b[2][2] + a[3][3]*b[3][2],
            a[3][0]*b[0][3] + a[3][1]*b[1][3] + a[3][2]*b[2][3] + a[3][3]*b[3][3],
        ]])
    }

    /// Perform the matrix multiplication of two matrices
    /// and save the result in a mutable reference to Mat4x4
    pub fn dot_ref(a: &Mat4x4, b: &Mat4x4, out: &mut Mat4x4) {
        // Abomination but faster than a for loop
        out[0][0] = a[0][0]*b[0][0] + a[0][1]*b[1][0] + a[0][2]*b[2][0] + a[0][3]*b[3][0];
        out[0][1] = a[0][0]*b[0][1] + a[0][1]*b[1][1] + a[0][2]*b[2][1] + a[0][3]*b[3][1];
        out[0][2] = a[0][0]*b[0][2] + a[0][1]*b[1][2] + a[0][2]*b[2][2] + a[0][3]*b[3][2];
        out[0][3] = a[0][0]*b[0][3] + a[0][1]*b[1][3] + a[0][2]*b[2][3] + a[0][3]*b[3][3];

        out[1][0] = a[1][0]*b[0][0] + a[1][1]*b[1][0] + a[1][2]*b[2][0] + a[1][3]*b[3][0];
        out[1][1] = a[1][0]*b[0][1] + a[1][1]*b[1][1] + a[1][2]*b[2][1] + a[1][3]*b[3][1];
        out[1][2] = a[1][0]*b[0][2] + a[1][1]*b[1][2] + a[1][2]*b[2][2] + a[1][3]*b[3][2];
        out[1][3] = a[1][0]*b[0][3] + a[1][1]*b[1][3] + a[1][2]*b[2][3] + a[1][3]*b[3][3];

        out[2][0] = a[2][0]*b[0][0] + a[2][1]*b[1][0] + a[2][2]*b[2][0] + a[2][3]*b[3][0];
        out[2][1] = a[2][0]*b[0][1] + a[2][1]*b[1][1] + a[2][2]*b[2][1] + a[2][3]*b[3][1];
        out[2][2] = a[2][0]*b[0][2] + a[2][1]*b[1][2] + a[2][2]*b[2][2] + a[2][3]*b[3][2];
        out[2][3] = a[2][0]*b[0][3] + a[2][1]*b[1][3] + a[2][2]*b[2][3] + a[2][3]*b[3][3];

        out[3][0] = a[3][0]*b[0][0] + a[3][1]*b[1][0] + a[3][2]*b[2][0] + a[3][3]*b[3][0];
        out[3][1] = a[3][0]*b[0][1] + a[3][1]*b[1][1] + a[3][2]*b[2][1] + a[3][3]*b[3][1];
        out[3][2] = a[3][0]*b[0][2] + a[3][1]*b[1][2] + a[3][2]*b[2][2] + a[3][3]*b[3][2];
        out[3][3] = a[3][0]*b[0][3] + a[3][1]*b[1][3] + a[3][2]*b[2][3] + a[3][3]*b[3][3];

    }

}

// Index operator implementations
//
// Implement the index operator for Mat2x2
impl ops::Index<usize> for Mat2x2 {
    type Output = [f64; 2];
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}
// Implement the mutable index operator for Mat2x2
impl ops::IndexMut<usize> for Mat2x2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

// Implement the index operator for Mat3x3
impl ops::Index<usize> for Mat3x3 {
    type Output = [f64; 3];
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}
// Implement the mutable index operator for Mat3x3
impl ops::IndexMut<usize> for Mat3x3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

// Implement the index operator for Mat4x4
impl ops::Index<usize> for Mat4x4 {
    type Output = [f64; 4];
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}
// Implement the mutable index operator for Mat4x4
impl ops::IndexMut<usize> for Mat4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}
