use std::ops;

// Define the Vec3 and Vec2 structs
#[derive(Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

// Implement constructor and uitility functions for Vec3
impl Vec3 {
    /// Construct a new Vec3 with the given coordinates
    /// Take 3 float64 as input
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Construct a new Vec3 from a given Vec2 and a scalar for the z axis
    /// Take as input a reference to a Vec2 and a float64
    pub fn from_vec2(v: &Vec2, z: f64) -> Self {
        Self { 
            x: v.x,
            y: v.y,
            z,
        }
    }

    /// Calculate the lenght of the vector and return a float
    pub fn lenght(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Return the normalized vector
    /// (Keep direction but make the lenght 1.0)
    pub fn normalize(&self) -> Self {
        self / self.lenght()
    }

    /// Return the dot product of two vector
    /// Take a reference to two vectors as input
    pub fn dot(va: &Self, vb: &Self) -> f64 {
        (va.x * vb.x) + 
        (va.y * vb.y) +
        (va.z * vb.z)
    }

    /// Return the cross product of two Vec3
    /// Take a reference to two vectors as input
    pub fn cross(va: &Self, vb: &Self) -> Self {
        Self { 
            x: (va.y * vb.z) - (va.z * vb.y), 
            y: - (va.x * vb.z) + (va.z * vb.x), 
            z: (va.x * vb.y) - (va.y * vb.x),
        }
    }
}

// Implement constructor and uitility functions for Vec3
impl Vec2 {
    /// Construct a new Vec3 with the given coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculate the lenght of the vector and return a float
    pub fn lenght(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Return the normalized vector
    ///(Keep direction but make the lenght 1.0)
    pub fn normalize(&self) -> Self {
        self / self.lenght()
    }
    
    /// Return the dot product of two vector
    /// Take a reference to a vector as one of the input
    pub fn dot(va: &Self, vb: &Self) -> f64 {
        (va.x * vb.x) + 
        (va.y * vb.y) 
    }
}

// Implementing Vec3 Debug trait
impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {}, {}, {} ]", self.x, self.y, self.z)
    }
}

// Implementing Vec3 Debug trait
impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {}, {} ]", self.x, self.y)
    }
}

// Overloading operaor for &Vec3 type
//
// Implement Add operator
impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Implement Sub operator
impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Implement Mul operator
impl ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// Implement Div operator
impl ops::Div for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

// Implement Mul operator for scalar
impl ops::Add<f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

// Implement Div operator for scalar
impl ops::Sub<f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

// Implement Mul operator for scalar
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// Implement Div operator for scalar
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// Overloading operaor for &Vec2 type
//
// Implement Add operator
impl ops::Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Implement Sub operator
impl ops::Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Implement Mul operator
impl ops::Mul for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

// Implement Div operator
impl ops::Div for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

// Implement Add operator for scalar
impl ops::Add<f64> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

// Implement Sub operator for scalar
impl ops::Sub<f64> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

// Implement Mul operator for scalar
impl ops::Mul<f64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Implement Div operator for scalar
impl ops::Div<f64> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// Implement float trait
//
// Implement Add by vector3 for f64
impl ops::Add<&Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        rhs + self
    }
}

// Implement Sub by vector3 for f64
impl ops::Sub<&Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        rhs - self
    }
}

// Implement Mul by vector3 for f64
impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

// Implement Div by vector3 for f64
impl ops::Div<&Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        rhs / self
    }
}

// Implement Add by vector2 for f64
impl ops::Add<&Vec2> for f64 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        rhs + self
    }
}

// Implement Sub by vector2 for f64
impl ops::Sub<&Vec2> for f64 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        rhs - self
    }
}

// Implement Mul by vector2 for f64
impl ops::Mul<&Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        rhs * self
    }
}

// Implement Div by vector2 for f64
impl ops::Div<&Vec2> for f64 {
    type Output = Vec2;

    fn div(self, rhs: &Vec2) -> Self::Output {
        rhs / self
    }
}
