mod vec3;

use std::fs::File;
use std::io::{Error, Write};

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
    red : f64,
    green : f64,
    blue : f64,
) -> Result<(), Error> {
    let b_red = (red * 255.999) as u8;
    let b_green = (green * 255.999) as u8;
    let b_blue = (blue * 255.999) as u8;
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
            let red = (i as f64) / ((image_width - 1) as f64);
            let green = (j as f64) / ((image_height - 1) as f64);
            let blue = 0.0;
            write_ppm_colour(&mut image_file, red, green, blue).expect("Could not write to file")
        }
    }
    print!("\rDONE                      ");
}
