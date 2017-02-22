#![feature(custom_attribute)]
#![no_std]


#[cfg(feature = "thread_prng")]
extern crate std;

extern crate rng;


mod prng;
#[cfg(feature = "thread_prng")]
mod thread_prng;


pub use prng::*;
#[cfg(feature = "thread_prng")]
pub use thread_prng::ThreadPrng;
