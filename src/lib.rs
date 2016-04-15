#![no_std]


use core::usize::MAX as USIZE_MAX;


// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use

#[cfg(target_pointer_width = "32")]
static OFFSET: usize = 1013904223;
#[cfg(target_pointer_width = "32")]
static MULTIPLIER: usize = 1664525;


#[cfg(target_pointer_width = "64")]
static OFFSET: usize = 1442695040888963407;
#[cfg(target_pointer_width = "64")]
static MULTIPLIER: usize = 6364136223846793005;


static mut SEED: usize = USIZE_MAX / 2usize;


// http://indiegamr.com/generate-repeatable-random-numbers-in-js/
#[inline(always)]
pub fn rand() -> usize {
    unsafe {
        SEED = (MULTIPLIER * SEED + OFFSET) % USIZE_MAX;
        SEED
    }
}

#[inline(always)]
pub fn srand(s: usize) {
    unsafe {
        SEED = s;
    }
}

#[test]
fn test_rand() {
    if cfg!(target_pointer_width = "64") {
        assert_eq!(rand(), 4301930853896946210);
        srand(4301930853896946210);
        assert_eq!(rand(), 3578485316352917321);
    } else if cfg!(target_pointer_width = "32") {
        assert_eq!(rand(), 3159723346);
        srand(3159723346);
        assert_eq!(rand(), 2954349705);
    }
}
