rs-pseudo_random
=====

pseudo random number generator

```rust
extern crate pseudo_random;


fn main() {
    let random = pseudo_random::Random::new();

    random.set_seed(24564764657); // some random unsigned integer

    let x = random.next(); // usize number between 0 and usize MAX
    let y = random.next_f32(); // f32 between 0 and 1
    let z = random.next_f64(); // f64 between 0 and 1
}

```
