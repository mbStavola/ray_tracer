use std::{fs::File, io::Read, path::Path};

use serde_derive::Deserialize;
use toml;

#[derive(Default, Deserialize)]
pub struct TracerConfig {
    output_path: Option<String>,
    screen_width: Option<usize>,
    screen_height: Option<usize>,
    antialias_iterations: Option<usize>,
    dynamic_world: bool,
}

impl TracerConfig {
    pub fn output_path(&self) -> Option<&String> {
        self.output_path.as_ref()
    }

    pub fn screen_width(&self) -> Option<usize> {
        self.screen_width
    }

    pub fn screen_height(&self) -> Option<usize> {
        self.screen_height
    }

    pub fn antialias_iterations(&self) -> Option<usize> {
        self.antialias_iterations
    }

    pub fn dynamic_world(&self) -> bool {
        self.dynamic_world
    }
}

pub fn read_tracer_config<P: AsRef<Path>>(input_path: P) -> TracerConfig {
    let mut config_file = File::open(input_path).unwrap();

    let mut buffer = String::new();
    config_file.read_to_string(&mut buffer);

    toml::from_str(&buffer).unwrap_or_default()
}
