use std::sync::{Arc, Mutex};

use rng::Rng;
use prng::Prng;


#[derive(Clone)]
pub struct ThreadPrng {
    data: Arc<Mutex<Prng>>,
}

impl ThreadPrng {

    pub fn new() -> Self {
        ThreadPrng {
            data: Arc::new(Mutex::new(Prng::new())),
        }
    }

    pub fn seed(&self) -> usize {
        match self.data.lock() {
            Ok(guard) => guard.seed(),
            Err(..) => 0usize,
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
            Err(..) => 0usize,
        }
    }

    fn next_f32(&mut self) -> f32 {
        match self.data.lock() {
            Ok(mut guard) => guard.next_f32(),
            Err(..) => 0f32,
        }
    }

    fn next_f64(&mut self) -> f64 {
        match self.data.lock() {
            Ok(mut guard) => guard.next_f64(),
            Err(..) => 0f64,
        }
    }
}

unsafe impl Send for ThreadPrng {}
unsafe impl Sync for ThreadPrng {}
