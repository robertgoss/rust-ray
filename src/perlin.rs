use rand::Rng;
use rand::seq::SliceRandom;
use crate::vec3::Point3;

const POINT_COUNT : usize = 256;
pub struct Perlin {
    floats : [f64; POINT_COUNT],
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
    let int = (fint as i64);
    ((int as usize) & (POINT_COUNT - 1), ((int+1) as usize) & (POINT_COUNT - 1), rem)
}

fn trilinear_interp(verts : &[f64; 8], x : f64, y : f64, z : f64) -> f64 {
    x * y * z * verts[0] +
        x * y * (1.0-z) * verts[1] +
        x * (1.0-y) * z * verts[2] +
        x * (1.0-y) * (1.0-z) * verts[3] +
        (1.0-x) * y * z * verts[4] +
        (1.0-x) * y * (1.0-z) * verts[5] +
        (1.0-x) * (1.0-y) * z * verts[6] +
        (1.0-x) * (1.0-y) * (1.0-z) * verts[7]
}


impl Perlin {
    pub fn new<R>(rng : &mut R) -> Perlin
    where R : Rng
    {
        let mut floats : [f64; POINT_COUNT] = [0.0; POINT_COUNT];
        floats.iter_mut().for_each(
            |val| *val = rng.gen::<f64>()
        );
        let x_permute = make_permute(rng);
        let y_permute = make_permute(rng);
        let z_permute = make_permute(rng);

        Perlin {
            floats,
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
        let vals = [
            self.coord(x_max, y_max, z_max),
            self.coord(x_max, y_max, z_min),
            self.coord(x_max, y_min, z_max),
            self.coord(x_max, y_min, z_min),
            self.coord(x_min, y_max, z_max),
            self.coord(x_min, y_max, z_min),
            self.coord(x_min, y_min, z_max),
            self.coord(x_min, y_min, z_min)
        ];
        trilinear_interp(&vals, u, v, w)
    }

    fn coord(&self, i : usize, j : usize, k : usize) -> f64 {
        let index = self.x_permute[i] ^ self.y_permute[j] ^ self.z_permute[k];
        self.floats[index as usize]
    }
}