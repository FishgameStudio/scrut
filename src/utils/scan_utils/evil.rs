//! Scan malicious contents.

use regex::Regex;

use super::utils::{Rule, diagnostic_by_regex};
use crate::utils::scan::ItemType::Evil;

use once_cell::sync::Lazy;

/// Rules for matching malicious content.
pub static RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    vec![
        Rule::new(
            "Unconditionally delete files",
            Evil,
            vec![
                Regex::new(r"\brm -rf\b").unwrap(),
                Regex::new(r"\bdel /f /s\b").unwrap(),
            ],
        ),
        Rule::new(
            "Unsafe raw pointet operations",
            Evil,
            vec![
                Regex::new(r"\bmem::transmute\b").unwrap(),
                Regex::new(r"\bptr::(write|copy|copy_nonoverlapping)\b").unwrap(),
            ],
        ),
        Rule::new(
            "Attempt to set file permissions to 0o777",
            Evil,
            vec![Regex::new(r"\bchmod 777\b").unwrap()],
        ),
    ]
});

pub fn scan_evil(s: &String, filename: &String) -> i32 {
    let mut total_error: i32 = 0;
    let filename = filename.as_str();
    for rule in &*RULES {
        total_error += diagnostic_by_regex(rule, s, filename);
    }
    total_error
}
