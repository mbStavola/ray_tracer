#![deny(rust_2018_idioms)]

use std::{env, f64, time::Instant};

use rand::{rngs::SmallRng, SeedableRng};

use crate::{
    camera::Camera, config::read_tracer_config, renderer::render, vec3::Vec3, world::gen_world,
};

mod camera;
mod config;
mod hittable;
mod material;
mod ppm;
mod ray;
mod renderer;
mod util;
mod vec3;
mod world;

fn main() {
    let tracer_config = {
        let config_path = env::var("TRACER_CONFIG_PATH")
            .unwrap_or_else(|_| "./resources/tracer.toml".to_string());
        read_tracer_config(config_path)
    };

    let screen_width = tracer_config.screen_width().unwrap_or_else(|| 200);
    let screen_height = tracer_config.screen_height().unwrap_or_else(|| 100);

    let camera = {
        let look_from = Vec3::new(20.0, 3.0, -6.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let v_up = Vec3::new(0.0, 1.0, 0.0);

        let focus_distance = (&look_from - &look_at).length();
        let aperture = 0.2;

        Camera::new(
            look_from,
            look_at,
            v_up,
            15.0,
            screen_width as f64 / screen_height as f64,
            aperture,
            focus_distance,
        )
    };

    let world = {
        let mut rng = SmallRng::from_entropy();

        let is_dynamic = tracer_config.dynamic_world();
        gen_world(&mut rng, is_dynamic)
    };

    let tracing_start = Instant::now();
    println!("Start tracing");

    let antialias_iterations = tracer_config.antialias_iterations().unwrap_or_else(|| 100);
    let buffer: Vec<u8> = render(
        &world,
        &camera,
        screen_width,
        screen_height,
        antialias_iterations,
    );

    println!(
        "End tracing-- took {} ms",
        tracing_start.elapsed().as_millis()
    );

    let ppm_start = Instant::now();
    println!("Start ppm creation");

    let output_path = tracer_config
        .output_path()
        .cloned()
        .unwrap_or_else(|| "./resources/output.ppm".to_string());
    ppm::create(output_path, screen_width, screen_height, &buffer);

    println!(
        "End ppm creation-- took {} ms",
        ppm_start.elapsed().as_millis()
    );
}
