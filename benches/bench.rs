#![feature(test)]


extern crate test;
extern crate rand;
extern crate num_cpus;

extern crate pseudo_random;
extern crate rng;


use test::Bencher;


#[bench]
fn test_pseudo_random(b: &mut Bencher) {
    use rng::Rng;
    use pseudo_random::Prng;

    let mut random = Prng::new();

    b.iter(|| {
        random.next()
    });
}

#[bench]
fn test_rand(b: &mut Bencher) {
    use std::thread;

    let cpus = num_cpus::get();

    b.iter(|| {
        let mut handles = Vec::new();

        for _ in 0..cpus {
            handles.push(thread::spawn(|| rand::random::<usize>()));
        }

        for handle in handles {
            let _ = handle.join();
        }
    });
}
#[bench]
fn test_thread_safe_pseudo_random(b: &mut Bencher) {
    use std::thread;
    use rng::Rng;
    use pseudo_random::ThreadPrng;

    let cpus = num_cpus::get();
    let mut random = ThreadPrng::new();

    b.iter(|| {
        let mut handles = Vec::new();

        for _ in 0..cpus {
            let mut random = random.clone();
            handles.push(thread::spawn(move || random.next()));
        }

        for handle in handles {
            let _ = handle.join();
        }
    });
}
