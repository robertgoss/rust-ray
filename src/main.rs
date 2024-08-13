// Playing with basic ray tracer in rust based on
// https://raytracing.github.io/books/RayTracingInOneWeekend.html


mod vec3;
mod colour;
mod ray;

use std::cmp::max;
use std::fs::File;
use std::io::{Error, Write};
use crate::colour::{Colour, write_ppm_colour};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

fn write_ppm_header(
    file : &mut File,
    image_width : usize,
    image_height : usize
) -> Result<(), Error> {
    file.write("P3\n".as_bytes())?;
    file.write(image_width.to_string().as_bytes())?;
    file.write(" ".as_bytes())?;
    file.write(image_height.to_string().as_bytes())?;
    file.write("\n255\n".as_bytes())?;
    Ok(())
}

// Render ray
fn ray_colour(ray : &Ray) -> Colour {
    let unit_direction = ray.direction().unit();
    let a = 0.5*(unit_direction.y() + 1.0);
    return (1.0-a)*Colour::new(1.0, 1.0, 1.0) + a*Colour::new(0.5, 0.7, 1.0);
}

fn main() {
    // Get dimensions
    let aspect_ratio = 16.0 / 9.0;
    let image_width : usize = 400;
    let image_height = max(
        1,
        ((image_width as f64) / aspect_ratio) as usize
    );

    // Setup the camera coords
    let focal_length = 2.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64)/ (image_height as f64);
    let camera_center = Point3::zero();
    // Convert img corrds to view coords
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    // Pixel size in viewport
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_width as f64);
    // Start point
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Output image
    let mut image_file = File::create("image.ppm").expect("Could not open file");
    write_ppm_header(&mut image_file, image_width, image_height).expect("Could not write header");

    // Render
    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let viewpoint_pt = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray = Ray::between(&camera_center, &viewpoint_pt);
            let colour = ray_colour(&ray);
            write_ppm_colour(&mut image_file, &colour).expect("Could not write to file")
        }
    }
    print!("\rDONE                      ");
}
