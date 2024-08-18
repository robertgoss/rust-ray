use rand::Rng;
use crate::colour::Colour;
use crate::hittables::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_unit, reflect};

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
    albedo : Colour
}

impl Metal {
    pub fn new(colour : &Colour) -> Metal {
        Metal { albedo : *colour }
    }
}

impl<R> Material<R> for Metal {
    fn scatter(&self, _rng: &mut R, ray_in: &Ray, hit_record: &HitRecord<R>) -> Option<(Colour, Ray)>
    where
        R: Rng
    {
        let scatter_direction = reflect(ray_in.direction(), &hit_record.normal);
        Some( (self.albedo, Ray::new(&hit_record.point, &scatter_direction)) )
    }
}