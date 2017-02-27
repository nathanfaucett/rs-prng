#![feature(alloc)]
#![no_std]


extern crate alloc;

#[macro_use]
extern crate lazy_static;
extern crate rng;


mod atomic_prng;
mod prng;
mod thread_prng;


pub use self::atomic_prng::*;
pub use self::prng::*;
pub use self::thread_prng::*;


lazy_static! {
    static ref THREAD_PRNG: ThreadPrng = ThreadPrng::new();
}


#[inline]
pub fn random() -> f64 {
    random_f64()
}
#[inline]
pub fn random_f32() -> f32 {
    THREAD_PRNG.next_prn() as f32 * rng::INV_MAX_F32
}
#[inline]
pub fn random_f64() -> f64 {
    THREAD_PRNG.next_prn() as f64 * rng::INV_MAX_F64
}
