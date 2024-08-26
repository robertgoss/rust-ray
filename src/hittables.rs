use std::f64::consts::PI;
use crate::aabb::AABB;
use crate::interval::Interval;
use crate::materials::{Isotropic, Material};
use crate::ray::Ray;
use crate::vec3::{cross, dot, Axis3, Point3, UnitVec3, Vec3};

use ordered_float::NotNan;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::textures::Texture;

pub struct HitRecord<'mat> {
    pub point : Point3,
    pub normal : UnitVec3,
    pub t : f64,
    pub u : f64,
    pub v : f64,
    pub front_face : bool,
    pub material : &'mat dyn Material
}
pub trait Hittable {
    fn hit(&self, ray : &Ray, ray_t : &Interval, rng : &mut ThreadRng) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

impl<'mat> HitRecord<'mat> {
    fn new(point : &Point3, t : f64, ray : &Ray, outward_normal : &Vec3, u : f64, v : f64, material : &'mat dyn Material) -> HitRecord<'mat> {
        if dot(&ray.direction, outward_normal) < 0.0 {
            HitRecord {
                point : *point,
                t,
                normal : *outward_normal,
                u,
                v,
                front_face : true,
                material
            }
        } else {
            HitRecord {
                point : *point,
                t,
                normal : -outward_normal,
                u,
                v,
                front_face : false,
                material
            }
        }
    }
}

pub struct Sphere<'mat> {
    center : Point3,
    radius : f64,
    material : &'mat (dyn Material + 'mat)
}

impl<'mat> Sphere<'mat> {
    pub fn new(center : &Point3, radius : f64, material : &'mat (dyn Material + 'mat)) -> Sphere<'mat> {
        if radius < 0.0 {
            Sphere { center: *center, radius: 0.0, material}
        } else {
            Sphere { center: *center, radius , material}
        }
    }
}

impl<'mat> Hittable for Sphere<'mat> {
    fn hit(&self, ray: &Ray, ray_t : &Interval, _rng : &mut ThreadRng) -> Option<HitRecord<'mat>> {
        let oc = self.center - ray.origin;
        // Quad formula
        let a = ray.direction.length_squared();
        let h = dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        // Find nearest root in range
        let mut root = (h-sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h+sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let theta = (-normal.y()).acos();
        let phi = f64::atan2(-normal.z(), normal.x());
        let u = phi / (2.0 * PI) + 0.5;
        let v = theta / PI;
        Some (
            HitRecord::new(
                &point,
                root,
                ray,
                &normal,
                u,
                v,
                self.material
            )
        )
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            &Interval::about(self.center.x(), self.radius),
            &Interval::about(self.center.y(), self.radius),
            &Interval::about(self.center.z(), self.radius)
        )
    }
}

pub struct Quadrilateral<'mat> {
    corner : Point3,
    u : Vec3,
    v : Vec3,
    w : Vec3,
    normal : UnitVec3,
    level : f64,
    material : &'mat (dyn Material + 'mat),
    bounding_box : AABB
}

impl<'mat> Quadrilateral<'mat> {
    pub fn new(corner : &Point3, u : &Vec3, v : &Vec3, material : &'mat (dyn Material + 'mat)) -> Quadrilateral<'mat> {
        let bound1 = AABB::from_points(&corner, &(corner+u));
        let bound2 = AABB::from_points(&(corner+v), &(corner+u+v));
        let mut bound = bound1.union(&bound2);
        let c = cross(u, v);
        let l = c.length_squared();
        let w = c / l;
        let normal = c / l.sqrt();
        let level = dot(&normal, corner);
        bound.pad(0.0001);
        Quadrilateral {
            corner : *corner,
            u : *u,
            v : *v,
            w,
            normal,
            level,
            material,
            bounding_box : bound
        }
    }
}

