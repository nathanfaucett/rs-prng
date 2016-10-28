#![no_std]
#![feature(alloc, custom_attribute)]


extern crate alloc;
extern crate spin;

extern crate rng;


use alloc::arc::Arc;
use core::usize;

use spin::Mutex;

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

    #[inline(always)]
    pub fn new() -> Self {
        Prng {
            seed: MAX / 2,
        }
    }

    #[inline(always)]
    pub fn seed(&self) -> usize {
        self.seed
    }
    #[inline(always)]
    pub fn set_seed(&mut self, s: usize) {
        self.seed = s;
    }
}

impl Rng for Prng {
    // http://indiegamr.com/generate-repeatable-random-numbers-in-js/
    #[inline(always)]
    fn next(&mut self) -> usize {
        self.seed = ((MULTIPLIER.wrapping_mul(self.seed)).wrapping_add(OFFSET)) % MAX;
        self.seed
    }

    #[inline(always)]
    fn next_f32(&mut self) -> f32 { self.next() as f32 / MAX_F32 }
    #[inline(always)]
    fn next_f64(&mut self) -> f64 { self.next() as f64 / MAX_F64 }
}


#[derive(Clone)]
pub struct ThreadPrng {
    data: Arc<Mutex<Prng>>,
}

impl ThreadPrng {

    #[inline(always)]
    pub fn new() -> Self {
        ThreadPrng {
            data: Arc::new(Mutex::new(Prng::new())),
        }
    }

    #[inline(always)]
    pub fn seed(&self) -> usize {
        self.data.lock().seed()
    }
    #[inline(always)]
    pub fn set_seed(&self, s: usize) {
        self.data.lock().set_seed(s);
    }
}

impl Rng for ThreadPrng {
    #[inline(always)]
    fn next(&mut self) -> usize { self.data.lock().next() }
    #[inline(always)]
    fn next_f32(&mut self) -> f32 { self.data.lock().next_f32() }
    #[inline(always)]
    fn next_f64(&mut self) -> f64 { self.data.lock().next_f64() }
}

unsafe impl Send for ThreadPrng {}
unsafe impl Sync for ThreadPrng {}
