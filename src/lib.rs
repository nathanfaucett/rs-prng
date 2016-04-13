#![no_std]


static mut seed: u64 = 19940046431;
use core::usize::MAX;


#[inline(always)]
pub fn rand() -> usize {
    unsafe {
        seed ^= seed << 12;
        seed ^= seed << 25;
        seed ^= seed << 27;
        seed = seed * 82724793451 + 12345;
        (seed as usize) % MAX
    }
}

#[inline(always)]
pub fn srand(s: usize) {
    unsafe {
        seed = s as u64;
    }
}

#[test]
fn test_rand() {
    assert_eq!(rand(), 9480282067609464302);
    srand(9480282067609464302);
    assert_eq!(rand(), 13439875519330593459);
}
