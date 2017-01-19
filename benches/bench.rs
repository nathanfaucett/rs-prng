#![feature(test)]


extern crate test;
extern crate rand;

extern crate pseudo_random;
extern crate rng;


use test::Bencher;
use pseudo_random::{Prng, ThreadPrng};


#[bench]
fn test_pseudo_random(b: &mut Bencher) {
    use rng::Rng;
    let mut random = Prng::new();
    
    b.iter(|| {
        random.next();
    });
}
#[bench]
fn test_rand(b: &mut Bencher) {
    use rand::Rng;
    let mut random = rand::weak_rng();
    
    b.iter(|| {
        random.next_u64();
    });
}

#[bench]
fn test_thread_safe_pseudo_random(b: &mut Bencher) {
    use rng::Rng;
    let mut random = ThreadPrng::new();
    
    b.iter(|| {
        random.next();
    });
}
#[bench]
fn test_thread_safe_rand(b: &mut Bencher) {
    use rand::Rng;
    let mut random = rand::thread_rng();
    
    b.iter(|| {
        random.next_u64();
    });
}
