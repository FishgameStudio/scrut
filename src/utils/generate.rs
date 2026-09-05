//! Generates specified items.

use crate::utils::gen_utils::{
    password::gen_password, rand::gen_randfloat, rand::gen_randint, sha256::gen_checksums,
    sha256::gen_sha256,
};
use crate::utils::logging::{fatal, verbose};

use std::fs::write;
use std::ops::Range;

use owo_colors::OwoColorize;

#[derive(Debug)]
pub enum GenerationItemType {
    Password(usize),
    RandInt(Range<i32>),
    RandFloat(Range<f64>),
    Sha256(String),
    Checksum,
}

/// Convert an item name to a enumeration.
/// # Panics
/// If the name is unknown.
pub fn str2enum(
    s: &str,
    password_len: Option<usize>,
    randint_range: Option<Range<i32>>,
    randfloat_range: Option<Range<f64>>,
    content: Option<String>,
) -> GenerationItemType {
    verbose!("Matching string {s}");
    use GenerationItemType::*;
    match s {
        "password" => Password(match password_len {
            Some(len) => len,
            None => fatal!("Argument 'len' is required if the item specified as 'password'"),
        }),
        "randint" => RandInt(match randint_range {
            Some(range) => range,
            None => fatal!("Argument 'range' is required if the item specified as 'randint'"),
        }),
        "randfloat" => RandFloat(match randfloat_range {
            Some(range) => range,
            None => fatal!("Argument 'range' is required if the item specified as 'randfloat'"),
        }),
        "sha256" => Sha256(match content {
            Some(content) => content,
            None => fatal!(
                "One of arguments 'content' and 'from-file' is required if the item specified as 'sha256'"
            ),
        }),
        "checksum" => Checksum,
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
        Sha256(content) => gen_sha256(&content),
        Checksum => match gen_checksums() {
            Ok(s) => {
                // Write to the file `SHA256SUMS` first.
                const CHECKSUMS_FILE: &str = "./SHA256SUMS";
                verbose!("Saving checksums to file {CHECKSUMS_FILE:?} ...");
                if let Err(e) = write("./SHA256SUMS", &s) {
                    fatal!("Failed to save checksums to file {CHECKSUMS_FILE}: {e}");
                } else {
                    println!("{} {CHECKSUMS_FILE}", "Saved checksums to file: ".green());
                }
                String::new() // Return an empty string and nothing will printed later.
            }
            Err(e) => fatal!("Failed to generate checksums: {e}"),
        },
    }
}
