use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::bvh::BoundingVolumeHierarchy;
use crate::texture::Texture;
use crate::{hittable::Shape, material::Material, util::DRand48};

fn static_world() -> Vec<Shape> {
    let sphere_a = Shape::sphere(0.0, 0.0, -1.0, 0.5, Material::lambertian(0.8, 0.3, 0.3));
    let sphere_b = Shape::sphere(
        0.0,
        -100.5,
        -1.0,
        100.0,
        Material::lambertian(0.8, 0.8, 0.0),
    );
    let sphere_c = Shape::sphere(1.0, 0.0, -1.0, 0.5, Material::metal(0.8, 0.6, 0.2, 0.0));
    let sphere_d = Shape::sphere(-1.0, 0.0, -1.0, 0.5, Material::dielectric(1.5));
    let sphere_e = Shape::sphere(-1.0, 0.0, -1.0, -0.45, Material::dielectric(1.5));

    let mut spheres = vec![sphere_a, sphere_b, sphere_c, sphere_d, sphere_e];

    for i in 0..3 {
        let i = i as f32;
        let x = 1.5 + (i * 0.2);
        let z = -1.5 + (i * 0.2);

        let material = Material::lambertian(i * 0.5, i * 0.5, i * 0.5);
        let sphere = Shape::sphere(x, 0.0, z, 0.2, material);
        spheres.push(sphere);
    }

    for i in 0..2 {
        let i = i as f32;
        let x = 1.1 + (i * 0.4) + i;
        let z = -1.3 - (i * 0.3) + i;

        let material = Material::lambertian(i * 0.5, i * 0.5, i * 0.5);
        let sphere = Shape::moving_sphere(x, 0.0, z, x, 0.7, z, 0.2, material, 0.0, 1.0);
        spheres.push(sphere);
    }

    for i in 0..3 {
        let i = i as f32;
        let x = 1.2 - (i * 0.8) + i;
        let z = -1.6 + (i * 0.1) + i;

        let material = Material::metal(0.8, 0.6, 0.2, 0.0);
        let sphere = Shape::sphere(x, 0.0, z, 0.2, material);
        spheres.push(sphere);
    }

    for i in 0..2 {
        let i = i as f32;
        let x = 1.4 + (i * 0.2) + i;
        let z = -1.2 + (i * 0.6) + i;

        let material = Material::dielectric(1.5);
        let sphere = Shape::sphere(x, 0.0, z, 0.2, material);
        spheres.push(sphere);
    }

    spheres
}

fn random_world<T: Rng>(rng: &mut T, max_objects: usize) -> Vec<Shape> {
    let max_objects = max_objects + 4;
    let mut world = Vec::with_capacity(max_objects);

    {
        let checker_pattern = Texture::checker_color(0.2, 0.3, 0.1, 0.9, 0.9, 0.9);
        let ground_sphere = Shape::sphere(
            0.0,
            -1000.0,
            0.0,
            1000.0,
            Material::textured(checker_pattern),
        );

        let glass_sphere = Shape::sphere(0.0, 1.0, 0.0, 1.0, Material::dielectric(1.5));
        let lambertian_sphere =
            Shape::sphere(-4.0, 1.0, 0.0, 1.0, Material::lambertian(0.4, 0.2, 0.1));
        let metal_sphere = Shape::sphere(4.0, 1.0, 0.0, 1.0, Material::metal(0.7, 0.6, 0.5, 0.0));

        world.push(ground_sphere);
        world.push(glass_sphere);
        world.push(lambertian_sphere);
        world.push(metal_sphere);
    }

    let mut outer_range = (-11..11).collect_vec();
    outer_range.shuffle(rng);

    for a in outer_range.into_iter() {
        let mut inner_range = (-11..11).collect_vec();
        inner_range.shuffle(rng);

        for b in inner_range.into_iter() {
            if world.len() == max_objects {
                break;
            }

            let material_choice = rng.gen48();
            let material = if material_choice < 0.8 {
                Material::lambertian(
                    rng.gen48() * rng.gen48(),
                    rng.gen48() * rng.gen48(),
                    rng.gen48() * rng.gen48(),
                )
            } else if material_choice < 0.95 {
                Material::metal(
                    0.5 * (1.0 + rng.gen48()),
                    0.5 * (1.0 + rng.gen48()),
                    0.5 * (1.0 + rng.gen48()),
                    0.5 * rng.gen48(),
                )
            } else {
                Material::dielectric(1.5)
            };

            let is_moving = rng.gen48() < 0.40;
            let sphere = if material_choice < 0.8 && is_moving {
                let x = (a as f32) + 0.9 * rng.gen48();
                let z = (b as f32) + 0.9 * rng.gen48();

                Shape::moving_sphere(
                    x,
                    0.2,
                    z,
                    x,
                    0.2 + (0.5 * rng.gen48()),
                    z,
                    0.2,
                    material,
                    0.0,
                    1.0,
                )
            } else {
                Shape::sphere(
                    (a as f32) + 0.9 * rng.gen48(),
                    0.2,
                    (b as f32) + 0.9 * rng.gen48(),
                    0.2,
                    material,
                )
            };

            world.push(sphere);
        }
    }

    world
}

pub fn gen_world<T: Rng>(
    rng: &mut T,
    is_dynamic: bool,
    max_objects: usize,
    time_initial: f32,
    time_final: f32,
) -> BoundingVolumeHierarchy {
    let world = if is_dynamic {
        random_world(rng, max_objects)
    } else {
        static_world()
    };

    BoundingVolumeHierarchy::new(rng, world, time_initial, time_final)
}
