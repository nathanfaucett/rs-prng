#![feature(alloc)]
#![no_std]


extern crate alloc;

extern crate rng;


mod atomic_prng;
mod prng;


pub use self::atomic_prng::*;
pub use self::prng::*;
