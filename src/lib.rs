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


pub struct Random {
    _seed: usize,
}

impl Random {
    #[inline(always)]
    pub fn new() -> Random {
        Random {
            _seed: USIZE_MAX / 2,
        }
    }

    // http://indiegamr.com/generate-repeatable-random-numbers-in-js/
    #[inline(always)]
    pub fn rand(&mut self) -> usize {
        self._seed = (MULTIPLIER * self._seed + OFFSET) % USIZE_MAX;
        self._seed
    }

    #[inline(always)]
    pub fn seed(&mut self, s: usize) -> &mut Self {
        self._seed = s;
        self
    }
}

#[test]
fn test_random() {
    let mut random = Random::new();

    if cfg!(target_pointer_width = "64") {
        assert_eq!(random.rand(), 4301930853896946210);
        random.seed(4301930853896946210);
        assert_eq!(random.rand(), 3578485316352917321);
    } else if cfg!(target_pointer_width = "32") {
        assert_eq!(random.rand(), 3159723346);
        random.seed(3159723346);
        assert_eq!(random.rand(), 2954349705);
    }
}
