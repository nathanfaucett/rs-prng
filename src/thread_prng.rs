use std::sync::{Arc, Mutex};

use rng::Rng;
use prng::Prng;


#[derive(Clone)]
pub struct ThreadPrng {
    data: Arc<Mutex<Prng>>,
}

unsafe impl Send for ThreadPrng {}
unsafe impl Sync for ThreadPrng {}

impl ThreadPrng {

    pub fn new() -> Self {
        ThreadPrng {
            data: Arc::new(Mutex::new(Prng::new())),
        }
    }

    pub fn seed(&self) -> usize {
        match self.data.lock() {
            Ok(guard) => guard.seed(),
            Err(..) => 0,
        }
    }

    pub fn set_seed(&self, seed: usize) {
        match self.data.lock() {
            Ok(mut guard) => guard.set_seed(seed),
            Err(..) => (),
        }
    }
}

impl Rng for ThreadPrng {
    fn next(&mut self) -> usize {
        match self.data.lock() {
            Ok(mut guard) => guard.next(),
            Err(..) => 0,
        }
    }
}
