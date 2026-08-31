//! Generates specified items.

use crate::utils::gen_utils::{password::gen_password, rand::gen_randfloat, rand::gen_randint};
use crate::utils::logging::{fatal, verbose};

use std::ops::Range;

#[derive(Debug)]
pub enum GenerationItemType {
    Password(usize),
    RandInt(Range<i32>),
    RandFloat(Range<f64>),
}

/// Convert an item name to a enumeration.
/// # Panics
/// If the name is unknown.
pub fn str2enum(
    s: &str,
    password_len: Option<usize>,
    randint_range: Option<Range<i32>>,
    randfloat_range: Option<Range<f64>>,
) -> GenerationItemType {
    verbose!("Matching string {s}");
    use GenerationItemType::*;
    match s {
        "password" => Password(password_len.unwrap()),
        "randint" => RandInt(randint_range.unwrap()),
        "randfloat" => RandFloat(randfloat_range.unwrap()),
        item => fatal!("Unknown type of item: {item}"),
    }
}

/// Generate a specified item.
pub fn generate(item: GenerationItemType) -> String {
    verbose!("Generating item '{item:?}' ...");
    use GenerationItemType::*;
    match item {
        Password(len) => gen_password(len),
        RandInt(range) => gen_randint(range).to_string(),
        RandFloat(range) => gen_randfloat(range).to_string(),
    }
}
