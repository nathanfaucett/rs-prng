use core::usize;

use rng::Rng;


// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
#[cfg(target_pointer_width = "32")]
pub static OFFSET: usize = 1013904223;
#[cfg(target_pointer_width = "32")]
pub static MULTIPLIER: usize = 1664525;


#[cfg(target_pointer_width = "64")]
pub static OFFSET: usize = 1442695040888963407;
#[cfg(target_pointer_width = "64")]
pub static MULTIPLIER: usize = 6364136223846793005;

pub static MAX: usize = usize::MAX;
pub static MAX_F32: f32 = usize::MAX as f32;
pub static MAX_F64: f64 = usize::MAX as f64;


pub struct Prng {
    seed: usize,
}

impl Prng {

    pub fn new() -> Self {
        Prng {
            // get a value for initial seed
            seed: &false as *const _ as usize,
        }
    }

    pub fn seed(&self) -> usize {
        self.seed
    }

    pub fn set_seed(&mut self, seed: usize) {
        self.seed = seed;
    }
}

impl Rng for Prng {
    // http://indiegamr.com/generate-repeatable-random-numbers-in-js/
    fn next(&mut self) -> usize {
        self.seed = ((MULTIPLIER.wrapping_mul(self.seed)).wrapping_add(OFFSET)) % MAX;
        self.seed
    }

    fn next_f32(&mut self) -> f32 { self.next() as f32 / MAX_F32 }

    fn next_f64(&mut self) -> f64 { self.next() as f64 / MAX_F64 }
}
