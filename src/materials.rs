use rand::Rng;
use crate::colour::Colour;
use crate::hittables::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_unit, reflect, refract};

pub trait Material<R> {
    fn scatter(&self, rng : &mut R, ray_in : &Ray, hit_record: &HitRecord<R>) -> Option<(Colour, Ray)>
    where R : Rng;
}

pub struct Lambertian {
    albedo : Colour
}

impl Lambertian {
    pub fn new(colour : &Colour) -> Lambertian {
        Lambertian { albedo : *colour }
    }
}

impl<R> Material<R> for Lambertian {
    fn scatter(&self, rng: &mut R, _ray_in: &Ray, hit_record: &HitRecord<R>) -> Option<(Colour, Ray)>
    where
        R: Rng
    {
        let mut scatter_direction = hit_record.normal + random_unit(rng);
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        Some( (self.albedo, Ray::new(&hit_record.point, &scatter_direction)) )
    }
}


pub struct Metal {
    albedo : Colour,
    fuzz : f64
}

impl Metal {
    pub fn new(colour : &Colour, fuzz : f64) -> Metal {
        if fuzz < 1.0 {
            Metal { albedo: *colour, fuzz }
        } else {
            Metal { albedo: *colour, fuzz : 1.0 }
        }
    }
}

impl<R> Material<R> for Metal {
    fn scatter(&self, rng: &mut R, ray_in: &Ray, hit_record: &HitRecord<R>) -> Option<(Colour, Ray)>
    where
        R: Rng
    {
        let scatter_direction = reflect(ray_in.direction(), &hit_record.normal);
        let fuzzed_scatter_direction = scatter_direction.unit() + self.fuzz * random_unit(rng);
        Some( (self.albedo, Ray::new(&hit_record.point, &fuzzed_scatter_direction)) )
    }
}

pub struct Dielectric {
    refraction_index : f64
}

impl Dielectric {
    pub fn new(refraction_index : f64) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl<R> Material<R> for Dielectric {
    fn scatter(&self, _rng: &mut R, ray_in: &Ray, hit_record: &HitRecord<R>) -> Option<(Colour, Ray)>
    where
        R: Rng
    {
        let ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let scatter_direction = refract(&ray_in.direction().unit(), &hit_record.normal, ratio);
        Some( (
            Colour::new(1.0,1.0,1.0),
            Ray::new(&hit_record.point, &scatter_direction)
        ))
    }
}


