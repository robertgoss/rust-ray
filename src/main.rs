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
use crate::aabb::AABB;
use crate::camera::Camera;
use crate::colour::{random_colour_light, random_colour_sq, Colour};
use crate::hittables::{make_box, ConstantVolume, HittableList, MovingObject, Quadrilateral, RotateY, Sphere, Translated, BVH};
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::textures::{Checker, ImageTexture, MarbleTexture, SolidColour, TextureWorld};
use crate::vec3::{Point3, Vec3};


fn random_material<'tex>(rng : &mut ThreadRng, textures : &'tex TextureWorld<'tex>) -> (bool, Box<dyn Material + 'tex>) {
    let choice = rng.gen::<f64>();
    if choice < 0.8 {
        (true, Box::new(Lambertian::new(textures.chose("dark", rng).unwrap())))
    } else if choice < 0.95 {
        let fuzz = rng.gen::<f64>() * 0.5;
        (false, Box::new(Metal::new(textures.chose("light", rng).unwrap(), fuzz)))
    } else {
        (false, Box::new(Dielectric::new(1.5)))
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
        small_sphere_materials.push(random_material(&mut rng, &textures).1);
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

fn bouncing_spheres(image_file : &str) {
    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width : u32 = 800;
    let samples_per_pixel = 400;
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
    let mut small_sphere_materials : Vec<(bool, Box<dyn Material>)> = Vec::new();
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
    for (center, (moving, mat)) in centers.iter().zip(small_sphere_materials.iter()) {
        let sphere = Box::new(Sphere::new(&center, 0.2, mat.as_ref()));
        if *moving {
            let jump = Vec3::new(0.0, rng.gen::<f64>() * 0.4, 0.0);
            world.add(Box::new(MovingObject::new(&jump, sphere)))
        } else {
            world.add(sphere);
        }
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
    let world_ordered = BVH::new(world);
    camera.render(image_file, &world_ordered);
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
    let world_ordered = BVH::new(world);
    camera.render(image_file, &world_ordered);
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
    let world_ordered = BVH::new(world);
    camera.render(image_file, &world_ordered);
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
    let world_ordered = BVH::new(world);
    camera.render(image_file, &world_ordered);
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
    let world_ordered = BVH::new(world);
    camera.render(image_file, &world_ordered);
}

fn cornell_box(image_file : &str) {
    // Camera
    let aspect_ratio = 1.0;
    let image_width : u32 = 600;
    let samples_per_pixel = 500;
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
        &Point3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        &light_material
    )));
    // Boxes
    let box1 = Box::new(make_box(
        &Point3::zero(), &Point3::new(165.0,330.0,165.0), &white_material
    ));
    world.add(Box::new(Translated::new(
        &Vec3::new(265.0,0.0,295.0),
        Box::new(RotateY::new(15.0, box1))
    )));
    let box2 = Box::new(make_box(
        &Point3::zero(), &Point3::new(165.0,165.0,165.0), &white_material
    ));
    world.add(Box::new(Translated::new(
        &Vec3::new(130.0,0.0,65.0),
        Box::new(RotateY::new(-18.0, box2))
    )));
    // Render
    let world_ordered = BVH::new(world);
    camera.render(image_file, &world_ordered);
}

fn cornell_smoke(image_file : &str) {
    // Camera
    let aspect_ratio = 1.0;
    let image_width : u32 = 600;
    let samples_per_pixel = 500;
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
    let pure_white = SolidColour::new(&Colour::new(1.0, 1.0, 1.0));
    let black = SolidColour::new(&Colour::new(0.0, 0.0, 0.0));
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
        &Point3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        &light_material
    )));
    // Boxes
    let box1 = Box::new(make_box(
        &Point3::zero(), &Point3::new(165.0,330.0,165.0), &white_material
    ));
    let moved_box1 = Box::new(Translated::new(
        &Vec3::new(265.0,0.0,295.0),
        Box::new(RotateY::new(15.0, box1))
    ));
    let box2 = Box::new(make_box(
        &Point3::zero(), &Point3::new(165.0,165.0,165.0), &white_material
    ));
    let moved_box2 = Box::new(Translated::new(
        &Vec3::new(130.0,0.0,65.0),
        Box::new(RotateY::new(-18.0, box2))
    ));
    // Smoke
    world.add(Box::new(ConstantVolume::new(0.01, moved_box1, &pure_white)));
    world.add(Box::new(ConstantVolume::new(0.01, moved_box2, &black)));
    // Render
    let world_ordered = BVH::new(world);
    camera.render(image_file, &world_ordered);
}

