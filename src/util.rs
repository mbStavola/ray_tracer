use rand::Rng;

pub trait RandomDouble {
    fn random_double(&mut self) -> f64;
}

impl<T: Rng> RandomDouble for T {
    fn random_double(&mut self) -> f64 {
        self.gen_range(0.0, 1.0)
    }
}
