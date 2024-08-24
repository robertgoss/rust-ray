use std::collections::HashMap;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::colour::Colour;
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