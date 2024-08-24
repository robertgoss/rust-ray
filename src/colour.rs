use image::{Rgb, RgbImage};
use rand::Rng;
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

pub fn read_colour(
    image : &RgbImage,
    i : u32,
    j : u32
) -> Colour {
    let b_colour = image.get_pixel(i, j);
    let r = b_colour.0[0] as f64 / 255.0;
    let g = b_colour.0[1] as f64 / 255.0;
    let b = b_colour.0[2] as f64 / 255.0;
    Colour::new(r, g, b)

}

pub fn write_colour(
    image : &mut RgbImage,
    i : u32,
    j : u32,
    colour : &Colour
) {
    let range = Interval {min : 0.0, max : 1.0};
    let r = gamma_correct(colour.x());
    let g = gamma_correct(colour.y());
    let b = gamma_correct(colour.z());
    let b_red = (range.clamp(r) * 256.0) as u8;
    let b_green = (range.clamp(g) * 256.0) as u8;
    let b_blue = (range.clamp(b) * 256.0) as u8;
    let rgb = Rgb::from([b_red, b_green, b_blue]);
    image.put_pixel(i, j, rgb);
}

pub fn random_colour_light<R>(rng : &mut R) -> Colour
where R : Rng
{
    let r = (1.0 + rng.gen::<f64>()) * 0.5;
    let g = (1.0 + rng.gen::<f64>()) * 0.5;
    let b = (1.0 + rng.gen::<f64>()) * 0.5;
    Colour::new(r,g,b)
}
pub fn random_colour_sq<R>(rng : &mut R) -> Colour
where R : Rng
{
    let r = rng.gen::<f64>() * rng.gen::<f64>();
    let g = rng.gen::<f64>() * rng.gen::<f64>();
    let b = rng.gen::<f64>() * rng.gen::<f64>();
    Colour::new(r,g,b)
}