use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use crate::{
    bvh::BoundingVolumeHierarchy, config::WorldConfig, hittable::Shape, material::Material,
    texture::Texture, util::RandomDouble,
};

pub fn gen_world<T: Rng>(
    rng: &mut T,
    world_config: &WorldConfig,
    time_initial: f64,
    time_final: f64,
) -> BoundingVolumeHierarchy {
    let world = match world_config {
        WorldConfig::Basic => static_world(),
        WorldConfig::Dynamic { max_objects } => random_world(rng, *max_objects),
        WorldConfig::Checker => two_checker_spheres(),
        WorldConfig::Perlin => two_perlin_spheres(rng),
        WorldConfig::Earth => earth(),
        WorldConfig::SimpleLight => simple_light(rng),
        WorldConfig::CornellBox => cornell_box(),
    };

    BoundingVolumeHierarchy::new(rng, world, time_initial, time_final)
}

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
        let i = i as f64;
        let x = 1.5 + (i * 0.2);
        let z = -1.5 + (i * 0.2);

        let material = Material::lambertian(i * 0.5, i * 0.5, i * 0.5);
        let sphere = Shape::sphere(x, 0.0, z, 0.2, material);
        spheres.push(sphere);
    }

    for i in 0..2 {
        let i = i as f64;
        let x = 1.1 + (i * 0.4) + i;
        let z = -1.3 - (i * 0.3) + i;

        let material = Material::lambertian(i * 0.5, i * 0.5, i * 0.5);
        let sphere = Shape::moving_sphere(x, 0.0, z, x, 0.7, z, 0.2, material, 0.0, 1.0);
        spheres.push(sphere);
    }

    for i in 0..3 {
        let i = i as f64;
        let x = 1.2 - (i * 0.8) + i;
        let z = -1.6 + (i * 0.1) + i;

        let material = Material::metal(0.8, 0.6, 0.2, 0.0);
        let sphere = Shape::sphere(x, 0.0, z, 0.2, material);
        spheres.push(sphere);
    }

    for i in 0..2 {
        let i = i as f64;
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

            let material_choice = rng.random_double();
            let material = if material_choice < 0.8 {
                Material::lambertian(
                    rng.random_double() * rng.random_double(),
                    rng.random_double() * rng.random_double(),
                    rng.random_double() * rng.random_double(),
                )
            } else if material_choice < 0.95 {
                Material::metal(
                    0.5 * (1.0 + rng.random_double()),
                    0.5 * (1.0 + rng.random_double()),
                    0.5 * (1.0 + rng.random_double()),
                    0.5 * rng.random_double(),
                )
            } else {
                Material::dielectric(1.5)
            };

            let is_moving = rng.random_double() < 0.40;
            let sphere = if material_choice < 0.8 && is_moving {
                let x = (a as f64) + 0.9 * rng.random_double();
                let z = (b as f64) + 0.9 * rng.random_double();

                Shape::moving_sphere(
                    x,
                    0.2,
                    z,
                    x,
                    0.2 + (0.5 * rng.random_double()),
                    z,
                    0.2,
                    material,
                    0.0,
                    1.0,
                )
            } else {
                Shape::sphere(
                    (a as f64) + 0.9 * rng.random_double(),
                    0.2,
                    (b as f64) + 0.9 * rng.random_double(),
                    0.2,
                    material,
                )
            };

            world.push(sphere);
        }
    }

    world
}

fn two_checker_spheres() -> Vec<Shape> {
    let texture = Texture::checker_color(0.2, 0.3, 0.1, 0.9, 0.9, 0.9);

    let bottom = Shape::sphere(0.0, -10.0, 0.0, 10.0, Material::textured(texture.clone()));
    let top = Shape::sphere(0.0, 10.0, 0.0, 10.0, Material::textured(texture));

    vec![bottom, top]
}

fn two_perlin_spheres<T: Rng>(rng: &mut T) -> Vec<Shape> {
    let texture = Texture::scaled_noise(rng, 4.0);

    let ground_sphere = Shape::sphere(
        0.0,
        -1000.0,
        0.0,
        1000.0,
        Material::textured(texture.clone()),
    );

    let sphere = Shape::sphere(0.0, 2.0, 0.0, 2.0, Material::textured(texture));

    vec![ground_sphere, sphere]
}

fn earth() -> Vec<Shape> {
    let texture = Texture::image("resources/earthmap.jpg");
    let globe = Shape::sphere(0.0, 0.0, 0.0, 2.0, Material::textured(texture));
    vec![globe]
}

fn simple_light<T: Rng>(rng: &mut T) -> Vec<Shape> {
    let noise = Texture::noise(rng);
    let sphere_a = Shape::sphere(0.0, -1000.0, 0.0, 1000.0, Material::textured(noise.clone()));
    let sphere_b = Shape::sphere(0.0, 2.0, 0.0, 2.0, Material::textured(noise));

    let color = Texture::constant(4.0, 4.0, 4.0);
    let light_square = Shape::xy_rect(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Material::diffuse_light(color.clone()),
    );
    let light_sphere = Shape::sphere(0.0, 7.0, 0.0, 2.0, Material::diffuse_light(color));

    vec![sphere_a, sphere_b, light_square, light_sphere]
}

fn cornell_box() -> Vec<Shape> {
    let red = Material::lambertian(0.65, 0.05, 0.05);
    let green = Material::lambertian(0.12, 0.45, 0.15);
    let white = Material::lambertian(0.73, 0.73, 0.73);
    let light = Material::diffuse_light(Texture::constant(15.0, 15.0, 15.0));

    let left_wall = Shape::yz_rect(0.0, 555.0, 0.0, 555.0, 555.0, green);
    let right_wall = Shape::yz_rect(0.0, 555.0, 0.0, 555.0, 0.0, red);

    let light = Shape::xz_rect(213.0, 343.0, 227.0, 332.0, 554.0, light);

    let floor = Shape::xz_rect(0.0, 555.0, 0.0, 555.0, 0.0, white.clone());
    let ceiling = Shape::xz_rect(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());
    let back_wall = Shape::xy_rect(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());

    let box_a = Shape::cube(130.0, 0.0, 65.0, 295.0, 165.0, 230.0, white.clone());
    let box_b = Shape::cube(265.0, 0.0, 295.0, 430.0, 330.0, 460.0, white);

    vec![
        left_wall, right_wall, light, floor, ceiling, back_wall, box_a, box_b,
    ]
}
