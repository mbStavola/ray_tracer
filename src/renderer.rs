use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    bvh::BoundingVolumeHierarchy, camera::Camera, hittable::Hittable, ray::Ray, util::RandomDouble,
    vec3::Vec3,
};

pub fn render_world<'a>(
    world: &'a BoundingVolumeHierarchy,
    camera: &Camera,
    screen_width: usize,
    screen_height: usize,
    antialias_iterations: usize,
    render_parallel: bool,
    use_bounding_volume: bool,
) -> Vec<u8> {
    let screen = 0..(screen_width * screen_height);

    let world: &'a dyn Hittable<'a, _> = if use_bounding_volume {
        world
    } else {
        world.shapes()
    };

    if render_parallel {
        screen
            .into_par_iter()
            .map_init(SmallRng::from_entropy, |rng, idx| {
                render(
                    rng,
                    world,
                    camera,
                    screen_width,
                    screen_height,
                    antialias_iterations,
                    idx,
                )
            })
            .flat_map(|pixel| vec![pixel.r() as u8, pixel.g() as u8, pixel.b() as u8])
            .collect()
    } else {
        screen
            .map(|idx| {
                let mut rng = SmallRng::from_entropy();
                render(
                    &mut rng,
                    world,
                    camera,
                    screen_width,
                    screen_height,
                    antialias_iterations,
                    idx,
                )
            })
            .flat_map(|pixel| vec![pixel.r() as u8, pixel.g() as u8, pixel.b() as u8])
            .collect()
    }
}

#[inline(always)]
fn render<'a, T: Rng>(
    rng: &mut T,
    world: &'a dyn Hittable<'a, T>,
    camera: &Camera,
    screen_width: usize,
    screen_height: usize,
    antialias_iterations: usize,
    idx: usize,
) -> Vec3 {
    let i = idx % screen_width;
    let j = screen_height - 1 - idx / screen_width;

    let mut pixel = Vec3::default();
    for _ in 0..antialias_iterations {
        let u = (i as f64 + rng.random_double()) / (screen_width as f64);
        let v = (j as f64 + rng.random_double()) / (screen_height as f64);

        let ray = camera.ray(rng, u, v);
        pixel += color(rng, &ray, world, 0);
    }
    pixel /= antialias_iterations as f64;
    pixel = Vec3::new(pixel.r().sqrt(), pixel.g().sqrt(), pixel.b().sqrt());
    pixel *= 255.99;

    pixel
}

fn color<'a, T: Rng>(rng: &mut T, ray: &Ray, world: &'a dyn Hittable<'a, T>, depth: u8) -> Vec3 {
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
