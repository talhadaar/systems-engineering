mod impls;

use impls::*;

#[test]
fn test_parallel_sum() {
    let n: u64 = if cfg!(miri) { 1_000 } else { 1_000_000 };
    let nums: Vec<u64> = (1..=n).collect();

    assert_eq!(parallel_sum(&nums), nums.iter().sum::<u64>());
    assert_eq!(parallel_sum(&[]), 0);
}

#[test]
fn test_shared_counter_v1() {
    let n = if cfg!(miri) { 100 } else { 100_000 };
    assert_eq!(shared_counter_v1(8, n), 8 * n);
}
