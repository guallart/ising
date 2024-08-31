use crate::constants::L;
use crate::types_def::Index;
use rand::{rngs::ThreadRng, Rng};

pub struct RandomGenerator {
    rng: ThreadRng,
}

impl RandomGenerator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn fran(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }

    pub fn bran(&mut self) -> bool {
        self.rng.gen::<bool>()
    }

    pub fn index(&mut self) -> Index {
        (self.rng.gen_range(0..L), self.rng.gen_range(0..L))
    }
}
