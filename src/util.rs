use rand::Rng;

pub trait DRand48 {
    fn gen48(&mut self) -> f32;
}

impl<T: Rng> DRand48 for T {
    fn gen48(&mut self) -> f32 {
        self.gen_range(0.0, 1.0)
    }
}
