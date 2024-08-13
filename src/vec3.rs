use std::ops;

pub struct Vec3 {
    coords : [f64; 3]
}

pub type Colour = Vec3;
pub type Point3 = Vec3;


impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3{ coords : [0.0; 3] }
    }
    pub fn new(x : f64, y : f64, z : f64) -> Vec3 {
        Vec3{ coords : [x, y, z] }
    }

    pub fn length_squared(&self) -> f64 {
        self.coords[0] * self.coords[0] +
            self.coords[1] * self.coords[1]+
            self.coords[2] * self.coords[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }
    pub fn y(&self) -> f64 {
        self.coords[1]
    }
    pub fn z(&self) -> f64 {
        self.coords[2]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z())
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z())
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(),
                  self * rhs.y(),
                  self * rhs.z())
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self / rhs.x(),
                  self / rhs.y(),
                  self / rhs.z())
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.coords[0] += rhs.coords[0];
        self.coords[1] += rhs.coords[1];
        self.coords[2] += rhs.coords[2];
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.coords[0] -= rhs.coords[0];
        self.coords[1] -= rhs.coords[1];
        self.coords[2] -= rhs.coords[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.coords[0] *= rhs;
        self.coords[1] *= rhs;
        self.coords[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.coords[0] /= rhs;
        self.coords[1] /= rhs;
        self.coords[2] /= rhs;
    }
}