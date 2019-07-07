#![deny(rust_2018_idioms)]

use std::f64;

use crate::{
    hittable::{Hittable, Sphere},
    ray::Ray,
    vec3::Vec3,
};

const OUTPUT_PATH: &str = "/home/mbs/workspace/rust/ray_tracer/resources/foo.ppm";

const NX: usize = 1280;
const NY: usize = 720;

mod camera;
mod ppm;
mod vec3;
mod ray;
mod hittable;

fn color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0, f64::INFINITY) {
        0.5 * Vec3::new(hit.normal().x() + 1.0, hit.normal().y() + 1.0, hit.normal().z() + 1.0)
    } else {
        let unit = ray.direction().unit();
        let t = 0.5 * (unit.y() + 1.0);

        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::default();

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
    ];

    let mut buffer = vec![];
    for j in (0..NY).into_iter().rev() {
        for i in 0..NX {
            let u = (i as f64) / (NX as f64);
            let v = (j as f64) / (NY as f64);
            let direction = lower_left_corner.clone() + u * horizontal.clone() + v * vertical.clone();
            let ray = Ray::new(origin.clone(), direction);
            let pixel = 255.99 * color(&ray, &world);

            buffer.push(pixel.r() as u8);
            buffer.push(pixel.g() as u8);
            buffer.push(pixel.b() as u8);
        }
    }

    ppm::create(OUTPUT_PATH, NX, NY, &buffer);
}