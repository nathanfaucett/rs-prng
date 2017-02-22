use core::usize;

use rng::Rng;


// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
#[cfg(target_pointer_width = "32")]
pub const OFFSET: usize = 1013904223;
#[cfg(target_pointer_width = "32")]
pub const MULTIPLIER: usize = 1664525;

#[cfg(target_pointer_width = "64")]
pub const OFFSET: usize = 1442695040888963407;
#[cfg(target_pointer_width = "64")]
pub const MULTIPLIER: usize = 6364136223846793005;

pub const MAX: usize = usize::MAX as usize;


pub struct Prng {
    seed: usize,
}

impl Prng {
    #[inline]
    pub fn new() -> Self {
        Prng {
            // get a value for initial seed
            seed: &false as *const _ as usize,
        }
    }
    #[inline]
    pub fn seed(&self) -> usize { self.seed }
    #[inline]
    pub fn set_seed(&mut self, seed: usize) {
        self.seed = seed;
    }
}

impl Rng for Prng {
    // http://indiegamr.com/generate-repeatable-random-numbers-in-js/
    #[inline]
    fn next(&mut self) -> usize {
        self.seed = ((MULTIPLIER.wrapping_mul(self.seed)).wrapping_add(OFFSET)) % MAX;
        self.seed
    }
}
