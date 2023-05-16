use crate::vectors::Vec4;
use std::ops;

// Overloading assign operator Vec3
//
// Implement AddAssing operator
impl ops::AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;   
        self.y += rhs.y;   
        self.z += rhs.z;   
        self.w += rhs.w;   
    }
}

// Implement AddAssing operator
impl ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;   
        self.y -= rhs.y;   
        self.z -= rhs.z;   
        self.w -= rhs.w;   
    }
}

// Implement AddAssing operator
impl ops::DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;   
        self.y /= rhs.y;   
        self.z /= rhs.z;   
        self.w /= rhs.w;   
    }
}

// Implement AddAssing operator
impl ops::MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;   
        self.y *= rhs.y;   
        self.z *= rhs.z;   
        self.w *= rhs.w;   
    }
}

// Overloading operaor for Vec3 type
//
// Implement Add operator
impl ops::Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

// Implement Sub operator
impl ops::Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

// Implement Mul operator
impl ops::Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

// Implement Div operator
impl ops::Div for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

// Implement Mul operator for scalar
impl ops::Add<f64> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

// Implement Div operator for scalar
impl ops::Sub<f64> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

// Implement Mul operator for scalar
impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

// Implement Div operator for scalar
impl ops::Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

// Implement float trait Vec3
//
// Implement Add by Vec3 for f64
impl ops::Add<Vec4> for f64 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        rhs + self
    }
}

// Implement Sub by Vec3 for f64
impl ops::Sub<Vec4> for f64 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        rhs - self
    }
}

// Implement Mul by Vec3 for f64
impl ops::Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs * self
    }
}

// Implement Div by Vec3 for f64
impl ops::Div<Vec4> for f64 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        rhs / self
    }
}

