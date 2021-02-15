use rand::Rng;

use crate::{
    hittable::Hit,
    ray::Ray,
    texture::{Texturable, Texture},
    util::RandomDouble,
    vec3::Vec3,
};

pub trait Scatterable<T: Rng> {
    fn scatter(&self, rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse>;
    fn emit(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
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

    pub fn scattered(&self) -> &Ray {
        &self.scattered
    }

    pub fn attenuation(&self) -> &Vec3 {
        &self.attenuation
    }
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Texture,
}

impl<'a> Lambertian {
    pub fn new(albedo: Texture) -> Lambertian {
        Lambertian { albedo }
    }
}

impl<'a, T: Rng> Scatterable<T> for Lambertian {
    fn scatter(&self, rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse> {
        let target = hit.p() + hit.normal() + random_in_unit_sphere(rng);
        let scattered = Ray::new(hit.p().clone(), target - hit.p(), ray.time());
        let attenuation = self.albedo.value(hit.u(), hit.v(), hit.p());
        let response = ScatterResponse::new(scattered, attenuation);
        Some(response)
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Metal {
        let fuzziness = fuzziness.min(1.0);
        Metal { albedo, fuzziness }
    }
}

impl<T: Rng> Scatterable<T> for Metal {
    fn scatter(&self, rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse> {
        let reflected = reflect(&ray.direction().unit(), hit.normal());
        let scattered = Ray::new(
            hit.p().clone(),
            reflected + self.fuzziness * random_in_unit_sphere(rng),
            ray.time(),
        );

        if scattered.direction().dot(hit.normal()) > 0.0 {
            let attenuation = self.albedo.clone();
            let response = ScatterResponse::new(scattered, attenuation);
            Some(response)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl<T: Rng> Scatterable<T> for Dielectric {
    fn scatter(&self, rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse> {
        let refraction_ratio = if hit.is_front_facing() {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = (-unit_direction.clone()).dot(hit.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || schlick(cos_theta, refraction_ratio) > rng.random_double() {
                reflect(&unit_direction, hit.normal())
            } else {
                refract(&unit_direction, hit.normal(), refraction_ratio)
            };

        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refracted = Ray::new(hit.p().clone(), direction, ray.time());
        let scatter = ScatterResponse::new(refracted, attenuation);

        Some(scatter)
    }
}

#[derive(Clone, Debug)]
pub struct DiffuseLight {
    texture: Texture,
}

impl DiffuseLight {
    fn new(texture: Texture) -> Self {
        Self { texture }
    }
}

impl<T: Rng> Scatterable<T> for DiffuseLight {
    fn scatter(&self, _rng: &mut T, _ray: &Ray, _hit: &Hit<'_, T>) -> Option<ScatterResponse> {
        None
    }

    fn emit(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.texture.value(u, v, p)
    }
}

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    Metal(Metal),
    DiffuseLight(DiffuseLight),
}

impl Material {
    pub fn lambertian(r: f64, g: f64, b: f64) -> Self {
        let albedo = Texture::constant(r, g, b);
        let material = Lambertian::new(albedo);
        Self::Lambertian(material)
    }

    pub fn textured(texture: Texture) -> Self {
        let material = Lambertian::new(texture);
        Self::Lambertian(material)
    }

    pub fn metal(e0: f64, e1: f64, e2: f64, fuzziness: f64) -> Self {
        let albedo = Vec3::new(e0, e1, e2);
        let material = Metal::new(albedo, fuzziness);
        Self::Metal(material)
    }

    pub fn dielectric(ref_idx: f64) -> Self {
        let material = Dielectric::new(ref_idx);
        Self::Dielectric(material)
    }

    pub fn diffuse_light(texture: Texture) -> Self {
        let material = DiffuseLight::new(texture);
        Self::DiffuseLight(material)
    }
}

impl<'a, T: Rng> Scatterable<T> for Material {
    fn scatter(&self, rng: &mut T, ray: &Ray, hit: &Hit<'_, T>) -> Option<ScatterResponse> {
        match self {
            Material::Lambertian(material) => material.scatter(rng, ray, hit),
            Material::Dielectric(material) => material.scatter(rng, ray, hit),
            Material::Metal(material) => material.scatter(rng, ray, hit),
            Material::DiffuseLight(material) => material.scatter(rng, ray, hit),
        }
    }

    fn emit(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match self {
            Material::Lambertian(material) => <dyn Scatterable<T>>::emit(material, u, v, p),
            Material::Dielectric(material) => <dyn Scatterable<T>>::emit(material, u, v, p),
            Material::Metal(material) => <dyn Scatterable<T>>::emit(material, u, v, p),
            Material::DiffuseLight(material) => <dyn Scatterable<T>>::emit(material, u, v, p),
        }
    }
}

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    let mut gen_p = || {
        2.0 * Vec3::new(
            rng.random_double(),
            rng.random_double(),
            rng.random_double(),
        ) - Vec3::new(1.0, 1.0, 1.0)
    };

    let mut p = gen_p();
    while p.square_length() >= 1.0 {
        p = gen_p();
    }
    p
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * v.dot(n) * n)
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv.clone()).dot(n).min(1.0);
    let perpendicular = etai_over_etat * (uv + cos_theta * n);
    let parallel = -(1.0 - perpendicular.square_length()).abs().sqrt() * n;

    perpendicular + parallel
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
