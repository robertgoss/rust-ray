use std::cmp::max;
use std::f64::consts::PI;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use image::{RgbImage};

use crate::colour::{attenuate, write_colour, Colour};
use crate::hittables::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{cross, random_in_disc, Point3, Vec3};


pub struct Camera {
    image_width : u32,
    image_height : u32,
    samples_per_pixel : u32,
    center : Point3,
    pixel00_loc : Point3,
    pixel_delta_u : Vec3,
    pixel_delta_v : Vec3,
    defocus_u : Vec3,
    defocus_v : Vec3,
    max_depth : u8,
    background : Colour
}

impl Camera {

    pub fn new(
        look_from : &Point3,
        look_at : &Point3,
        look_up : &Vec3,
        aspect_ratio : f64,
        image_width : u32,
        samples_per_pixel : u32,
        max_depth : u8,
        vertical_fov : f64,
        defocus_distance : f64,
        defocus_angle : f64,
        background : Colour
    ) -> Camera {
        let image_height = max(
            1,
            ((image_width as f64) / aspect_ratio) as u32
        );
        // Setup the camera coords
        let h = (vertical_fov * PI / 360.0).tan();
        let viewport_height = 2.0 * h * defocus_distance;
        let viewport_width = viewport_height * (image_width as f64)/ (image_height as f64);
        let w = (look_from - look_at).unit();
        let u = cross(look_up, &w).unit();
        let v = cross(&w, &u);

        // Convert img coords to view coords
        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;
        // Pixel size in viewport
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);
        // defocus
        let defocus_radius = defocus_distance * (defocus_angle * PI / 180.0).tan();
        let defocus_u = defocus_radius * u;
        let defocus_v = defocus_radius * v;
        // Start point
        let viewport_upper_left = look_from
            - defocus_distance*w - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            center : *look_from,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_u,
            defocus_v,
            max_depth,
            background
        }
    }

    pub fn render<Hit>(&self, image_file : &str, world : &Hit)
        where Hit : Hittable
    {
        let mut rng = thread_rng();
        let pixel_colour_scale = 1.0 / self.samples_per_pixel as f64;
        let mut image = RgbImage::new(self.image_width, self.image_height);
        // Render
        for j in 0..self.image_height {
            print!("\rScanlines remaining: {}       \n", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.ray(&mut rng, i, j);
                    pixel_colour += self.ray_colour(&mut rng, world, &ray, self.max_depth);
                }
                pixel_colour *= pixel_colour_scale;
                write_colour(&mut image, i, j, &pixel_colour);
            }
        }
        image.save(image_file).expect("Unable to write image");
        print!("\rDONE                      ");
    }

    fn ray<R>(&self, rng : &mut R, i : u32, j : u32) -> Ray
      where R : Rng
    {
        let offset = self.offset(rng);
        let u = i as f64 + offset.x();
        let v = j as f64 + offset.y();
        let viewpoint_pt = self.pixel00_loc
            + (u * self.pixel_delta_u)
            + (v * self.pixel_delta_v);
        let origin = self.defocus_disc_sample(rng);
        let time = rng.gen::<f64>();
        Ray::between(&origin, &viewpoint_pt, time)
    }

    fn defocus_disc_sample<R>(&self, rng : &mut R) -> Point3
    where R : Rng {
        let sample = random_in_disc(rng);
        self.center + sample.x() * self.defocus_u + sample.y() * self.defocus_v
    }

    fn offset<R>(&self, rng : &mut R) -> Vec3
    where R : Rng
    {
        let x = rng.gen::<f64>() - 0.5;
        let y = rng.gen::<f64>() - 0.5;
        Vec3::new(x, y, 0.0)
    }

    fn ray_colour<Hit>(&self, rng : &mut ThreadRng, world : &Hit, ray : &Ray, max_depth : u8) -> Colour
    where Hit : Hittable
    {
        if max_depth == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }
        let initial_t = Interval { min: 0.001, max: f64::MAX };
        if let Some(hit) = world.hit(ray, &initial_t, rng) {
            let emission = hit.material.emitted(hit.u, hit.v, &hit.point);
            if let Some((attenuation, scattered_ray)) = hit.material.scatter(rng, ray, &hit) {
                let scattered = self.ray_colour(rng, world, &scattered_ray, max_depth - 1);
                emission + attenuate(&attenuation, &scattered)
            } else {
                emission
            }
        } else {
            self.background
        }
    }
}