use crate::aabb::AABB;
use crate::interval::Interval;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Axis3, Point3, UnitVec3, Vec3};

use ordered_float::NotNan;

pub struct HitRecord<'mat> {
    pub point : Point3,
    pub normal : UnitVec3,
    pub t : f64,
    pub front_face : bool,
    pub material : &'mat dyn Material
}
pub trait Hittable {
    fn hit(&self, ray : &Ray, ray_t : &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

impl<'mat> HitRecord<'mat> {
    fn new(point : &Point3, t : f64, ray : &Ray, outward_normal : &Vec3, material : &'mat dyn Material) -> HitRecord<'mat> {
        if dot(&ray.direction, outward_normal) < 0.0 {
            HitRecord {
                point : *point,
                t,
                normal : *outward_normal,
                front_face : true,
                material
            }
        } else {
            HitRecord {
                point : *point,
                t,
                normal : -outward_normal,
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
    fn hit(&self, ray: &Ray, ray_t : &Interval) -> Option<HitRecord<'mat>> {
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
        Some (
            HitRecord::new(
                &point,
                root,
                ray,
                &normal,
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
    fn hit(&self, ray: &Ray, ray_t : &Interval) -> Option<HitRecord> {
        let mut best_hit : Option<HitRecord> = None;
        let mut current_best = ray_t.max;
        for object in self.objects.iter() {
            let current_ray_t = Interval { min: ray_t.min, max : current_best };
            if let Some(hit) = object.hit(ray, &current_ray_t) {
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let shift = self.direction*ray.time;
        let moved_ray = Ray::new(&(ray.origin - shift), &ray.direction, 0.0);
        self.object.hit(&moved_ray, ray_t).map(
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            BVH::Empty => None,
            BVH::Leaf(aabb, item) => {
                if aabb.hit(ray, ray_t) {
                    item.hit(ray, ray_t)
                } else {
                    None
                }
            }
            BVH::Split(aabb, left, right) => {
                if aabb.hit(ray, ray_t) {
                    if let Some(hit_left) = left.hit(ray, ray_t) {
                        let ray_tl = Interval::new(ray_t.min, hit_left.t);
                        if let Some(hit_right) = right.hit(ray, &ray_tl) {
                            if hit_left.t < hit_right.t {
                                Some(hit_left)
                            } else {
                                Some(hit_right)
                            }
                        } else {
                            Some(hit_left)
                        }
                    } else {
                        right.hit(ray, ray_t)
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