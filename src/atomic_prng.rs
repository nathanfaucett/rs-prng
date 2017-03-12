use alloc::arc::Arc;

use core::sync::atomic::{AtomicUsize, Ordering};

use rng::Rng;

use super::prng::{MULTIPLIER, OFFSET, MAX};


pub struct AtomicPrng {
    seed: Arc<AtomicUsize>,
}

impl AtomicPrng {
    #[inline]
    pub fn new() -> Self {
        AtomicPrng {
            // get a value for initial seed
            seed: Arc::new(AtomicUsize::new(&false as *const _ as usize)),
        }
    }
    #[inline]
    pub fn seed(&self) -> usize {
        self.seed.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn set_seed(&self, seed: usize) {
        self.seed.store(seed, Ordering::Relaxed);
    }
    #[inline]
    pub fn next_prn(&self) -> usize {
        let seed = &self.seed;
        let current_seed = seed.load(Ordering::Relaxed);
        let next_seed = ((MULTIPLIER.wrapping_mul(current_seed)).wrapping_add(OFFSET)) % MAX;
        seed.compare_and_swap(current_seed, next_seed, Ordering::Relaxed);
        next_seed
    }
}

impl Clone for AtomicPrng {
    #[inline(always)]
    fn clone(&self) -> Self {
        AtomicPrng {
            seed: self.seed.clone(),
        }
    }
}

impl Rng for AtomicPrng {
    #[inline(always)]
    fn next(&mut self) -> usize {
        self.next_prn()
    }
}
