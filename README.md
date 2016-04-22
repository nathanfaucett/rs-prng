rs-pseudo_random
=====

pseudo random number generator

```rust
extern crate pseudo_random;


fn main() {
    let random = pseudo_random::Random::new();
    
    random.seed(24564764657); // some random number
    
    let v = random.rand(); // usize number between 0 and usize MAX
    let n = (v as f32) / (std::usize::MAX as f32); // get float between 0.0 and 1.0
}

```
