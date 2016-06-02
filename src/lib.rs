#![no_std]


pub use core::usize::MAX;


// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
#[cfg(target_pointer_width = "32")]
static OFFSET: usize = 1013904223;
#[cfg(target_pointer_width = "32")]
static MULTIPLIER: usize = 1664525;


#[cfg(target_pointer_width = "64")]
static OFFSET: usize = 1442695040888963407;
#[cfg(target_pointer_width = "64")]
static MULTIPLIER: usize = 6364136223846793005;


pub struct Random {
    seed: usize,
}

impl Random {
    #[inline(always)]
    pub fn new() -> Self {
        Random {
            seed: MAX / 2,
        }
    }

    // http://indiegamr.com/generate-repeatable-random-numbers-in-js/
    #[inline(always)]
    pub fn next(&mut self) -> usize {
        self.seed = (MULTIPLIER * self.seed + OFFSET) % MAX;
        self.seed
    }

    #[inline(always)]
    pub fn get_seed(&self) -> usize {
        self.seed
    }
    #[inline(always)]
    pub fn set_seed(&mut self, s: usize) {
        self.seed = s;
    }
}

#[test]
fn test_random_struct() {
    let mut random = Random::new();

    if cfg!(target_pointer_width = "64") {
        assert_eq!(random.next(), 4301930853896946210);
        random.set_seed(1);
        assert_eq!(random.next(), 7806831264735756412);
    } else if cfg!(target_pointer_width = "32") {
        assert_eq!(random.next(), 3159723346);
        random.set_seed(1);
        assert_eq!(random.next(), 1015568748);
    }
}
