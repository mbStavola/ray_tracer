#![deny(rust_2018_idioms)]

use std::f64;

use rand::{Rng, SeedableRng};

use crate::material::{Lambertian, Metal};
use crate::util::drand48;
use crate::{
    camera::Camera,
    hittable::{Hittable, Shape, Sphere},
    ray::Ray,
    util::DRand48,
    vec3::Vec3,
};
use std::cell::RefCell;

const OUTPUT_PATH: &str = "/home/mbs/workspace/rust/ray_tracer/resources/foo.ppm";

const NX: usize = 200;
const NY: usize = 100;
const NS: usize = 100;

mod camera;
mod hittable;
mod material;
mod ppm;
mod ray;
mod util;
mod vec3;

fn random_in_unit_sphere() -> Vec3 {
    let gen_p = || 2.0 * Vec3::new(drand48(), drand48(), drand48()) - Vec3::new(1.0, 1.0, 1.0);

    let mut p = gen_p();
    while p.square_length() >= 1.0 {
        p = gen_p();
    }
    p
}

fn color<'a>(ray: &Ray, world: &'a Vec<Shape<'a>>, depth: u8) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        let target = hit.p() + hit.normal() + random_in_unit_sphere();
        let bounce = Ray::new(hit.p().clone(), target - hit.p().clone());

        if depth > 50 {
            return Vec3::new(0.0, 0.0, 0.0)
        }

        if let Some(scatter) = hit.material().scatter(ray, &hit) {
            color(scatter.scattered(), world, depth + 1) * scatter.attenuation().clone()
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit = ray.direction().unit();
        let t = 0.5 * (unit.y() + 1.0);

        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let camera = Camera::new();

    let material_a = Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let sphere_a = Shape::sphere(0.0, 0.0, -1.0, 0.5, &material_a);

    let material_b = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let sphere_b = Shape::sphere(0.0, -100.5, -1.0, 100.0, &material_b);

    let material_c = Metal::new(Vec3::new(0.8, 0.6, 0.2));
    let sphere_c = Shape::sphere(1.0, 0.0, -1.0, 0.5, &material_c);

    let material_d = Metal::new(Vec3::new(0.8, 0.8, 0.8));
    let sphere_d = Shape::sphere(-1.0, 0.0, -1.0, 0.5, &material_d);

    let world: Vec<Shape<'_>> = vec![sphere_a, sphere_b, sphere_c, sphere_d];

    let mut buffer = vec![];
    for j in (0..NY).into_iter().rev() {
        for i in 0..NX {
            let mut pixel = Vec3::default();
            for _ in 0..NS {
                let u = (i as f64 + drand48()) / (NX as f64);
                let v = (j as f64 + drand48()) / (NY as f64);

                let ray = camera.ray(u, v);
                pixel += color(&ray, &world, 0);
            }
            pixel /= NS as f64;
            pixel = Vec3::new(pixel.r().sqrt(), pixel.g().sqrt(), pixel.b().sqrt());
            pixel *= 255.99;

            buffer.push(pixel.r() as u8);
            buffer.push(pixel.g() as u8);
            buffer.push(pixel.b() as u8);
        }
    }

    ppm::create(OUTPUT_PATH, NX, NY, &buffer);
}
