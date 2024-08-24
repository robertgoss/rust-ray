use std::collections::HashMap;
use image::RgbImage;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::colour::{read_colour, Colour};
use crate::interval::Interval;
use crate::perlin::Perlin;
use crate::vec3::Point3;

pub trait Texture {
    fn value(&self, u : f64, v : f64, point : &Point3) -> Colour;
}

pub struct SolidColour {
    colour : Colour
}

impl SolidColour {
    pub fn new(colour : &Colour) -> SolidColour {
        SolidColour { colour : *colour }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _point: &Point3) -> Colour {
        self.colour
    }
}


pub struct Checker<'tex> {
    inv_scale : f64,
    even : &'tex dyn Texture,
    odd : &'tex dyn Texture
}

impl<'tex> Checker<'tex> {
    pub fn new(scale : f64, even : &'tex dyn Texture, odd : &'tex dyn Texture) -> Checker<'tex> {
        Checker {
            inv_scale : 1.0 / scale,
            even,
            odd
        }
    }
}

impl<'tex> Texture for Checker<'tex> {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Colour {
        let x_int = (self.inv_scale * point.x()).floor() as i64;
        let y_int = (self.inv_scale * point.y()).floor() as i64;
        let z_int = (self.inv_scale * point.z()).floor() as i64;
        if (x_int + y_int + z_int) % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}

pub struct ImageTexture {
    data : RgbImage
}

impl ImageTexture {
    pub fn load(name : &str) -> Option<ImageTexture> {
        let path = "./data/".to_string() + name;
        let image = image::open(path).ok()?;
        Some(ImageTexture {
            data: image.into_rgb8()
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: &Point3) -> Colour {
        let unit = Interval::new(0.0, 1.0);
        let local_u = unit.clamp(u);
        let local_v = 1.0 - unit.clamp(v);
        let i = (local_u * (self.data.width() as f64)) as u32;
        let j = (local_v * (self.data.height() as f64)) as u32;
        read_colour(&self.data, i, j)
    }
}


pub struct MarbleTexture {
    scale : f64,
    noise : Perlin
}

impl MarbleTexture {
    pub fn new<R>(rng : &mut R, scale : f64) -> MarbleTexture
    where R : Rng
    {
        MarbleTexture {
            scale,
            noise : Perlin::new(rng)
        }
    }
}

impl Texture for MarbleTexture {
    fn value(&self, _u: f64, _v: f64, point: &Point3) -> Colour {
        let freq = self.scale * point.z() + 10.0 * self.noise.turbulence(point, 7);
        let val = 0.5 * (freq.sin() + 1.0);
        Colour::new(val, val, val)
    }
}

pub struct TextureWorld<'tex> {
    textures : HashMap<String, Vec<Box<dyn Texture + 'tex>>>
}

impl<'tex> TextureWorld<'tex> {

    pub fn new() -> TextureWorld<'tex> {
        TextureWorld {
            textures : HashMap::new()
        }
    }
    pub fn add(&mut self, test_type : &str, texture : Box<dyn Texture + 'tex>) {
        self.textures.entry(test_type.to_string()).or_insert(Vec::new()).push(texture)
    }

    pub fn chose<'a, R>(&'a self, test_type : &str, rng : &mut R) -> Option<&'a dyn Texture>
    where R : Rng
    {
        self.textures.get(test_type).and_then(|textures : &Vec<Box<dyn Texture>>|
            textures.choose(rng)
        ).map(
            |boxed| boxed.as_ref()
        )
    }
}