#![feature(test)]


extern crate test;
extern crate rand;
extern crate num_cpus;

extern crate prng;
extern crate rng;


use test::Bencher;


#[bench]
fn test_prng(b: &mut Bencher) {
    use rng::Rng;
    use prng::Prng;
    let mut random = Prng::new();

    b.iter(|| {
        random.next_f64()
    });
}
#[bench]
fn test_rand(b: &mut Bencher) {
    use rand::Rng;
    let mut random = rand::weak_rng();

    b.iter(|| {
        random.next_f64()
    });
}
