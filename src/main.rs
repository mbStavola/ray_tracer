#![deny(rust_2018_idioms)]

use std::{env, f64, time::Instant};

use rand::{rngs::SmallRng, SeedableRng};

use crate::{
    camera::Camera, config::read_tracer_config, renderer::render_world, vec3::Vec3,
    world::gen_world,
};

mod aabb;
mod bvh;
mod camera;
mod config;
mod hittable;
mod material;
mod perlin;
mod ppm;
mod ray;
mod renderer;
mod texture;
mod util;
mod vec3;
mod world;

fn main() {
    let tracer_config = {
        let config_path = env::var("TRACER_CONFIG_PATH")
            .unwrap_or_else(|_| "./resources/tracer.toml".to_string());
        read_tracer_config(config_path)
    };

    let screen_width = tracer_config.renderer_config().screen_width();
    let screen_height = tracer_config.renderer_config().screen_height();

    let camera = {
        let camera_config = tracer_config.camera_config();
        let look_from = camera_config.look_from().clone();
        let look_at = camera_config.look_at().clone();
        let time_start = camera_config.time_start();
        let time_end = camera_config.time_end();

        let v_up = Vec3::new(0.0, 1.0, 0.0);

        let focus_distance = 10.0; //(&look_from - &look_at).length();

        Camera::new(
            look_from,
            look_at,
            v_up,
            camera_config.fov(),
            screen_width as f64 / screen_height as f64,
            camera_config.aperture(),
            focus_distance,
            time_start,
            time_end,
        )
    };

    let world_gen_start = Instant::now();
    println!("Start world gen");
    let world = {
        let mut rng = SmallRng::from_entropy();

        let time_start = tracer_config.camera_config().time_start();
        let time_end = tracer_config.camera_config().time_end();

        gen_world(&mut rng, tracer_config.world_config(), time_start, time_end)
    };

    println!("End world gen-- took {:.2?}", world_gen_start.elapsed());

    let tracing_start = Instant::now();
    println!("Start tracing");

    let antialias_iterations = tracer_config.renderer_config().antialias_iterations();
    let render_parallel = tracer_config.renderer_config().render_parallel();
    let use_bounding_volume = tracer_config.renderer_config().use_bounding_volume();
    // let background = Vec3::new(0.70, 0.80, 1.00);
    let background = Vec3::new(0.0, 0.0, 0.0);

    let buffer: Vec<u8> = render_world(
        &background,
        &world,
        &camera,
        screen_width,
        screen_height,
        antialias_iterations,
        render_parallel,
        use_bounding_volume,
    );

    println!("End tracing-- took {:.2?}", tracing_start.elapsed());

    let ppm_start = Instant::now();
    println!("Start ppm creation");

    let output_path = tracer_config.output_config().output_path();
    ppm::create(output_path, screen_width, screen_height, &buffer);
    println!("End ppm creation-- took {:.2?}", ppm_start.elapsed());
}