impl<'mat> Hittable for Quadrilateral<'mat> {
    fn hit(&self, ray: &Ray, ray_t: &Interval, _rng : &mut ThreadRng) -> Option<HitRecord> {
        let denom = dot(&self.normal, &ray.direction);
        if denom.abs() < 1e-8 {
            return None; // Par
        }
        let t = (self.level - dot(&self.normal, &ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }
        let intersection = ray.at(t);
        let hit_vec = intersection - self.corner;
        let u = dot(&self.w, &cross(&hit_vec, &self.v));
        let v = dot(&self.w, &cross(&self.u, &hit_vec));
        let unit = Interval::unit();
        if !unit.contains(u) || !unit.contains(v) {
            return None;
        }
        // Return result
        Some(HitRecord::new(
            &intersection,
            t,
            ray,
            &self.normal,
            u,
            v,
            self.material
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box.clone()
    }
}


pub struct HittableList<'a> {
    objects : Vec<Box<dyn Hittable + 'a>>,
    bounding_box : AABB
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList { objects : Vec::new(), bounding_box : AABB::empty() }
    }

    pub fn add(&mut self, object : Box<dyn Hittable + 'a>) {
        self.bounding_box = self.bounding_box.union(&object.bounding_box());
        self.objects.push(object);
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, ray_t : &Interval, rng : &mut ThreadRng) -> Option<HitRecord> {
        let mut best_hit : Option<HitRecord> = None;
        let mut current_best = ray_t.max;
        for object in self.objects.iter() {
            let current_ray_t = Interval { min: ray_t.min, max : current_best };
            if let Some(hit) = object.hit(ray, &current_ray_t, rng) {
                current_best = hit.t;
                best_hit = Some(hit);
            }
        }
        best_hit
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box.clone()
    }
}

pub fn make_box<'a>(a : &Point3, b : &Point3, mat : &'a dyn Material) -> HittableList<'a> {
    let mut cube = HittableList::new();

    let aabb = AABB::from_points(a, b);
    let min_p = aabb.min_point();
    let max_p = aabb.max_point();

    let dx = Vec3::new(aabb.length(Axis3::X), 0.0, 0.0);
    let dy = Vec3::new(0.0, aabb.length(Axis3::Y),0.0);
    let dz = Vec3::new(0.0, 0.0, aabb.length(Axis3::Z));

    cube.add(Box::new(
        Quadrilateral::new(&min_p, &dx, &dy, mat)
    ));
    cube.add(Box::new(
        Quadrilateral::new(&min_p, &dx, &dz, mat)
    ));
    cube.add(Box::new(
        Quadrilateral::new(&min_p, &dy, &dz, mat)
    ));
    cube.add(Box::new(
        Quadrilateral::new(&max_p, &-dx, &-dy, mat)
    ));
    cube.add(Box::new(
        Quadrilateral::new(&max_p, &-dx, &-dz, mat)
    ));
    cube.add(Box::new(
        Quadrilateral::new(&max_p, &-dy, &-dz, mat)
    ));

    cube
}

pub struct ConstantVolume<'a, 'tex> {
    neg_inv_density : f64,
    boundary : Box<dyn Hittable + 'a>,
    material : Isotropic<'tex>
}

impl<'a, 'tex> ConstantVolume<'a, 'tex> {
    pub fn new(density: f64, boundary: Box<dyn Hittable + 'a>, texture: &'tex dyn Texture) -> ConstantVolume<'a, 'tex> {
        ConstantVolume {
            neg_inv_density : -1.0 / density,
            boundary,
            material : Isotropic::new(texture)
        }
    }
}

impl<'a, 'tex> Hittable for ConstantVolume<'a, 'tex> {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng : &mut ThreadRng) -> Option<HitRecord> {
        if let Some(hit1) = self.boundary.hit(ray, &Interval::universe(), rng) {
            if let Some(hit2) = self.boundary.hit(ray, &Interval::new(hit1.t + 0.001, f64::MAX), rng) {
                let hit_t = Interval::new(hit1.t, hit2.t);
                if let Some(mut vol_t) = ray_t.intersect(&hit_t) {
                    if vol_t.min < 0.0 {
                        vol_t.min = 0.0
                    }
                    let ray_length = ray.direction.length();
                    let dist_in_vol = ray_length * vol_t.length();
                    let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();
                    if hit_distance > dist_in_vol {
                        return None;
                    }
                    let t = vol_t.min + (hit_distance / ray_length);
                    return Some(
                        HitRecord::new(
                            &ray.at(t),
                            t,
                            ray,
                            &Vec3::new(1.0,0.0,0.0),
                            0.0,
                            0.0,
                            &self.material
                        )
                    )
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}

pub struct MovingObject<'a> {
    direction : Vec3,
    object : Box<dyn Hittable + 'a>
}

impl<'a> MovingObject<'a> {
    pub fn new(direction : &Vec3, object : Box<dyn Hittable + 'a>) -> MovingObject<'a> {
        MovingObject {
            direction : *direction,
            object
        }
    }
}

impl<'a> Hittable for MovingObject<'a> {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng : &mut ThreadRng) -> Option<HitRecord> {
        let shift = self.direction*ray.time;
        let moved_ray = Ray::new(&(ray.origin - shift), &ray.direction, 0.0);
        self.object.hit(&moved_ray, ray_t, rng).map(
            |record| {
                let mut record_mut = record;
                record_mut.point = record_mut.point + shift;
                record_mut
            }
        )
    }

    fn bounding_box(&self) -> AABB {
        let start = self.object.bounding_box();
        let end = start.translate(&self.direction);
        start.union(&end)
    }
}



pub struct Translated<'a> {
    direction : Vec3,
    object : Box<dyn Hittable + 'a>
}

impl<'a> Translated<'a> {
    pub fn new(direction : &Vec3, object : Box<dyn Hittable + 'a>) -> Translated<'a> {
        Translated {
            direction : *direction,
            object
        }
    }
}

impl<'a> Hittable for Translated<'a> {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng : &mut ThreadRng) -> Option<HitRecord> {
        let moved_ray = Ray::new(&(ray.origin - self.direction), &ray.direction, ray.time);
        self.object.hit(&moved_ray, ray_t, rng).map(
            |record| {
                let mut record_mut = record;
                record_mut.point = record_mut.point + self.direction;
                record_mut
            }
        )
    }

