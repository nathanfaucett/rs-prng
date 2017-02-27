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

#[bench]
fn test_thread_rand(b: &mut Bencher) {
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
fn test_thread_safe_prng(b: &mut Bencher) {
    use std::thread;
    use rng::Rng;
    use prng::ThreadPrng;

    let cpus = num_cpus::get();
    let random = ThreadPrng::new();

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
