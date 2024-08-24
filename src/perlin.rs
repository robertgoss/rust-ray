use rand::Rng;
use rand::seq::SliceRandom;
use crate::vec3::{dot, random_unit, Point3, Vec3};

const POINT_COUNT : usize = 256;
pub struct Perlin {
    vectors : [Vec3; POINT_COUNT],
    x_permute : [u16; POINT_COUNT],
    y_permute : [u16; POINT_COUNT],
    z_permute : [u16; POINT_COUNT]
}

fn make_permute<R>(rng : &mut R) -> [u16; POINT_COUNT]
where R : Rng {
    let mut permute : [u16; POINT_COUNT] = [0; POINT_COUNT];
    permute.iter_mut().enumerate().for_each(
        |(i, val) | *val = i as u16
    );
    permute.shuffle(rng);
    permute
}

fn split_index(val : f64) -> (usize, usize, f64) {
    let fint = val.floor();
    let rem = val - fint;
    let int = fint as i64;
    ((int as usize) & (POINT_COUNT - 1), ((int+1) as usize) & (POINT_COUNT - 1), rem)
}

fn perlin_interp(verts : &[Vec3; 8], x : f64, y : f64, z : f64) -> f64 {
    let xx = x*x*(3.0-2.0*x);
    let yy = y*y*(3.0-2.0*y);
    let zz = z*z*(3.0-2.0*z);
    xx * yy * zz * dot(&verts[0], &Vec3::new(x-1.0,y-1.0,z-1.0)) +
        xx * yy * (1.0-zz) * dot(&verts[1], &Vec3::new(x-1.0,y-1.0,z)) +
        xx * (1.0-yy) * zz * dot(&verts[2], &Vec3::new(x-1.0,y,z-1.0)) +
        xx * (1.0-yy) * (1.0-zz) * dot(&verts[3], &Vec3::new(x-1.0,y,z)) +
        (1.0-xx) * yy * zz * dot(&verts[4], &Vec3::new(x,y-1.0,z-1.0)) +
        (1.0-xx) * yy * (1.0-zz) * dot(&verts[5], &Vec3::new(x,y-1.0,z)) +
        (1.0-xx) * (1.0-yy) * zz * dot(&verts[6], &Vec3::new(x,y,z-1.0)) +
        (1.0-xx) * (1.0-yy) * (1.0-zz) * dot(&verts[7], &Vec3::new(x,y,z))
}


impl Perlin {
    pub fn new<R>(rng : &mut R) -> Perlin
    where R : Rng
    {
        let mut vectors : [Vec3; POINT_COUNT] = [Vec3::zero(); POINT_COUNT];
        vectors.iter_mut().for_each(
            |val| *val = random_unit(rng)
        );
        let x_permute = make_permute(rng);
        let y_permute = make_permute(rng);
        let z_permute = make_permute(rng);

        Perlin {
            vectors,
            x_permute,
            y_permute,
            z_permute
        }
    }

    pub fn noise(&self, point : &Point3) -> f64 {
        let (x_min, x_max, x_rem) = split_index(point.x());
        let (y_min, y_max, y_rem) = split_index(point.y());
        let (z_min, z_max, z_rem) = split_index(point.z());
        let u = x_rem*x_rem*(3.0-2.0*x_rem);
        let v = y_rem*y_rem*(3.0-2.0*y_rem);
        let w = z_rem*z_rem*(3.0-2.0*z_rem);
        let vecs = [
            self.vector(x_max, y_max, z_max),
            self.vector(x_max, y_max, z_min),
            self.vector(x_max, y_min, z_max),
            self.vector(x_max, y_min, z_min),
            self.vector(x_min, y_max, z_max),
            self.vector(x_min, y_max, z_min),
            self.vector(x_min, y_min, z_max),
            self.vector(x_min, y_min, z_min)
        ];
        perlin_interp(&vecs, u, v, w)
    }

    pub fn turbulence(&self, point : &Point3, depth : usize) -> f64 {
        let mut curr_point = *point;
        let mut weight = 0.5;
        let mut val = 0.0;
        for _ in 0..depth {
            val += weight * self.noise(&curr_point);
            weight *= 0.5;
            curr_point *= 2.0;
        }
        val.abs()
    }

    fn vector(&self, i : usize, j : usize, k : usize) -> Vec3 {
        let index = self.x_permute[i] ^ self.y_permute[j] ^ self.z_permute[k];
        self.vectors[index as usize]
    }
}