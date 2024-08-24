// Playing with basic ray tracer in rust based on
// https://raytracing.github.io/books/RayTracingInOneWeekend.html


mod vec3;
mod colour;
mod ray;
mod hittables;
mod interval;
mod camera;
mod materials;
mod aabb;

use std::fs::File;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use crate::camera::Camera;
use crate::colour::{random_colour_light, random_colour_sq, Colour};
use crate::hittables::{HittableList, Sphere, BVH};
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::vec3::{Point3, Vec3};


fn random_material(rng : &mut ThreadRng) -> Box<dyn Material> {
    let choice = rng.gen::<f64>();
    if choice < 0.8 {
        Box::new(Lambertian::new(&random_colour_sq(rng)))
    } else if choice < 0.95 {
        let fuzz = rng.gen::<f64>() * 0.5;
        Box::new(Metal::new(&random_colour_light(rng), fuzz))
    } else {
        Box::new(Dielectric::new(1.5))
    }
}

fn random_small_center(rng : &mut ThreadRng, i : i64, j : i64) -> Point3 {
    let x = (i as f64) + rng.gen::<f64>() * 0.9;
    let z = (j as f64) + rng.gen::<f64>() * 0.9;
    Point3::new(x, 0.2, z)
}

fn main() {
    // Get dimensions
    let aspect_ratio = 16.0 / 9.0;
    let image_width : usize = 400;
    let samples_per_pixel = 200;
    let max_depth : u8 = 50;
    let fov : f64 = 20.0;
    let camera = Camera::new(
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        fov,
        10.0,
        0.6
    );

    let small_sphere_num_side = 11i64;
    let small_sphere_num_total = 4 * small_sphere_num_side * small_sphere_num_side;

    let mut rng : ThreadRng = thread_rng();

    // Make materials
    let material_ground = Lambertian::new(&Colour::new(0.5,0.5,0.5));
    let mut small_sphere_materials : Vec<Box<dyn Material>> = Vec::new();
    for _ in 0..small_sphere_num_total {
        small_sphere_materials.push(random_material(&mut rng));
    }
    let sphere_material1 = Dielectric::new(1.5);
    let sphere_material2 = Lambertian::new(&Colour::new(0.4, 0.2, 0.1));
    let sphere_material3 = Metal::new(&Colour::new(0.7, 0.6, 0.5), 0.0);

    // Make world
    let mut world = HittableList::new();
    // Ground
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, &material_ground)));
    // Small
    let clearing = Point3::new(4.0, 0.2, 0.0);
    let mut centers : Vec<Point3> = Vec::new();
    for i in -small_sphere_num_side..small_sphere_num_side {
        for j in -small_sphere_num_side..small_sphere_num_side {
            let center = random_small_center(&mut rng, i, j);
            if (center - clearing).length() > 0.9 {
                centers.push(center);
            }
        }
    }
    for (center, mat) in centers.iter().zip(small_sphere_materials.iter()) {
        let sphere = Box::new(Sphere::new(&center, 0.2, mat.as_ref()));
        world.add(sphere);
    }

    // Add big spheres
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 1.0, 0.0), 1.0, &sphere_material1)));
    world.add(Box::new(Sphere::new(&Point3::new(-4.0, 1.0, 0.0), 1.0, &sphere_material2)));
    world.add(Box::new(Sphere::new(&Point3::new(4.0, 1.0, 0.0), 1.0, &sphere_material3)));

    let ordered_world = BVH::new(world);

    // Output image
    let mut image_file = File::create("image.ppm").expect("Could not open file");
    camera.render(&mut image_file, &ordered_world)
}
