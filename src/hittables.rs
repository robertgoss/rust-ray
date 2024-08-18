use crate::interval::Interval;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, UnitVec3, Vec3};

pub struct HitRecord<'mat, R> {
    pub point : Point3,
    pub normal : UnitVec3,
    pub t : f64,
    pub front_face : bool,
    pub material : &'mat dyn Material<R>
}
pub trait Hittable<R> {
    fn hit(&self, ray : &Ray, ray_t : &Interval) -> Option<HitRecord<R>>;
}

impl<'mat, R> HitRecord<'mat, R> {
    fn new(point : &Point3, t : f64, ray : &Ray, outward_normal : &Vec3, material : &'mat dyn Material<R>) -> HitRecord<'mat, R> {
        if dot(ray.direction(), outward_normal) < 0.0 {
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



pub struct Sphere<'mat, R> {
    center : Point3,
    radius : f64,
    material : &'mat (dyn Material<R> + 'mat)
}

impl<'mat, R> Sphere<'mat, R> {
    pub fn new(center : &Point3, radius : f64, material : &'mat (dyn Material<R> + 'mat)) -> Sphere<'mat, R> {
        if radius < 0.0 {
            Sphere { center: *center, radius: 0.0, material}
        } else {
            Sphere { center: *center, radius , material}
        }
    }
}

impl<'mat, R> Hittable<R> for Sphere<'mat, R> {
    fn hit(&self, ray: &Ray, ray_t : &Interval) -> Option<HitRecord<'mat, R>> {
        let oc = self.center - ray.origin();
        // Quad formula
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), &oc);
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
}


pub struct HittableList<'a, R> {
    objects : Vec<Box<dyn Hittable<R> + 'a>>
}

impl<'a, R> HittableList<'a, R> {
    pub fn new() -> HittableList<'a, R> {
        HittableList { objects : Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object : Box<dyn Hittable<R> + 'a>) {
        self.objects.push(object)
    }
}

impl<'a, R> Hittable<R> for HittableList<'a, R> {
    fn hit(&self, ray: &Ray, ray_t : &Interval) -> Option<HitRecord<R>> {
        let mut best_hit : Option<HitRecord<R>> = None;
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
}
