use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    camera::Camera,
    hittable::{Hittable, Shape},
    ray::Ray,
    util::DRand48,
    vec3::Vec3,
};

fn color<'a, T: Rng>(rng: &mut T, ray: &Ray, world: &'a [Shape], depth: u8) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, std::f64::INFINITY) {
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

pub fn render(
    world: &[Shape],
    camera: &Camera,
    screen_width: usize,
    screen_height: usize,
    antialias_iterations: usize,
) -> Vec<u8> {
    (0..screen_width * screen_height)
        .into_par_iter()
        .map_init(SmallRng::from_entropy, |mut rng, idx| {
            let i = idx % screen_width;
            let j = screen_height - 1 - idx / screen_width;

            let mut pixel = Vec3::default();
            for _ in 0..antialias_iterations {
                let u = (i as f64 + rng.gen48()) / (screen_width as f64);
                let v = (j as f64 + rng.gen48()) / (screen_height as f64);

                let ray = camera.ray(&mut rng, u, v);
                pixel += color(&mut rng, &ray, &world, 0);
            }
            pixel /= antialias_iterations as f64;
            pixel = Vec3::new(pixel.r().sqrt(), pixel.g().sqrt(), pixel.b().sqrt());
            pixel *= 255.99;

            pixel
        })
        .flat_map(|pixel| vec![pixel.r() as u8, pixel.g() as u8, pixel.b() as u8])
        .collect()
}
