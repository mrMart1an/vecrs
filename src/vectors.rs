use rand::Rng;
use rand::thread_rng;

/// Define the Vec4 struct
#[derive(Copy, Clone, PartialEq, Default)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

/// Define the Vec3 struct
#[derive(Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Define the Vec2 struct
#[derive(Copy, Clone, PartialEq, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

// Implement constructor and uitility functions for Vec4
impl Vec4 {
    /// Construct a new Vec4 with the given coordinates
    /// Take 4 float64 as input
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Construct a new Vec4 from a given Vec3 and a scalar for the z axis
    /// Take as input a reference to a Vec2 and a float64
    pub fn from_vec3(v: Vec3, w: f64) -> Self {
        Self { 
            x: v.x,
            y: v.y,
            z: v.z,
            w,
        }
    }

    /// Create a vector with random initialized fields
    /// Each field is initialized with a random f64 in the specified range
    pub fn rand(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();

        Self { 
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
            w: rng.gen_range(min..=max),
        }
    }

    /// Calculate the squared lenght of the vector and return a float
    pub fn lenght_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)
    }

    /// Calculate the lenght of the vector and return a float
    pub fn lenght(&self) -> f64 {
        self.lenght_sq().sqrt()
    }

    /// Return the normalized vector
    /// (Keep direction but make the lenght 1.0)
    pub fn normalize(self) -> Self {
        self / self.lenght()
    }

    /// Return the dot product of two vector
    /// Take a reference to two vectors as input
    pub fn dot(va: &Self, vb: &Self) -> f64 {
        (va.x * vb.x) + 
        (va.y * vb.y) +
        (va.z * vb.z) +
        (va.w * vb.w)
    }
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
    
    /// Create a vector with random initialized fields
    /// Each field is initialized with a random f64 in the specified range
    pub fn rand(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();

        Self { 
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    /// Calculate the squared lenght of the vector and return a float
    pub fn lenght_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    
    /// Calculate the lenght of the vector and return a float
    pub fn lenght(&self) -> f64 {
        self.lenght_sq().sqrt()
    }

    /// Return the normalized vector
    /// (Keep direction but make the lenght 1.0)
    pub fn normalize(self) -> Self {
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
            y: (va.x * vb.z) - (va.z * vb.x), 
            z: (va.x * vb.y) - (va.y * vb.x),
        }
    }
}

// Implement constructor and uitility functions for Vec2
impl Vec2 {
    /// Construct a new Vec3 with the given coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a vector with random initialized fields
    /// Each field is initialized with a random f64 in the specified range
    pub fn rand(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();

        Self { 
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
        }
    }

    /// Calculate the squared lenght of the vector and return a float
    pub fn lenght_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    /// Calculate the lenght of the vector and return a float
    pub fn lenght(&self) -> f64 {
        self.lenght_sq().sqrt()
    }

    /// Return the normalized vector
    ///(Keep direction but make the lenght 1.0)
    pub fn normalize(self) -> Self {
        self / self.lenght()
    }
    
    /// Return the dot product of two vector
    /// Take a reference to a vector as one of the input
    pub fn dot(va: &Self, vb: &Self) -> f64 {
        (va.x * vb.x) + 
        (va.y * vb.y) 
    }
}

// Debug trait Implementation
//
// Implementing Vec4 Debug trait
impl std::fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {}, {}, {}, {} ]", self.x, self.y, self.z, self.w)
    }
}

// Implementing Vec3 Debug trait
impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {}, {}, {} ]", self.x, self.y, self.z)
    }
}

// Implementing Vec2 Debug trait
impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {}, {} ]", self.x, self.y)
    }
}

