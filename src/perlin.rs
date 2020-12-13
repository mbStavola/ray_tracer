use crate::{util::RandomDouble, vec3::Vec3};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

type TrilinearSample = [[[f64; 2]; 2]; 2];

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug)]
pub struct Perlin {
    random_floats: [f64; POINT_COUNT],

    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new<T: Rng>(rng: &mut T) -> Self {
        let mut random_floats = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            random_floats[i] = rng.random_double();
        }

        Self {
            random_floats,
            perm_x: generate_perm(rng),
            perm_y: generate_perm(rng),
            perm_z: generate_perm(rng),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut sample: TrilinearSample = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = (i + di as i32) & 255;
                    let j = (j + dj as i32) & 255;
                    let k = (k + dk as i32) & 255;

                    let x = self.perm_x[i as usize];
                    let y = self.perm_y[j as usize];
                    let z = self.perm_z[k as usize];

                    let index = (x ^ y ^ z) as usize;

                    sample[di][dj][dk] = self.random_floats[index];
                }
            }
        }

        trilinear_interp(sample, u, v, w)
    }
}

fn generate_perm<T: Rng>(rng: &mut T) -> [i32; POINT_COUNT] {
    let mut vec = (0..POINT_COUNT as i32).rev().collect_vec();
    &vec.shuffle(rng);

    let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];
    for (index, value) in vec.into_iter().enumerate() {
        p[index] = value;
    }

    p
}

fn trilinear_interp(sample: TrilinearSample, u: f64, v: f64, w: f64) -> f64 {
    let mut accumulator = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let color = sample[i][j][k];

                let i = i as f64 * u + (1.0 - i as f64) * (1.0 - u);
                let j = j as f64 * v + (1.0 - j as f64) * (1.0 - v);
                let k = k as f64 * w + (1.0 - k as f64) * (1.0 - w);

                accumulator += color * i * j * k;
            }
        }
    }

    accumulator
}
