use std::{fs::File, io::Read, path::Path};

use serde_derive::Deserialize;
use toml;

use crate::vec3::Vec3;

#[derive(Debug, Default, Deserialize)]
pub struct TracerConfig {
    renderer: RendererConfig,
    camera: CameraConfig,
    output: OutputConfig,
    world: WorldConfig,
}

impl TracerConfig {
    pub fn renderer_config(&self) -> &RendererConfig {
        &self.renderer
    }

    pub fn output_config(&self) -> &OutputConfig {
        &self.output
    }

    pub fn camera_config(&self) -> &CameraConfig {
        &self.camera
    }

    pub fn world_config(&self) -> &WorldConfig {
        &self.world
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct CameraConfig {
    #[serde(default = "CameraConfig::default_fov")]
    fov: f64,
    #[serde(default = "CameraConfig::default_aperture")]
    aperture: f64,
    #[serde(default = "CameraConfig::default_look_from")]
    look_from: Vec3,
    #[serde(default = "CameraConfig::default_look_at")]
    look_at: Vec3,
}

impl CameraConfig {
    fn default_fov() -> f64 {
        15.0
    }

    fn default_aperture() -> f64 {
        0.2
    }

    fn default_look_from() -> Vec3 {
        Vec3::default()
    }

    fn default_look_at() -> Vec3 {
        Vec3::default()
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }

    pub fn aperture(&self) -> f64 {
        self.aperture
    }

    pub fn look_from(&self) -> &Vec3 {
        &self.look_from
    }

    pub fn look_at(&self) -> &Vec3 {
        &self.look_at
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct RendererConfig {
    #[serde(default = "RendererConfig::default_screen_width")]
    screen_width: usize,
    #[serde(default = "RendererConfig::default_screen_height")]
    screen_height: usize,
    #[serde(default = "RendererConfig::default_antialias_iterations")]
    antialias_iterations: usize,
}

impl RendererConfig {
    fn default_screen_width() -> usize {
        200
    }

    fn default_screen_height() -> usize {
        100
    }

    fn default_antialias_iterations() -> usize {
        100
    }

    pub fn screen_width(&self) -> usize {
        self.screen_width
    }

    pub fn screen_height(&self) -> usize {
        self.screen_height
    }

    pub fn antialias_iterations(&self) -> usize {
        self.antialias_iterations
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "OutputConfig::default_output_path")]
    output_path: String,
}

impl OutputConfig {
    fn default_output_path() -> String {
        "./resources/output.ppm".to_string()
    }

    pub fn output_path(&self) -> &str {
        &self.output_path
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct WorldConfig {
    #[serde(default)]
    is_dynamic: bool,
}

impl WorldConfig {
    pub fn is_dyanmic(&self) -> bool {
        self.is_dynamic
    }
}

pub fn read_tracer_config<P: AsRef<Path>>(input_path: P) -> TracerConfig {
    let mut config_file = File::open(input_path).unwrap();

    let mut buffer = String::new();
    config_file.read_to_string(&mut buffer);

    toml::from_str(&buffer).unwrap()
}
