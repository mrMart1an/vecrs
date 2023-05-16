use crate::vectors::Vec2;
use std::ops;

// Overloading assign operator Vec2
//
// Implement AddAssing operator
impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;   
        self.y += rhs.y;   
    }
}

// Implement AddAssing operator
impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;   
        self.y -= rhs.y;   
    }
}

// Implement AddAssing operator
impl ops::DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;   
        self.y /= rhs.y;   
    }
}

// Implement AddAssing operator
impl ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;   
        self.y *= rhs.y;   
    }
}

// Overloading operaor for Vec2 type
//
// Implement Add operator
impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Implement Sub operator
impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Implement Mul operator
impl ops::Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

// Implement Div operator
impl ops::Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

// Implement Add operator for scalar
impl ops::Add<f64> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

// Implement Sub operator for scalar
impl ops::Sub<f64> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

// Implement Mul operator for scalar
impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Implement Div operator for scalar
impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// Implement float trait Vec2
//
// Implement Add by Vec2 for f64
impl ops::Add<Vec2> for f64 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        rhs + self
    }
}

// Implement Sub by Vec2 for f64
impl ops::Sub<Vec2> for f64 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        rhs - self
    }
}

// Implement Mul by Vec2 for f64
impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

// Implement Div by Vec2 for f64
impl ops::Div<Vec2> for f64 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        rhs / self
    }
}

// Implement Vec2 Neg operator
impl ops::Neg for Vec2  {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}
