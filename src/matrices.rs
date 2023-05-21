use std::ops;

// Define the 2 by 2 matrix struct
#[derive(Debug, Default)]
pub struct Mat2x2 {
    matrix: [[f64; 2]; 2],
}

// Define the 3 by 3 matrix struct
#[derive(Debug, Default)]
pub struct Mat3x3 {
    matrix: [[f64; 3]; 3],
}

// Define the 4 by 4 matrix struct
#[derive(Debug, Default)]
pub struct Mat4x4 {
    matrix: [[f64; 4]; 4],
}

// Implement constructor and method form Mat2x2
impl Mat2x2 {
    /// Create a new matrix initialized with 
    /// the elements in the float array
    pub fn new(matrix: [[f64; 2]; 2]) -> Self {
        Self { matrix }
    }

    /// Create a 3 by 3 identity matrix
    pub fn indentity() -> Self {
        Self::new([
            [1., 0.],
            [0., 1.],
        ])
    }
}

// Implement constructor and method form Mat3x3
impl Mat3x3 {
    /// Create a new matrix initialized with 
    /// the elements in the float array
    pub fn new(matrix: [[f64; 3]; 3]) -> Self {
        Self { matrix }
    }

    /// Create a 3 by 3 identity matrix
    pub fn indentity() -> Self {
        Self::new([
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ])
    }
}

// Implement constructor and method form Mat4x4
impl Mat4x4 {
    /// Create a new matrix initialized with 
    /// the elements in the float array
    pub fn new(matrix: [[f64; 4]; 4]) -> Self {
        Self { matrix }
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
}

// Index operator implementations
//
// Implement the index operator for Mat2x2
impl ops::Index<usize> for Mat2x2 {
    type Output = [f64; 2];
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

// Implement the index operator for Mat3x3
impl ops::Index<usize> for Mat3x3 {
    type Output = [f64; 3];
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

// Implement the index operator for Mat4x4
impl ops::Index<usize> for Mat4x4 {
    type Output = [f64; 4];
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}