fn final_scene(image_file : &str, image_width : u32, samples_per_pixel: u32, max_depth : u8) {
    // Camera
    let aspect_ratio = 1.0;
    let fov: f64 = 40.0;
    let camera = Camera::new(
        &Point3::new(478.0, 278.0, -600.0),
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

    let mut rng = thread_rng();

    // Make materials
    let white = SolidColour::new(&Colour::new(0.73, 0.73, 0.73));
    let white_material = Lambertian::new(&white);
    let black = SolidColour::new(&Colour::new(0.0, 0.0, 0.0));
    let smoke_colour = SolidColour::new(&Colour::new(0.2, 0.4, 0.9));
    let ground_colour = SolidColour::new(&Colour::new(0.48, 0.83, 0.53));
    let ground_material = Lambertian::new(&ground_colour);
    let light_colour = SolidColour::new(&Colour::new(7.0,7.0,7.0));
    let light_material = DiffuseLight::new(&light_colour);
    let sphere_colour = SolidColour::new(&Colour::new(0.7,0.3,0.1));
    let sphere_material = Lambertian::new(&sphere_colour);
    let globe_texture = ImageTexture::load("earthmap.jpg").expect("Could not load texture");
    let globe_material = Lambertian::new(&globe_texture);
    let marble_texture = MarbleTexture::new(&mut rng, 0.2);
    let marble_material = Lambertian::new(&marble_texture);
    let metal_colour = SolidColour::new(&Colour::new(0.8, 0.8, 0.9));
    let metal = Metal::new(&metal_colour, 1.0);
    let glass = Dielectric::new(1.5);

    // Make world
    let mut world = HittableList::new();

    // Ground
    let mut ground = HittableList::new();
    let boxes_per_side = 20;
    let tile_width = 100.0;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let x0 = -1000.0 + (i as f64)*tile_width;
            let z0 = -1000.0 + (j as f64)*tile_width;
            let y0 = 0.0;
            let x1 = x0 + tile_width;
            let y1 = 1.0 + rng.gen::<f64>() * 100.0;
            let z1 = z0 + tile_width;
            ground.add(Box::new(make_box(
                &Point3::new(x0,y0,z0), &Point3::new(x1,y1,z1), &ground_material
            )))
        }
    }
    world.add(Box::new(BVH::new(ground)));
    world.add(Box::new(Quadrilateral::new(
        &Point3::new(123.0,554.0,147.0),
        &Vec3::new(300.0,0.0,0.0),
        &Vec3::new(0.0,0.0,265.0),
        &light_material
    )));
    // Spheres
    world.add(Box::new(MovingObject::new(
        &Vec3::new(0.0, 0.0, 30.0),
        Box::new(Sphere::new(&Point3::new(400.0, 400.0, 200.0), 50.0, &sphere_material))
    )));
    world.add(Box::new(
        Sphere::new(&Point3::new(260.0, 150.0, 45.0), 50.0, &glass)
    ));
    world.add(Box::new(
        Sphere::new(&Point3::new(0.0, 150.0, 145.0), 50.0, &metal)
    ));
    world.add(Box::new(
        Sphere::new(&Point3::new(400.0, 200.0, 400.0), 100.0, &globe_material)
    ));
    world.add(Box::new(
        Sphere::new(&Point3::new(220.0,280.0,300.0), 80.0, &marble_material)
    ));
    // Smoke
    let small_boundary = Box::new(Sphere::new(
        &Point3::new(360.0,150.0,145.0),
        70.0,
        &glass
    ));
    world.add(Box::new(ConstantVolume::new(0.2, small_boundary, &smoke_colour)));
    let small_boundary = Box::new(Sphere::new(
        &Point3::new(0.0,0.0,5.0),
        5000.0,
        &glass
    ));
    world.add(Box::new(ConstantVolume::new(0.001, small_boundary, &black)));
    // Small spheres
    let area = AABB::from_points(&Point3::zero(), &Point3::new(165.0,165.0,165.0));
    let mut spheres = HittableList::new();
    for _ in 0..1000 {
        spheres.add(Box::new(Sphere::new(
            &area.random(&mut rng), 10.0, &white_material
        )));
    }
    let ordered_spheres = Box::new(BVH::new(spheres));
    world.add(Box::new(Translated::new(
        &Vec3::new(-100.0,270.0,395.0),
        Box::new(RotateY::new(15.0, ordered_spheres))
    )));


    // Render
    let world_ordered = BVH::new(world);
    camera.render(image_file, &world_ordered);
}

fn main() {
    let scene = args().into_iter().nth(1).unwrap_or("final_scene_fast".to_string());
    let filename = "./renders/".to_string() + &scene + ".png";
    match scene.as_str() {
        "many_spheres" => many_spheres_scene(&filename),
        "bouncing_spheres" => bouncing_spheres(&filename),
        "checkered_spheres" => checkered_spheres(&filename),
        "earth" => earth(&filename),
        "perlin_spheres" => perlin_spheres(&filename),
        "quads" => quads(&filename),
        "simple_light" => simple_light(&filename),
        "cornell_box" => cornell_box(&filename),
        "cornell_smoke" => cornell_smoke(&filename),
        "final_scene_fast" => final_scene(&filename, 400, 250, 4),
        _ => println!("Please enter valid scene name")
    }
}