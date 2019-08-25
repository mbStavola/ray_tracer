use crate::vec3::Vec3;

pub trait Texturable {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

#[derive(Debug)]
pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    fn new(color: Vec3) -> ConstantTexture {
        ConstantTexture { color }
    }
}

impl Texturable for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.color.clone()
    }
}

#[derive(Debug)]
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
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Debug)]
pub enum Texture {
    Constant(ConstantTexture),
    Checker(CheckerTexture),
}

impl Texture {
    pub fn constant(r: f32, g: f32, b: f32) -> Texture {
        let color = Vec3::new(r, g, b);
        let texture = ConstantTexture::new(color);
        Texture::Constant(texture)
    }

    pub fn checker_color(
        r_even: f32,
        g_even: f32,
        b_even: f32,
        r_odd: f32,
        g_odd: f32,
        b_odd: f32,
    ) -> Texture {
        let even = Texture::constant(r_even, g_even, b_even);
        let odd = Texture::constant(r_odd, g_odd, b_odd);

        let texture = CheckerTexture::new(even, odd);
        Texture::Checker(texture)
    }

    pub fn checker(even: Texture, odd: Texture) -> Texture {
        let texture = CheckerTexture::new(even, odd);
        Texture::Checker(texture)
    }
}

impl Texturable for Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        match self {
            Texture::Constant(texture) => texture.value(u, v, p),
            Texture::Checker(texture) => texture.value(u, v, p),
        }
    }
}