    fn bounding_box(&self) -> AABB {
        self.object.bounding_box().translate(&self.direction)
    }
}

pub struct RotateY<'a> {
    angle_cos : f64,
    angle_sin : f64,
    object : Box<dyn Hittable + 'a>,
    bounding_box : AABB
}

fn rotate_y_vec(angle_c :f64, angle_s : f64, v : &Vec3) -> Vec3 {
    Vec3::new(
        angle_c * v.x() + angle_s * v.z(),
        v.y(),
        -angle_s * v.x() + angle_c * v.z()
    )
}


impl<'a> RotateY<'a> {
    pub fn new(angle : f64, object : Box<dyn Hittable + 'a>) -> RotateY<'a> {
        let mut aabb = AABB::empty();
        let angle_rad = angle * PI / 180.0;
        let angle_cos = angle_rad.cos();
        let angle_sin = (1.0 - angle_cos * angle_cos).sqrt();
        for point in object.bounding_box().points() {
            aabb.expand(&rotate_y_vec(angle_cos, angle_sin, &point));
        }

        RotateY {
            angle_cos,
            angle_sin,
            object,
            bounding_box : aabb
        }
    }
}

impl<'a> Hittable for RotateY<'a> {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng : &mut ThreadRng) -> Option<HitRecord> {
        let r_origin = rotate_y_vec(self.angle_cos, -self.angle_sin, &ray.origin);
        let r_direction = rotate_y_vec(self.angle_cos, -self.angle_sin, &ray.direction);
        let r_ray = Ray::new(&r_origin, &r_direction, ray.time);
        self.object.hit(&r_ray, ray_t, rng).map(
            |record| {
                let mut record_mut = record;
                record_mut.point = rotate_y_vec(self.angle_cos, self.angle_sin, &record_mut.point);
                record_mut
            }
        )
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box.clone()
    }
}

pub enum BVH<'a> {
    Empty,
    Leaf(AABB, Box<dyn Hittable + 'a>),
    Split(AABB, Box<BVH<'a>>, Box<BVH<'a>>)
}

fn split_axis<'a>(mut items : Vec<Box<dyn Hittable + 'a>>, axis: Axis3) -> (HittableList, HittableList) {
    items.sort_by_cached_key(|item| {
        NotNan::new(item.bounding_box().coord(axis).min).unwrap()
    });
    let mut left : HittableList = HittableList::new();
    let mut right : HittableList = HittableList::new();
    let mid = items.len() / 2;
    for (i, object) in items.into_iter().enumerate() {
        if i < mid {
            left.add(object);
        } else {
            right.add(object);
        }
    }
    (left, right)
}

impl<'a> BVH<'a> {
    pub fn new(mut world : HittableList<'a>) -> BVH<'a> {
        if world.objects.is_empty() {
            return BVH::Empty;
        }
        if world.objects.len() <= 1 {
            return BVH::Leaf(world.bounding_box.clone(), world.objects.pop().unwrap());
        }
        let axis = world.bounding_box.longest_axis();
        let bound = world.bounding_box().clone();
        let (left, right) = split_axis(world.objects, axis);
        BVH::Split(bound, Box::new(BVH::new(left)), Box::new(BVH::new(right)))
    }
}

impl<'a> Hittable for BVH<'a> {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng : &mut ThreadRng) -> Option<HitRecord> {
        match self {
            BVH::Empty => None,
            BVH::Leaf(aabb, item) => {
                if aabb.hit(ray, ray_t) {
                    item.hit(ray, ray_t, rng)
                } else {
                    None
                }
            }
            BVH::Split(aabb, left, right) => {
                if aabb.hit(ray, ray_t) {
                    if let Some(hit_left) = left.hit(ray, ray_t, rng) {
                        let ray_tl = Interval::new(ray_t.min, hit_left.t);
                        if let Some(hit_right) = right.hit(ray, &ray_tl, rng) {
                            if hit_left.t < hit_right.t {
                                Some(hit_left)
                            } else {
                                Some(hit_right)
                            }
                        } else {
                            Some(hit_left)
                        }
                    } else {
                        right.hit(ray, ray_t, rng)
                    }
                } else {
                    None
                }
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        match self {
            BVH::Empty => AABB::empty(),
            BVH::Leaf(aabb, _) => aabb.clone(),
            BVH::Split(aabb, _, _) => aabb.clone()
        }
    }
}