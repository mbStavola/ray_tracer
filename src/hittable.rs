use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

pub trait Hittable<'a, T: Rng> {
    fn hit(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a, T>>;
}

pub struct Hit<'a, T: Rng> {
    t: f64,
    p: Vec3,
    normal: Vec3,
    material: &'a dyn Material<T>,
}

impl<T: Rng> Hit<'_, T> {
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> &Vec3 {
        &self.p
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn material(&self) -> &'_ dyn Material<T> {
        self.material
    }
}

pub struct Sphere<'a, T: Rng> {
    center: Vec3,
    radius: f64,
    material: &'a dyn Material<T>,
}

impl<'a, T: Rng> Sphere<'a, T> {
    pub fn new(center: Vec3, radius: f64, material: &'a dyn Material<T>) -> Sphere<'a, T> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Sphere<'a, T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a, T>> {
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
        let material = self.material;

        let hit = Hit {
            t,
            p,
            normal,
            material,
        };

        return Some(hit);
    }
}

pub enum Shape<'a, T: Rng> {
    Sphere(Sphere<'a, T>),
}

impl<'a, T: Rng> Shape<'a, T> {
    pub fn sphere(
        x: f64,
        y: f64,
        z: f64,
        radius: f64,
        material: &'a dyn Material<T>,
    ) -> Shape<'a, T> {
        let center = Vec3::new(x, y, z);
        let sphere: Sphere<'a, T> = Sphere::new(center, radius, material);
        Shape::Sphere(sphere)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Shape<'a, T> {
    fn hit(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a, T>> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Vec<Shape<'a, T>> {
    fn hit(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a, T>> {
        let mut min_distance = t_max;
        let mut nearest_hit = None;

        for hittable in self {
            if let Some(hit) = hittable.hit(ray, t_min, min_distance) {
                min_distance = hit.t();
                nearest_hit = Some(hit);
            }
        }

        return nearest_hit;
    }
}
