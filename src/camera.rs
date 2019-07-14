use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::default();

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let direction = &self.lower_left_corner + u * &self.horizontal + v * &self.vertical;
        Ray::new(self.origin.clone(), direction)
    }
}
