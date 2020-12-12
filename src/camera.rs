use std::f64::consts::PI;

use rand::Rng;

use crate::{ray::Ray, util::RandomDouble, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time_start: f64,
    time_end: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
        time_start: f64,
        time_end: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;

        let theta = vertical_fov * PI / 180.0;

        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (&look_from - &look_at).unit();
        let u = v_up.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let lower_left_corner = &origin
            - (half_width * focus_distance * &u)
            - (half_height * focus_distance * &v)
            - focus_distance * &w;
        let horizontal = 2.0 * half_width * focus_distance * &u;
        let vertical = 2.0 * half_height * focus_distance * &v;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time_start,
            time_end,
        }
    }

    pub fn ray<T: Rng>(&self, rng: &mut T, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = &self.u * rd.x() + &self.v * rd.y();
        let direction = &self.lower_left_corner + s * &self.horizontal + t * &self.vertical
            - &self.origin
            - &offset;

        let time =
            self.time_start + (rng.random_double() as f64) * (self.time_end - self.time_start);
        Ray::new(&self.origin + offset, direction, time)
    }
}

fn random_in_unit_disk<T: Rng>(rng: &mut T) -> Vec3 {
    let mut gen_p = || {
        2.0 * Vec3::new(rng.random_double(), rng.random_double(), 0.0) - Vec3::new(1.0, 1.0, 0.0)
    };

    let mut p = gen_p();
    while p.dot(&p) >= 1.0 {
        p = gen_p();
    }
    p
}
