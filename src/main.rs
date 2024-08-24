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
mod textures;
mod perlin;

use std::env::args;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use crate::camera::Camera;
use crate::colour::{random_colour_light, random_colour_sq, Colour};
use crate::hittables::{HittableList, Sphere, BVH};
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::textures::{Checker, ImageTexture, PerlinTexture, SolidColour, TextureWorld};
use crate::vec3::{Point3, Vec3};


fn random_material<'tex>(rng : &mut ThreadRng, textures : &'tex TextureWorld<'tex>) -> Box<dyn Material + 'tex> {
    let choice = rng.gen::<f64>();
    if choice < 0.8 {
        Box::new(Lambertian::new(textures.chose("dark", rng).unwrap()))
    } else if choice < 0.95 {
        let fuzz = rng.gen::<f64>() * 0.5;
        Box::new(Metal::new(textures.chose("light", rng).unwrap(), fuzz))
    } else {
        Box::new(Dielectric::new(1.5))
    }
}

fn random_small_center(rng : &mut ThreadRng, i : i64, j : i64) -> Point3 {
    let x = (i as f64) + rng.gen::<f64>() * 0.9;
    let z = (j as f64) + rng.gen::<f64>() * 0.9;
    Point3::new(x, 0.2, z)
}

fn many_spheres_scene(image_file : &str) {
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width : u32 = 400;
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

    // Make textures
    let mut textures = TextureWorld::new();
    for _ in 0..small_sphere_num_side * 4 {
        textures.add("dark", Box::new(SolidColour::new(&random_colour_sq(&mut rng))))
    }
    for _ in 0..small_sphere_num_side * 4 {
        textures.add("light", Box::new(SolidColour::new(&random_colour_light(&mut rng))))
    }

    // Make materials
    let ground_light = SolidColour::new(&Colour::new(0.9,0.9,0.9));
    let ground_dark = SolidColour::new(&Colour::new(0.2,0.3,0.1));
    let ground_texture = Checker::new(0.32, &ground_dark, &ground_light);
    let material_ground = Lambertian::new(&ground_texture);
    let mut small_sphere_materials : Vec<Box<dyn Material>> = Vec::new();
    for _ in 0..small_sphere_num_total {
        small_sphere_materials.push(random_material(&mut rng, &textures));
    }
    let sphere_material1 = Dielectric::new(1.5);
    let texture2 = SolidColour::new(&Colour::new(0.4, 0.2, 0.1));
    let sphere_material2 = Lambertian::new(&texture2);
    let texture3 = SolidColour::new(&Colour::new(0.7, 0.6, 0.5));
    let sphere_material3 = Metal::new(&texture3, 0.0);

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
    camera.render(image_file, &ordered_world)
}

fn checkered_spheres(image_file : &str) {
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width : u32 = 400;
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
    // Make checker
    let light = SolidColour::new(&Colour::new(0.9,0.9,0.9));
    let dark = SolidColour::new(&Colour::new(0.2,0.3,0.1));
    let checker_texture = Checker::new(0.32, &dark, &light);
    let checker_material = Lambertian::new(&checker_texture);
    // Make world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -10.0, 0.0), 10.0, &checker_material)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 10.0, 0.0), 10.0, &checker_material)));

    // Render
    camera.render(image_file, &world);
}

fn earth(image_file : &str) {
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width : u32 = 400;
    let samples_per_pixel = 200;
    let max_depth : u8 = 50;
    let fov : f64 = 20.0;
    let camera = Camera::new(
        &Point3::new(0.0, 0.0, 12.0),
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
    // Make image
    let globe_texture = ImageTexture::load("earthmap.jpg").expect("Could not load texture");
    let globe_material = Lambertian::new(&globe_texture);
    // Make world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 0.0, 0.0), 2.0, &globe_material)));

    // Render
    camera.render(image_file, &world);
}

fn perlin_spheres(image_file : &str) {
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width : u32 = 400;
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
    // Make checker
    let mut rng= thread_rng();
    let noise_texture = PerlinTexture::new(&mut rng, 4.0);
    let noise_material = Lambertian::new(&noise_texture);
    // Make world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, &noise_material)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 2.0, 0.0), 2.0, &noise_material)));

    // Render
    camera.render(image_file, &world);
}


fn main() {
    let scene = args().into_iter().nth(1).unwrap_or("perlin_spheres".to_string());
    let filename = "./renders/".to_string() + &scene + ".png";
    match scene.as_str() {
        "many_spheres" => many_spheres_scene(&filename),
        "checkered_spheres" => checkered_spheres(&filename),
        "earth" => earth(&filename),
        "perlin_spheres" => perlin_spheres(&filename),
        _ => many_spheres_scene(&filename)
    }
}