extern crate pseudo_random;

extern crate rng;


use std::thread;
use pseudo_random::{MAX, ThreadPrng};
use rng::Rng;


#[test]
fn test_random_struct() {
    let mut random = ThreadPrng::new();

    if cfg!(target_pointer_width = "64") {
        random.set_seed(MAX / 2);
        assert_eq!(random.next(), 4301930853896946210);
        random.set_seed(1);
        assert_eq!(random.next(), 7806831264735756412);

        let mut r = random.clone();
        let child = thread::spawn(move || {
            assert_eq!(r.next(), 9396908728118811419);
        });
        match child.join() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }

        assert_eq!(random.next(), 11960119808228829710);
    } else if cfg!(target_pointer_width = "32") {
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
}
