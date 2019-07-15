use crate::{
    ray::Ray,
    vec3::Vec3
};

use std::f64::consts::PI;
use rand::Rng;
use crate::util::DRand48;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, vertical_fov: f64, aspect: f64) -> Camera {
        let theta = vertical_fov * (PI / 180.0);

        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (&look_from - &look_at).unit();
        let u = v_up.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let lower_left_corner = &origin - half_width * &u - half_height * &v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let direction = &self.lower_left_corner + u * &self.horizontal + v * &self.vertical - &self.origin;
        Ray::new(self.origin.clone(), direction)
    }
}

fn random_in_unit_disk<T: Rng>(rng: &mut T) -> Vec3 {
    let mut gen_p =
        || 2.0 * Vec3::new(rng.gen48(), rng.gen48(), 0.0) - Vec3::new(1.0, 1.0, 0.0);

    let mut p = gen_p();
    while p.dot(&p) >= 1.0 {
        p = gen_p();
    }
    p
}