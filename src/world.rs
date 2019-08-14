use crate::{hittable::Shape, material::Material, util::DRand48};
use rand::Rng;

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

    vec![sphere_a, sphere_b, sphere_c, sphere_d, sphere_e]
}

fn random_world<T: Rng>(rng: &mut T, object_count: usize) -> Vec<Shape> {
    let mut world = Vec::with_capacity(object_count + 4);

    {
        let ground_sphere = Shape::sphere(
            0.0,
            -1000.0,
            0.0,
            1000.0,
            Material::lambertian(0.5, 0.5, 0.5),
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

    for a in -11..11 {
        for b in -11..11 {
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

            let sphere = Shape::sphere(
                f64::from(a) + 0.9 * rng.gen48(),
                0.2,
                f64::from(b) + 0.9 * rng.gen48(),
                0.2,
                material,
            );
            world.push(sphere);
        }
    }

    world
}

pub fn gen_world<T: Rng>(rng: &mut T, is_dynamic: bool) -> Vec<Shape> {
    if is_dynamic {
        random_world(rng, 1)
    } else {
        static_world()
    }
}
