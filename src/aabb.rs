use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{axes3, Axis3, Point3, Vec3};

#[derive(Clone)]
pub struct AABB {
    intervals : [Interval; 3]
}

impl AABB {
    pub fn empty() -> AABB {
        AABB {
            intervals : [Interval::empty(), Interval::empty(), Interval::empty()]
        }
    }

    pub fn new(x : &Interval, y : &Interval, z : &Interval) -> AABB {
        AABB {
            intervals : [*x, *y, *z]
        }
    }

    pub fn from_points(a : &Point3, b : &Point3) -> AABB {
        AABB {
            intervals :
              [Interval::from_vals(a.x(), b.x()),
               Interval::from_vals(a.y(), b.y()),
               Interval::from_vals(a.z(), b.z())]
        }
    }

    pub fn x(&self) -> &Interval {
        &self.intervals[0]
    }

    pub fn y(&self) -> &Interval {
        &self.intervals[1]
    }

    pub fn z(&self) -> &Interval {
        &self.intervals[2]
    }

    pub fn coord(&self, axis : Axis3) -> &Interval {
        match axis {
            Axis3::X => self.x(),
            Axis3::Y => self.y(),
            Axis3::Z => self.z()
        }
    }

    pub fn hit(&self, ray : &Ray, ray_t: &Interval) -> bool {
        let mut curr_t = *ray_t;
        for axis in axes3() {
            let a_range = self.coord(axis);
            let ad_inv = 1.0 / ray.direction.coord(axis);
            let a_origin = ray.origin.coord(axis);
            let t0 = (a_range.min - a_origin) * ad_inv;
            let t1 = (a_range.max - a_origin) * ad_inv;
            let a_t = Interval::from_vals(t0, t1);
            if let Some(inter_t) = curr_t.intersect(&a_t) {
                curr_t = inter_t;
            } else {
                return false;
            }
        }
        true
    }

    pub fn translate(&self, direction : &Vec3) -> AABB {
        AABB {
            intervals : [
                self.x().translate(direction.x()),
                self.y().translate(direction.y()),
                self.z().translate(direction.z())]

        }
    }

    pub fn union(&self, other : &AABB) -> AABB {
        AABB {
            intervals : [
                self.x().union(&other.x()),
                self.y().union(&other.y()),
                self.z().union(&other.z())]

        }
    }

    pub fn length(&self, axis: Axis3) -> f64 {
        self.coord(axis).length()
    }

    pub fn longest_axis(&self) -> Axis3 {
        if self.length(Axis3::X) > self.length(Axis3::Y) {
            if self.length(Axis3::X) > self.length(Axis3::Z){
                Axis3::X
            } else {
                Axis3::Z
            }
        } else {
            if self.length(Axis3::Y) > self.length(Axis3::Z){
                Axis3::Y
            } else {
                Axis3::Z
            }
        }
    }
}