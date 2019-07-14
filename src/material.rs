use crate::hittable::Hit;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;
use std::cell::{RefCell, RefMut};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit<'_>) -> Option<ScatterResponse>;
}

pub struct ScatterResponse {
    scattered: Ray,
    attenuation: Vec3,
}

impl ScatterResponse {
    pub fn new(scattered: Ray, attenuation: Vec3) -> ScatterResponse {
        ScatterResponse {
            scattered,
            attenuation,
        }
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit<'_>) -> Option<ScatterResponse> {
        let target = hit.p() + hit.normal() + random_in_unit_sphere();
        let response = ScatterResponse {
            scattered: Ray::new(hit.p().clone(), target - hit.p().clone()),
            attenuation: self.albedo.clone(),
        };
        Some(response)
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit<'_>) -> Option<ScatterResponse> {
        let reflected = reflect(&ray.direction().unit(), hit.normal());
        let scattered = Ray::new(hit.p().clone(), reflected);

        if scattered.direction().dot(hit.normal()) > 0.0 {
            let response = ScatterResponse {
                scattered,
                attenuation: self.albedo.clone(),
            };
            Some(response)
        } else {
            None
        }
    }
}

fn reflect(a: &Vec3, b: &Vec3) -> Vec3 {
    a - &(2.0 * a.dot(b) * b)
}
