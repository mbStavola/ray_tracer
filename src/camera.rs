use crate::{
    ray::Ray,
    vec3::Vec3
};

use std::f64::consts::PI;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(vertical_fov: f64, aspect: f64) -> Camera {
        let theta = vertical_fov * (PI / 180.0);

        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        let horizontal = Vec3::new(2.0 * half_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0 * half_height, 0.0);
        let origin = Vec3::default();

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
