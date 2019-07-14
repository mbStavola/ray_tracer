#![deny(rust_2018_idioms)]

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{
    camera::Camera,
    hittable::{Hittable, Shape},
    material::{Lambertian, Metal},
    ray::Ray,
    util::DRand48,
    vec3::Vec3,
};
use std::{f64, time::Instant};

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

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    let mut gen_p =
        || 2.0 * Vec3::new(rng.gen48(), rng.gen48(), rng.gen48()) - Vec3::new(1.0, 1.0, 1.0);

    let mut p = gen_p();
    while p.square_length() >= 1.0 {
        p = gen_p();
    }
    p
}

fn color<'a, T: Rng>(rng: &mut T, ray: &Ray, world: &'a Vec<Shape<'a, T>>, depth: u8) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if depth > 50 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(scatter) = hit.material().scatter(rng, ray, &hit) {
            color(rng, scatter.scattered(), world, depth + 1) * scatter.attenuation()
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
    let mut rng = SmallRng::from_entropy();
    let camera = Camera::new();

    let material_a = Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let sphere_a = Shape::sphere(0.0, 0.0, -1.0, 0.5, &material_a);

    let material_b = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let sphere_b = Shape::sphere(0.0, -100.5, -1.0, 100.0, &material_b);

    let material_c = Metal::new(Vec3::new(0.8, 0.6, 0.2));
    let sphere_c = Shape::sphere(1.0, 0.0, -1.0, 0.5, &material_c);

    let material_d = Metal::new(Vec3::new(0.8, 0.8, 0.8));
    let sphere_d = Shape::sphere(-1.0, 0.0, -1.0, 0.5, &material_d);

    let world = vec![sphere_a, sphere_b, sphere_c, sphere_d];

    let tracing_start = Instant::now();
    println!("Start tracing");
    let mut buffer = vec![];
    for j in (0..NY).into_iter().rev() {
        for i in 0..NX {
            let mut pixel = Vec3::default();
            for _ in 0..NS {
                let u = (i as f64 + rng.gen48()) / (NX as f64);
                let v = (j as f64 + rng.gen48()) / (NY as f64);

                let ray = camera.ray(u, v);
                pixel += color(&mut rng, &ray, &world, 0);
            }
            pixel /= NS as f64;
            pixel = Vec3::new(pixel.r().sqrt(), pixel.g().sqrt(), pixel.b().sqrt());
            pixel *= 255.99;

            buffer.push(pixel.r() as u8);
            buffer.push(pixel.g() as u8);
            buffer.push(pixel.b() as u8);
        }
    }
    println!(
        "End tracing-- took {} seconds",
        tracing_start.elapsed().as_secs()
    );

    let ppm_start = Instant::now();
    println!("Start ppm creation");
    ppm::create(OUTPUT_PATH, NX, NY, &buffer);
    println!(
        "End ppm creation-- took {} seconds",
        ppm_start.elapsed().as_secs()
    );
}
