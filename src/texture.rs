use crate::{perlin::Perlin, vec3::Vec3};
use rand::Rng;

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
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new<T: Rng>(rng: &mut T) -> Self {
        let noise = Perlin::new(rng);

        NoiseTexture { noise }
    }
}

impl Texturable for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}

#[derive(Clone, Debug)]
pub enum Texture {
    Constant(ConstantTexture),
    Checker(CheckerTexture),
    Noise(NoiseTexture),
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
}

impl Texturable for Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match self {
            Texture::Constant(texture) => texture.value(u, v, p),
            Texture::Checker(texture) => texture.value(u, v, p),
            Texture::Noise(texture) => texture.value(u, v, p),
        }
    }
}
