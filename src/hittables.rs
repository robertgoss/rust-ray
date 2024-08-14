use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    pub point : Point3,
    pub normal : Vec3,
    pub t : f64,
    pub front_face : bool
}
pub trait Hittable {
    fn hit(&self, ray : &Ray, ray_tmin : f64, ray_tmax : f64) -> Option<HitRecord>;
}

impl HitRecord {
    fn new(point : &Point3, t : f64, ray : &Ray, outward_normal : &Vec3) -> HitRecord {
        if dot(ray.direction(), outward_normal) < 0.0 {
            HitRecord {
                point : *point,
                t,
                normal : *outward_normal,
                front_face : true
            }
        } else {
            HitRecord {
                point : *point,
                t,
                normal : -outward_normal,
                front_face : false
            }
        }
    }
}



pub struct Sphere {
    center : Point3,
    radius : f64
}

impl Sphere {
    pub fn new(center : &Point3, radius : f64) -> Sphere {
        if radius < 0.0 {
            Sphere { center: *center, radius: 0.0}
        } else {
            Sphere { center: *center, radius }
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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
        if root <= ray_tmin || ray_tmax <= root {
            root = (h+sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
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
                &normal
            )
        )
    }
}


pub struct HittableList {
    objects : Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects : Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object : Box<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut best_hit : Option<HitRecord> = None;
        let mut current_best = ray_tmax;
        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, ray_tmin, current_best) {
                current_best = hit.t;
                best_hit = Some(hit);
            }
        }
        best_hit
    }
}
