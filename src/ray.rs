use crate::vec3::{Vec3, Point3};

pub struct Ray {
    _origin : Point3,
    _direction : Vec3
}

impl Ray {
    pub fn new(origin : &Point3, direction : &Vec3) -> Ray {
        Ray {_origin : *origin, _direction : *direction}
    }

    pub fn at(&self, t : f64) -> Point3 {
        self._origin + (t * self._direction)
    }

    pub fn origin(&self) -> &Point3 {
        &self._origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self._direction
    }
}