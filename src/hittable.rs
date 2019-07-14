use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hittable<'a> {
    fn hit(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a>>;
}

pub struct Hit<'a> {
    t: f64,
    p: Vec3,
    normal: Vec3,
    material: &'a dyn Material,
}

impl Hit<'_> {
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> &Vec3 {
        &self.p
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn material(&self) -> &'_ dyn Material {
        self.material
    }
}

pub struct Sphere<'a> {
    center: Vec3,
    radius: f64,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f64, material: &'a dyn Material) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable<'a> for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a>> {
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

pub enum Shape<'a> {
    Sphere(Sphere<'a>),
}

impl<'a> Shape<'a> {
    pub fn sphere(x: f64, y: f64, z: f64, radius: f64, material: &'a dyn Material) -> Shape<'a> {
        let center = Vec3::new(x, y, z);
        let sphere: Sphere<'a> = Sphere::new(center, radius, material);
        Shape::Sphere(sphere)
    }
}

impl<'a> Hittable<'a> for Shape<'a> {
    fn hit(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a>> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }
}

impl<'a> Hittable<'a> for Vec<Shape<'a>> {
    fn hit(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a>> {
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
