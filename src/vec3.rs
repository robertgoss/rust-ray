use std::ops;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vec3 {
    coords : [f64; 3]
}
pub type Point3 = Vec3;
pub type UnitVec3 = Vec3;

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

    pub fn unit(&self) -> Vec3 {
        unit_vector(self)
    }

    pub fn unitize(&mut self) {
        *self = unit_vector(self)
    }

    pub fn near_zero(&self) -> bool {
        self.x().abs() < 1e-8 || self.y().abs() < 1e-8 || self.z().abs() < 1e-8
    }

}

pub fn random_unit<R>(rng : &mut R) -> Vec3
where R : Rng {
    let x = (rng.gen::<f64>() * 2.0) - 1.0;
    let y = (rng.gen::<f64>() * 2.0) - 1.0;
    let z = (rng.gen::<f64>() * 2.0) - 1.0;
    let v = Vec3::new(x,y,z);
    let l = v.length_squared();
    if l > 1.0 {
        random_unit(rng)
    } else {
        v / (l.sqrt())
    }
}

pub fn random_hemisphere<R>(rng : &mut R, normal : &Vec3) -> Vec3
where R : Rng {
    let v = random_unit(rng);
    if dot(&v, normal) > 0.0 {
        v
    } else {
        -v
    }
}

pub fn reflect(v : &Vec3, normal : &UnitVec3) -> Vec3 {
    v - 2.0 * dot(v, normal) * normal
}

pub fn cross(v1 : &Vec3, v2 : &Vec3) -> Vec3 {
    Vec3::new(
        v1.y() * v2.z() - v1.z() * v2.y(),
        v1.z() * v2.x() - v1.x() * v2.z(),
        v1.x() * v2.y() - v1.y() - v2.x())
}

pub fn dot(v1 : &Vec3, v2 : &Vec3) -> f64 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub fn unit_vector(v : &Vec3) -> Vec3 {
    *v / v.length()
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z())
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        &self + rhs
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        &self + &rhs
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z())
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        &self - &rhs
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs,
                  self.y() / rhs,
                  self.z() / rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(),
                  self * rhs.y(),
                  self * rhs.z())
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs,
                  self.y() / rhs,
                  self.z() / rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        -(&self)
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.coords[0] += rhs.coords[0];
        self.coords[1] += rhs.coords[1];
        self.coords[2] += rhs.coords[2];
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.add_assign(&rhs)
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.coords[0] -= rhs.coords[0];
        self.coords[1] -= rhs.coords[1];
        self.coords[2] -= rhs.coords[2];
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.sub_assign(&rhs)
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