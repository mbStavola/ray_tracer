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
    #[serde(default = "CameraConfig::default_time_start")]
    time_start: f64,
    #[serde(default = "CameraConfig::default_time_end")]
    time_end: f64,
}

impl CameraConfig {
    #[allow(dead_code)]
    fn default_fov() -> f64 {
        15.0
    }

    #[allow(dead_code)]
    fn default_aperture() -> f64 {
        0.2
    }

    #[allow(dead_code)]
    fn default_look_from() -> Vec3 {
        Vec3::default()
    }

    #[allow(dead_code)]
    fn default_look_at() -> Vec3 {
        Vec3::default()
    }

    #[allow(dead_code)]
    fn default_time_start() -> f64 {
        0.0
    }

    #[allow(dead_code)]
    fn default_time_end() -> f64 {
        1.0
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

    pub fn time_start(&self) -> f64 {
        self.time_start
    }

    pub fn time_end(&self) -> f64 {
        self.time_end
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
    #[serde(default)]
    render_parallel: bool,
    #[serde(default)]
    use_bounding_volume: bool,
}

impl RendererConfig {
    #[allow(dead_code)]
    fn default_screen_width() -> usize {
        200
    }

    #[allow(dead_code)]
    fn default_screen_height() -> usize {
        100
    }

    #[allow(dead_code)]
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

    pub fn render_parallel(&self) -> bool {
        self.render_parallel
    }

    pub fn use_bounding_volume(&self) -> bool {
        self.use_bounding_volume
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "OutputConfig::default_output_path")]
    output_path: String,

    #[serde(default)]
    file_name: String,

    #[serde(default)]
    file_type: String,

    #[serde(default)]
    render_window: bool,
}

impl OutputConfig {
    #[allow(dead_code)]
    fn default_output_path() -> String {
        "./resources/output.ppm".to_string()
    }

    pub fn output_path(&self) -> &str {
        &self.output_path
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "scene")]
pub enum WorldConfig {
    Basic,
    Dynamic {
        #[serde(default = "WorldConfig::default_max_objects")]
        max_objects: usize,
    },
    Checker,
    Perlin,
    Earth,
}

impl WorldConfig {
    #[allow(dead_code)]
    fn default_max_objects() -> usize {
        10
    }
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self::Basic
    }
}

pub fn read_tracer_config<P: AsRef<Path>>(input_path: P) -> TracerConfig {
    let mut config_file = File::open(input_path).unwrap();

    let mut buffer = String::new();
    config_file.read_to_string(&mut buffer);

    toml::from_str(&buffer).unwrap()
}
