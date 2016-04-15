rs-pseudo_random
=====

pseudo random number generator

```rust
extern crate pseudo_random;


fn main() {
    pseudo_random::seed(24564764657); // some random number
    let r = pseudo_random::rand(); // usize number between 0 and usize MAX
    let n = (r as f32) / (std::usize::MAX as f32); // get float between 0.0 and 1.0
}

```
