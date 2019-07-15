use rand::Rng;

use crate::{hittable::Hit, ray::Ray, util::DRand48, vec3::Vec3};

pub trait Scatterable<T: Rng> {
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

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl<T: Rng> Scatterable<T> for Lambertian {
    fn scatter(&self, rng: &mut T, _ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse<'_>> {
        let target = hit.p() + hit.normal() + random_in_unit_sphere(rng);
        let scattered = Ray::new(hit.p().clone(), target - hit.p());
        let response = ScatterResponse::new(scattered, &self.albedo);
        Some(response)
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl<T: Rng> Scatterable<T> for Metal {
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

pub struct Dielectric {
    ref_idx: f64,
    attenuation: Vec3, // TODO(Matt): Figure out a way to include this in scatter instead
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric {
            ref_idx,
            attenuation: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl<T: Rng> Scatterable<T> for Dielectric {
    fn scatter(&self, rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse<'_>> {
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(hit.normal()) > 0.0 {
            let cosine =
                self.ref_idx * ray.direction().dot(hit.normal()) / ray.direction().length();
            (-hit.normal().clone(), self.ref_idx, cosine)
        } else {
            let cosine = -ray.direction().dot(hit.normal()) / ray.direction().length();
            (hit.normal().clone(), 1.0 / self.ref_idx, cosine)
        };

        let reflected = reflect(ray.direction(), hit.normal());
        let (reflect_prob, r) =
            if let Some(ref refraction) = refract(ray.direction(), &outward_normal, ni_over_nt) {
                let reflect_prob = schlick(cosine, self.ref_idx);
                (reflect_prob, refraction.clone())
            } else {
                (1.0, reflected.clone())
            };

        let refracted = if rng.gen48() < reflect_prob {
            Ray::new(hit.p().clone(), reflected)
        } else {
            Ray::new(hit.p().clone(), r.clone())
        };

        let scatter = ScatterResponse::new(refracted, &self.attenuation);

        Some(scatter)
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    Metal(Metal),
}

impl Material {
    pub fn lambertian(e0: f64, e1: f64, e2: f64) -> Material {
        let albedo = Vec3::new(e0, e1, e2);
        let material = Lambertian::new(albedo);
        Material::Lambertian(material)
    }

    pub fn metal(e0: f64, e1: f64, e2: f64) -> Material {
        let albedo = Vec3::new(e0, e1, e2);
        let material = Metal::new(albedo);
        Material::Metal(material)
    }

    pub fn dielectric(ref_idx: f64) -> Material {
        let material = Dielectric::new(ref_idx);
        Material::Dielectric(material)
    }
}

impl<'a, T: Rng> Scatterable<T> for Material {
    fn scatter(&self, rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse<'_>> {
        match self {
            Material::Lambertian(material) => material.scatter(rng, ray, hit),
            Material::Dielectric(material) => material.scatter(rng, ray, hit),
            Material::Metal(material) => material.scatter(rng, ray, hit),
        }
    }
}

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    let mut gen_p =
        || 2.0 * Vec3::new(rng.gen48(), rng.gen48(), rng.gen48()) - Vec3::new(1.0, 1.0, 1.0);

    let mut p = gen_p();
    while p.square_length() >= 1.0 {
        p = gen_p();
    }
    p
}

fn reflect(a: &Vec3, b: &Vec3) -> Vec3 {
    a - &(2.0 * a.dot(b) * b)
}

fn refract(a: &Vec3, b: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = a.unit();
    let dt = uv.dot(b);

    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - b * dt) - b * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
