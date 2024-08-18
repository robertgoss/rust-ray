use std::fs::File;
use std::io::{Error, Write};
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn attenuate(attenuation : &Colour, colour : &Colour) -> Colour {
    Colour::new(
        attenuation.x() * colour.x(),
        attenuation.y() * colour.y(),
        attenuation.z() * colour.z()
    )
}

fn gamma_correct(linear : f64) -> f64 {
    if linear > 0.0 {
        linear.sqrt()
    } else {
        linear
    }
}

pub fn write_ppm_colour(
    file : &mut File,
    colour : &Colour
) -> Result<(), Error> {
    let range = Interval {min : 0.0, max : 1.0};
    let r = gamma_correct(colour.x());
    let g = gamma_correct(colour.y());
    let b = gamma_correct(colour.z());
    let b_red = (range.clamp(r) * 256.0) as u8;
    let b_green = (range.clamp(g) * 256.0) as u8;
    let b_blue = (range.clamp(b) * 256.0) as u8;
    file.write(b_red.to_string().as_bytes())?;
    file.write(" ".as_bytes())?;
    file.write(b_green.to_string().as_bytes())?;
    file.write(" ".as_bytes())?;
    file.write(b_blue.to_string().as_bytes())?;
    file.write("\n".as_bytes())?;
    Ok(())
}