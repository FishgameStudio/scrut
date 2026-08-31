//! Generates random numbers.

use rand::{self, RngExt};

use std::ops::Range;

/// Generates a secure random integer in the given range.
pub fn gen_randint(range: Range<i32>) -> i32 {
    let mut rng = rand::rng();
    rng.random_range::<i32, Range<i32>>(range)
}
/// Generates a secure random float point number in the given range.
pub fn gen_randfloat(range: Range<f64>) -> f64 {
    let mut rng = rand::rng();
    rng.random_range::<f64, Range<f64>>(range)
}
