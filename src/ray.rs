use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        &self.origin + t * &self.direction
    }
}
