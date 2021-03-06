extern crate prng;
extern crate rng;


use std::thread;
use prng::{MAX, AtomicPrng};
use rng::Rng;


#[cfg(target_pointer_width = "32")]
#[test]
fn test_random_struct() {
    let mut random = AtomicPrng::new();

    random.set_seed(MAX / 2);
    assert_eq!(random.next(), 718406178);
    random.set_seed(1);
    assert_eq!(random.next(), 1140654204);

    let mut r = random.clone();
    let child = thread::spawn(move || {
        assert_eq!(r.next(), 2253003547);
    });
    match child.join() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    assert_eq!(random.next(), 546810382);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_random_struct() {
    let mut random = AtomicPrng::new();

    random.set_seed(MAX / 2);
    assert_eq!(Rng::next(&mut random), 4301930853896946210);
    random.set_seed(1);
    assert_eq!(random.next(), 7806831264735756412);

    let r = random.clone();
    let child = thread::spawn(move || {
        assert_eq!(r.next(), 9396908728118811419);
    });
    match child.join() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    assert_eq!(random.next(), 11960119808228829710);
}
