//! Generates random numbers.

use rand::{self, RngExt};

use std::ops::Range;

use crate::utils::logging::verbose;

/// Generates a secure random integer in the given range.
pub fn gen_randint(range: Range<i32>) -> i32 {
    verbose!("Randoming integer in range {range:?}");
    rand::rng().random_range::<i32, Range<i32>>(range)
}
/// Generates a secure random float point number in the given range.
pub fn gen_randfloat(range: Range<f64>) -> f64 {
    verbose!("Randoming floating-point number in range {range:?}");
    rand::rng().random_range::<f64, Range<f64>>(range)
}

/// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn random_test() {
        use super::*;
        let i = gen_randint(1..100);
        let f = gen_randfloat(1.9..25.2);
        assert!(0 < i && i < 100);
        assert!(1.9 <= f && f < 25.2);
    }
}
