use crate::{hittable::Hit, random_in_unit_sphere, ray::Ray, vec3::Vec3};
use rand::Rng;

pub trait Material<T: Rng> {
    fn scatter(&self, rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse<'_>>;
}

pub struct ScatterResponse<'a> {
    scattered: Ray,
    attenuation: &'a Vec3,
}

impl<'a> ScatterResponse<'a> {
    pub fn new(scattered: Ray, attenuation: &'a Vec3) -> ScatterResponse<'a> {
        ScatterResponse {
            scattered,
            attenuation,
        }
    }

    pub fn scattered(&self) -> &Ray {
        &self.scattered
    }

    pub fn attenuation(&self) -> &Vec3 {
        self.attenuation
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

impl<T: Rng> Material<T> for Lambertian {
    fn scatter(&self, rng: &mut T, _ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse<'_>> {
        let target = hit.p() + hit.normal() + random_in_unit_sphere(rng);
        let scattered = Ray::new(hit.p().clone(), target - hit.p());
        let response = ScatterResponse::new(scattered, &self.albedo);
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

impl<T: Rng> Material<T> for Metal {
    fn scatter(&self, _rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse<'_>> {
        let reflected = reflect(&ray.direction().unit(), hit.normal());
        let scattered = Ray::new(hit.p().clone(), reflected);

        if scattered.direction().dot(hit.normal()) > 0.0 {
            let response = ScatterResponse::new(scattered, &self.albedo);
            Some(response)
        } else {
            None
        }
    }
}

fn reflect(a: &Vec3, b: &Vec3) -> Vec3 {
    a - &(2.0 * a.dot(b) * b)
}
