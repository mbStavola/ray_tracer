use rand::{rngs::SmallRng, Rng, SeedableRng};

pub fn drand48() -> f64 {
    // TODO(Matt): It'd be nice to only have to instantiate this *once* but I'd need to figure out
    // the issue of gen_range() requiring a mutable reference
    let mut rng = SmallRng::from_entropy();
    rng.gen48()
}

pub trait DRand48 {
    fn gen48(&mut self) -> f64;
}

impl<T: Rng> DRand48 for T {
    fn gen48(&mut self) -> f64 {
        self.gen_range(0.0, 1.0)
    }
}
