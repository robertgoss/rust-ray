use std::fs::File;
use std::io::{Error, Write};
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_ppm_colour(
    file : &mut File,
    colour : &Colour
) -> Result<(), Error> {
    let range = Interval {min : 0.0, max : 1.0};
    let b_red = (range.clamp(colour.x()) * 256.0) as u8;
    let b_green = (range.clamp(colour.y()) * 256.0) as u8;
    let b_blue = (range.clamp(colour.z()) * 256.0) as u8;
    file.write(b_red.to_string().as_bytes())?;
    file.write(" ".as_bytes())?;
    file.write(b_green.to_string().as_bytes())?;
    file.write(" ".as_bytes())?;
    file.write(b_blue.to_string().as_bytes())?;
    file.write("\n".as_bytes())?;
    Ok(())
}