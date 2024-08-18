// Playing with basic ray tracer in rust based on
// https://raytracing.github.io/books/RayTracingInOneWeekend.html


mod vec3;
mod colour;
mod ray;
mod hittables;
mod interval;
mod camera;
mod materials;

use std::fs::File;
use rand::rngs::ThreadRng;
use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittables::{Hittable, HittableList, Sphere};
use crate::materials::{Lambertian, Metal};
use crate::vec3::Point3;

// Render ray

fn main() {
    // Get dimensions
    let aspect_ratio = 16.0 / 9.0;
    let image_width : usize = 400;
    let samples_per_pixel = 100;
    let max_depth : u8 = 50;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);


    let material_ground = Lambertian::new(&Colour::new(0.8,0.8,0.0));
    let material_center = Lambertian::new(&Colour::new(0.1,0.2,0.5));
    let material_left = Metal::new(&Colour::new(0.8,0.8,0.8));
    let material_right = Metal::new(&Colour::new(0.8,0.6,0.2));
    // Make world
    {
        let mut world = HittableList::<ThreadRng>::new();
        world.add(Box::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0, &material_ground)));
        world.add(Box::new(Sphere::new(&Point3::new(0.0, 0.0, -1.2), 0.5, &material_center)));
        world.add(Box::new(Sphere::new(&Point3::new(-1.0, 0.0, -1.0), 0.5, &material_left)));
        world.add(Box::new(Sphere::new(&Point3::new(1.0, 0.0, -1.0), 0.5, &material_right)));

        // Output image
        let mut image_file = File::create("image.ppm").expect("Could not open file");
        camera.render(&mut image_file, &world)
    }
}
