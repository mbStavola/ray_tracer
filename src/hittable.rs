use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Hit {
    t: f64,
    p: Vec3,
    normal: Vec3
}

impl Hit {
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> &Vec3 {
        &self.p
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}


impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut min_distance = t_max;
        let mut nearest_hit = None;

        for hittable in self {
            if let Some(hit) = hittable.hit(ray, t_min, min_distance) {
                min_distance = hit.t();
                nearest_hit = Some(hit);
            }
        }

        return nearest_hit
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin() - &self.center;

        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant = discriminant.sqrt();

        let f = (-b - discriminant) / a;
        let g = (-b + discriminant) / a;

        let t = if f < t_max && f > t_min {
            f
        } else if g < t_max && g > t_min {
            g
        } else {
            return None;
        };

        let p = ray.point_at(t);
        let normal = (&p - &self.center) / self.radius;

        let hit = Hit {
            t,
            p,
            normal
        };

        return Some(hit)
    }
}