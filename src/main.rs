// Playing with basic ray tracer in rust based on
// https://raytracing.github.io/books/RayTracingInOneWeekend.html


mod vec3;

use std::fs::File;
use std::io::{Error, Write};
use crate::vec3::Colour;

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

fn write_ppm_colour(
    file : &mut File,
    colour : &Colour
) -> Result<(), Error> {
    let b_red = (colour.x() * 255.999) as u8;
    let b_green = (colour.y() * 255.999) as u8;
    let b_blue = (colour.z() * 255.999) as u8;
    file.write(b_red.to_string().as_bytes())?;
    file.write(" ".as_bytes())?;
    file.write(b_green.to_string().as_bytes())?;
    file.write(" ".as_bytes())?;
    file.write(b_blue.to_string().as_bytes())?;
    file.write("\n".as_bytes())?;
    Ok(())
}

fn main() {
    let image_width : usize = 256;
    let image_height : usize = 256;
    let mut image_file = File::create("image.ppm").expect("Could not open file");
    write_ppm_header(&mut image_file, image_width, image_height).expect("Could not write header");
    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let colour = Colour::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0
            );
            write_ppm_colour(&mut image_file, &colour).expect("Could not write to file")
        }
    }
    print!("\rDONE                      ");
}
