use alloc::arc::Arc;

use rng::Rng;

use super::atomic_prng::AtomicPrng;


#[derive(Clone)]
pub struct ThreadPrng {
    data: Arc<AtomicPrng>,
}

unsafe impl Send for ThreadPrng {}
unsafe impl Sync for ThreadPrng {}

impl ThreadPrng {
    #[inline]
    pub fn new() -> Self {
        ThreadPrng {
            data: Arc::new(AtomicPrng::new()),
        }
    }
    #[inline]
    pub fn seed(&self) -> usize {
        self.data.seed()
    }
    #[inline]
    pub fn set_seed(&self, seed: usize) {
        self.data.set_seed(seed);
    }
}

impl Rng for ThreadPrng {
    fn next(&mut self) -> usize {
        self.data.next_prn()
    }
}
