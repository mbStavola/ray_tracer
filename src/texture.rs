use crate::{perlin::Perlin, vec3::Vec3};
use image::{self, DynamicImage, GenericImageView};
use rand::Rng;
use std::{
    fmt::{Debug, Formatter},
    path::Path,
};

pub trait Texturable {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone, Debug)]
pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    fn new(color: Vec3) -> ConstantTexture {
        ConstantTexture { color }
    }
}

impl Texturable for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color.clone()
    }
}

#[derive(Clone, Debug)]
pub struct CheckerTexture {
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(even: Texture, odd: Texture) -> CheckerTexture {
        CheckerTexture {
            even: Box::new(even),
            odd: Box::new(odd),
        }
    }
}

impl Texturable for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoiseTexture {
    noise: Box<Perlin>,
    scale: f64,
}

impl NoiseTexture {
    pub fn new<T: Rng>(rng: &mut T) -> Self {
        let noise = Perlin::new(rng);

        Self {
            noise: Box::new(noise),
            scale: 1.0,
        }
    }

    pub fn with_scale<T: Rng>(rng: &mut T, scale: f64) -> Self {
        let noise = Perlin::new(rng);

        Self {
            noise: Box::new(noise),
            scale,
        }
    }
}

impl Texturable for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        let turbulence = self.scale * p.z() + 10.0 * self.noise.turbulence(&p, 7);
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + turbulence.sin())
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(image: DynamicImage) -> Self {
        Self { image }
    }
}

impl Debug for ImageTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ImageTexture")
    }
}

impl Texturable for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let u = fclamp(u, 0.0, 1.0);
        let v = 1.0 - fclamp(v, 0.0, 1.0); // Flip V to image coords

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        let i = clamp(i, 0, self.image.width() - 1);
        let j = clamp(j, 0, self.image.height() - 1);

        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);

        color_scale * Vec3::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    }
}

#[derive(Clone, Debug)]
pub enum Texture {
    Constant(ConstantTexture),
    Checker(CheckerTexture),
    Noise(NoiseTexture),
    Image(ImageTexture),
}

impl Texture {
    pub fn constant(r: f64, g: f64, b: f64) -> Self {
        let color = Vec3::new(r, g, b);
        let texture = ConstantTexture::new(color);
        Self::Constant(texture)
    }

    pub fn checker_color(
        r_even: f64,
        g_even: f64,
        b_even: f64,
        r_odd: f64,
        g_odd: f64,
        b_odd: f64,
    ) -> Self {
        let even = Self::constant(r_even, g_even, b_even);
        let odd = Self::constant(r_odd, g_odd, b_odd);

        let texture = CheckerTexture::new(even, odd);
        Self::Checker(texture)
    }

    pub fn checker(even: Self, odd: Self) -> Self {
        let texture = CheckerTexture::new(even, odd);
        Self::Checker(texture)
    }

    pub fn noise<T: Rng>(rng: &mut T) -> Self {
        let texture = NoiseTexture::new(rng);
        Self::Noise(texture)
    }

    pub fn scaled_noise<T: Rng>(rng: &mut T, scale: f64) -> Self {
        let texture = NoiseTexture::with_scale(rng, scale);
        Self::Noise(texture)
    }

    pub fn image<P: AsRef<Path>>(filepath: P) -> Self {
        let filepath = filepath.as_ref();
        let texture = match image::open(filepath) {
            Ok(texture) => texture,
            // The file was not found, log and return an error texture
            Err(e) => {
                eprintln!("Unable to load image {:?}: {}", filepath, e);
                return Texture::constant(0.0, 1.0, 1.0);
            }
        };
        let texture = ImageTexture::new(texture);
        Self::Image(texture)
    }
}

impl Texturable for Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match self {
            Texture::Constant(texture) => texture.value(u, v, p),
            Texture::Checker(texture) => texture.value(u, v, p),
            Texture::Noise(texture) => texture.value(u, v, p),
            Texture::Image(texture) => texture.value(u, v, p),
        }
    }
}

fn fclamp(mut x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        x = min;
    }

    if x > max {
        x = max;
    }

    x
}

fn clamp(mut x: u32, min: u32, max: u32) -> u32 {
    if x < min {
        x = min;
    }

    if x > max {
        x = max;
    }

    x
}
