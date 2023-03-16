/*
    A short program for calculating the digits of pi,
    using exact rather than floating point calculations.
    This is impractical but illustrative of some of the challenges
    working with exact real numbers.
*/

use std::iter;

fn min_int(pred: impl Fn(usize) -> bool) -> usize {
    // Return the minimum integer that satisfies a predicate, assuming
    // the predicate is increasing, i.e. false exactly for some [0, ..., n-1]
    // Running time is O(log USIZE_MAX).
    if pred(0) {
        return 0;
    }
    let mut upper = iter::successors(Some(1), |&x| Some(2 * x))
        .find(|&x| pred(x))
        .unwrap();
    let mut step = upper;
    // Interval for result: (upper - step, upper];
    debug_assert!(pred(upper) && !pred(upper - step));
    while step > 1 {
        step /= 2;
        if pred(upper - step) {
            upper -= step;
        }
        debug_assert!(pred(upper) && !pred(upper - step));
    }
    upper
}
#[test]
fn test_min_int() {
    assert_eq!(min_int(|_| true), 0);
    for n in 0..100 {
        assert_eq!(min_int(|x| x >= n), n);
    }
    assert_eq!(min_int(|x| x * x >= 2021), 45);
}

fn floor_sqrt(n: usize) -> usize {
    min_int(|x| x * x > n) - 1
}
#[test]
fn test_floor_sqrt() {
    assert_eq!(floor_sqrt(0), 0);
    assert_eq!(floor_sqrt(1), 1);
    assert_eq!(floor_sqrt(2), 1);
    assert_eq!(floor_sqrt(3), 1);
    assert_eq!(floor_sqrt(4), 2);
    assert_eq!(floor_sqrt(9), 3);
    assert_eq!(floor_sqrt(10), 3);
    assert_eq!(floor_sqrt(2021), 44);
}

fn ceil_sqrt(n: usize) -> usize {
    min_int(|x| x * x >= n)
}
#[test]
fn test_ceil_sqrt() {
    assert_eq!(ceil_sqrt(0), 0);
    assert_eq!(ceil_sqrt(1), 1);
    assert_eq!(ceil_sqrt(2), 2);
    assert_eq!(ceil_sqrt(3), 2);
    assert_eq!(ceil_sqrt(4), 2);
    assert_eq!(ceil_sqrt(9), 3);
    assert_eq!(ceil_sqrt(10), 4);
    assert_eq!(ceil_sqrt(2021), 45);
}

fn approx_pi(scale: u32) {
    // count squares in [1, ..., n] x [1, ..., n] to bound area of
    // quarter-circle of radius n about 0.
    let n = 10_usize.pow(scale);
    let area = n * n;
    let mut lower = 0;
    let mut higher = 0;
    let f = |i: usize| n * n - i * i;
    for i in 0..n {
        higher += ceil_sqrt(f(i));
        lower += floor_sqrt(f(i + 1));
    }
    // lower / area <= pi / 4 <= higher / area
    higher *= 4;
    lower *= 4;
    // Print out
    let low_int = lower / area;
    let low_dec = lower % area;
    let high_int = higher / area;
    let high_dec = higher % area;
    // this printing doesn't handle the case of leading zeros
    assert!(low_dec >= area / 10);
    assert!(high_dec >= area / 10);
    println!("{}.{} < pi < {}.{}", low_int, low_dec, high_int, high_dec);
}

fn main() {
    if cfg!(debug_assertions) {
        println!("*** Warning: slow in debug mode. Try cargo run --release ***")
    }
    for i in 0..8 {
        approx_pi(i);
    }
}
