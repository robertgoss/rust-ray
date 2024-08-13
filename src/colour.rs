use std::fs::File;
use std::io::{Error, Write};
use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_ppm_colour(
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