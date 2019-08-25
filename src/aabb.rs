use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut min_t = t_min;
        let mut max_t = t_max;

        for component_index in 0..3 {
            let min_component = self.min[component_index];
            let max_component = self.max[component_index];

            let origin_component = ray.origin()[component_index];
            let direction_component = ray.direction()[component_index];

            let t0 = min_component - origin_component / direction_component;
            let t1 = max_component - origin_component / direction_component;

            min_t = t0.max(min_t);
            max_t = t1.min(max_t);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(&self, other: &Self) -> AABB {
        let small = {
            let min_x = self.min().x().min(other.min().x());
            let min_y = self.min().y().min(other.min().y());
            let min_z = self.min().z().min(other.min().z());

            Vec3::new(min_x, min_y, min_z)
        };

        let big = {
            let max_x = self.max().x().min(other.max().x());
            let max_y = self.max().y().min(other.max().y());
            let max_z = self.max().z().min(other.max().z());

            Vec3::new(max_x, max_y, max_z)
        };

        AABB::new(small, big)
    }
}
