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
use crate::hittables::{HittableList, Quadrilateral, Sphere, BVH};
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::textures::{Checker, ImageTexture, MarbleTexture, SolidColour, TextureWorld};
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
        0.6,
        Colour::new(0.7, 0.8, 1.0)
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
        0.6,
        Colour::new(0.7, 0.8, 1.0)
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
        0.6,
        Colour::new(0.7, 0.8, 1.0)
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
        0.05,
        Colour::new(0.7, 0.8, 1.00)
    );
    // Make checker
    let mut rng= thread_rng();
    let noise_texture = MarbleTexture::new(&mut rng, 4.0);
    let noise_material = Lambertian::new(&noise_texture);
    // Make world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, &noise_material)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 2.0, 0.0), 2.0, &noise_material)));

    // Render
    camera.render(image_file, &world);
}

fn quads(image_file : &str) {
    // Camera
    let aspect_ratio = 1.0;
    let image_width : u32 = 400;
    let samples_per_pixel = 200;
    let max_depth : u8 = 50;
    let fov : f64 = 80.0;
    let camera = Camera::new(
        &Point3::new(0.0, 0.0, 9.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        fov,
        10.0,
        0.0,
        Colour::new(0.7, 0.8, 1.0)
    );
    // Make materials
    let left_red_t = SolidColour::new(&Colour::new(1.0, 0.2, 0.2));
    let back_green_t = SolidColour::new(&Colour::new(0.2, 1.0, 0.2));
    let right_blue_t = SolidColour::new(&Colour::new(0.2, 0.2, 0.1));
    let upper_orange_t = SolidColour::new(&Colour::new(1.0, 0.5, 0.0));
    let lower_teal_t = SolidColour::new(&Colour::new(0.2, 0.8, 0.8));

    let left_red_m = Lambertian::new(&left_red_t);
    let back_green_m = Lambertian::new(&back_green_t);
    let right_blue_m= Lambertian::new(&right_blue_t);
    let upper_orange_m = Lambertian::new(&upper_orange_t);
    let lower_teal_m = Lambertian::new(&lower_teal_t);
    // Make world
    let mut world = HittableList::new();
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(-3.0, -2.0, 5.0),
        &Vec3::new(0.0, 0.0, -4.0),
        &Vec3::new(0.0, 4.0, 0.0),
        &left_red_m
    )));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(-2.0, -2.0, 0.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 4.0, 0.0),
        &back_green_m
    )));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(3.0, -2.0, 1.0),
        &Vec3::new(0.0, 0.0, 4.0),
        &Vec3::new(0.0, 4.0, 0.0),
        &right_blue_m
    )));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(-2.0, 3.0, 1.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 4.0),
        &upper_orange_m
    )));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(-2.0, -3.0, 5.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -4.0),
        &lower_teal_m
    )));

    // Render
    camera.render(image_file, &world);
}

fn simple_light(image_file : &str) {
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width : u32 = 400;
    let samples_per_pixel = 500;
    let max_depth : u8 = 50;
    let fov : f64 = 20.0;
    let camera = Camera::new(
        &Point3::new(23.0, 3.0, 6.0),
        &Point3::new(0.0, 2.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        fov,
        10.0,
        0.05,
        Colour::new(0.0, 0.0, 0.0)
    );
    // Make checker
    let mut rng= thread_rng();
    let noise_texture = MarbleTexture::new(&mut rng, 4.0);
    let noise_material = Lambertian::new(&noise_texture);

    // Make light
    let light_texture = SolidColour::new(&Colour::new(4.0, 4.0, 4.0));
    let light_material = DiffuseLight::new(&light_texture);
    // Make world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, &noise_material)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 2.0, 0.0), 2.0, &noise_material)));

    world.add(Box::new(Quadrilateral::new(
        &Point3::new(3.0, 1.0, -2.0),
        &Vec3::new(2.0, 0.0, 0.0),
        &Vec3::new(0.0, 2.0, 0.0),
        &light_material
    )));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 7.0, 0.0), 2.0, &light_material)));
    // Render
    camera.render(image_file, &world);
}

fn cornell_box(image_file : &str) {
    // Camera
    let aspect_ratio = 1.0;
    let image_width : u32 = 600;
    let samples_per_pixel = 400;
    let max_depth : u8 = 50;
    let fov : f64 = 40.0;
    let camera = Camera::new(
        &Point3::new(278.0, 278.0, -800.0),
        &Point3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        fov,
        10.0,
        0.00,
        Colour::new(0.0, 0.0, 0.0)
    );
    // Make materials
    let red = SolidColour::new(&Colour::new(0.65, 0.05, 0.05));
    let white = SolidColour::new(&Colour::new(0.73, 0.73, 0.73));
    let green = SolidColour::new(&Colour::new(0.12, 0.45, 0.15));
    let light = SolidColour::new(&Colour::new(15.0, 15.0, 15.0));

    let red_material = Lambertian::new(&red);
    let white_material = Lambertian::new(&white);
    let green_material = Lambertian::new(&green);
    let light_material = DiffuseLight::new(&light);
    // Make world
    let mut world = HittableList::new();
    // Walls
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        &green_material
    )));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        &red_material
    )));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        &white_material
    )));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(555.0, 555.0, 555.0),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -555.0),
        &white_material
    )));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(0.0, 0.0, 555.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &white_material
    )));
    // Light
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(343.0, 544.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        &light_material
    )));
    // Render
    camera.render(image_file, &world);
}

fn main() {
    let scene = args().into_iter().nth(1).unwrap_or("cornell_box".to_string());
    let filename = "./renders/".to_string() + &scene + ".png";
    match scene.as_str() {
        "many_spheres" => many_spheres_scene(&filename),
        "checkered_spheres" => checkered_spheres(&filename),
        "earth" => earth(&filename),
        "perlin_spheres" => perlin_spheres(&filename),
        "quads" => quads(&filename),
        "simple_light" => simple_light(&filename),
        "cornell_box" => cornell_box(&filename),
        _ => println!("Please enter valid scene name")
    }
}