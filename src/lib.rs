#![no_std]
#![feature(alloc)]


extern crate alloc;


use alloc::arc::Arc;
use core::cell::RefCell;
use core::usize::MAX;


// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
#[cfg(target_pointer_width = "32")]
static OFFSET: usize = 1013904223;
#[cfg(target_pointer_width = "32")]
static MULTIPLIER: usize = 1664525;


#[cfg(target_pointer_width = "64")]
static OFFSET: usize = 1442695040888963407;
#[cfg(target_pointer_width = "64")]
static MULTIPLIER: usize = 6364136223846793005;


struct RandomData {
    seed: usize,
}
impl RandomData {
    #[inline(always)]
    fn new() -> Self {
        RandomData {
            seed: MAX / 2,
        }
    }

    #[inline(always)]
    fn seed(&self) -> usize {
        self.seed
    }
    #[inline(always)]
    fn set_seed(&mut self, s: usize) {
        self.seed = s;
    }

    // http://indiegamr.com/generate-repeatable-random-numbers-in-js/
    #[inline(always)]
    fn next(&mut self) -> usize {
        self.seed = (MULTIPLIER * self.seed + OFFSET) % MAX;
        self.seed
    }
}


#[derive(Clone)]
pub struct Random {
    data: Arc<RefCell<RandomData>>,
}
impl Random {
    #[inline(always)]
    pub fn new() -> Self {
        Random {
            data: Arc::new(RefCell::new(RandomData::new())),
        }
    }

    #[inline(always)]
    pub fn seed(&self) -> usize {
        self.data.borrow().seed()
    }
    #[inline(always)]
    pub fn set_seed(&self, s: usize) {
        self.data.borrow_mut().set_seed(s);
    }

    #[inline(always)]
    pub fn next(&self) -> usize {
        self.data.borrow_mut().next()
    }
    #[inline(always)]
    pub fn next_f32(&self) -> f32 {
        self.next() as f32 / MAX as f32
    }
    #[inline(always)]
    pub fn next_f64(&self) -> f64 {
        self.next() as f64 / MAX as f64
    }
}

unsafe impl Send for Random {}
unsafe impl Sync for Random {}
