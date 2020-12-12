use crate::{util::DRand48, vec3::Vec3};
use itertools::Itertools;
use rand::{prelude::SmallRng, seq::SliceRandom, Rng};

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug)]
pub struct Perlin {
    random_floats: [f32; POINT_COUNT],

    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256],
}

impl Perlin {
    pub fn new<T: Rng>(rng: &mut T) -> Self {
        let mut random_floats = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            random_floats[i] = rng.gen48();
        }

        Self {
            random_floats,
            perm_x: generate_perm(rng),
            perm_y: generate_perm(rng),
            perm_z: generate_perm(rng),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let i: usize = (4.0 * p.x()) as usize & 255;
        let j: usize = (4.0 * p.y()) as usize & 255;
        let k: usize = (4.0 * p.z()) as usize & 255;

        let x = self.perm_x[i];
        let y = self.perm_y[j];
        let z = self.perm_z[k];

        let index = (x ^ y ^ z) as usize;

        self.random_floats[index]
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
