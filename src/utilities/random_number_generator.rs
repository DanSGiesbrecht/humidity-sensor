use rand::prelude::*;

pub struct RandomNumberGenerator {
    rng: SmallRng
}

impl RandomNumberGenerator {
    pub fn new(seed: u64) -> Self {
        RandomNumberGenerator{
            rng: SmallRng::seed_from_u64(seed)
        }
    }

    pub fn next(&mut self, range_bottom: u32, range_top: u32) -> u32 {
        self.rng.next_u32() % (range_top - range_bottom) + range_bottom
    }
}
