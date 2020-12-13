use crate::vec3::Vec3;
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};
use std::{mem, mem::MaybeUninit};

type TrilinearSample = [[[Vec3; 2]; 2]; 2];

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug)]
pub struct Perlin {
    random_vecs: [Vec3; POINT_COUNT],

    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new<T: Rng>(rng: &mut T) -> Self {
        // UNSAFE(Matt):
        // This is fine because we immediately initialize the memory right after
        let mut random_vecs: [MaybeUninit<Vec3>; POINT_COUNT] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for random_vec in random_vecs.iter_mut() {
            *random_vec = MaybeUninit::new(Vec3::random(rng, -1.0, 1.0).unit());
        }

        Self {
            random_vecs: unsafe { mem::transmute(random_vecs) },
            perm_x: generate_perm(rng),
            perm_y: generate_perm(rng),
            perm_z: generate_perm(rng),
        }
    }

    pub fn turbulence(&self, p: &Vec3, depth: usize) -> f64 {
        let mut current_p = p.clone();
        let mut weight: f64 = 1.0;

        let mut accumulator = 0.0;
        for _ in 0..depth {
            accumulator += weight * self.noise(&current_p);
            weight *= 0.5;
            current_p *= 2.0;
        }

        accumulator.abs()
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        // UNSAFE(Matt):
        // This is fine because we immediately initialize the memory right after
        let mut sample: [[[MaybeUninit<Vec3>; 2]; 2]; 2] =
            unsafe { MaybeUninit::uninit().assume_init() };
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

                    let sample = &mut sample[di][dj][dk];
                    *sample = MaybeUninit::new(self.random_vecs[index].clone());
                }
            }
        }

        trilinear_interp(unsafe { mem::transmute(sample) }, u, v, w)
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
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accumulator = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                let color = weight.dot(&sample[i][j][k]);

                let i = i as f64 * uu + (1.0 - i as f64) * (1.0 - uu);
                let j = j as f64 * vv + (1.0 - j as f64) * (1.0 - vv);
                let k = k as f64 * ww + (1.0 - k as f64) * (1.0 - ww);

                accumulator += color * i * j * k;
            }
        }
    }

    accumulator
}
