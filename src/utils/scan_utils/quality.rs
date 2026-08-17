//! Scan the codes including `todof`, etc.

use regex::Regex;

use super::utils::{Rule, diagnostic_by_regex};
use crate::utils::scan::ItemType as it;

use once_cell::sync::Lazy;

pub static BAD: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
        "Non-standard code",
        it::Quality,
        vec![
            // Non-standard code
            Regex::new(r#"\bwhile\s+true\s+\{"#).unwrap(),
            Regex::new(r#"dbg!\("#).unwrap(),
            Regex::new(r#"todo!\("#).unwrap(),
            Regex::new(r#"loop\{\s*\}"#).unwrap(),
            Regex::new(r#"unimplemented!\("#).unwrap(),
            Regex::new(r#"unreachable!\("#).unwrap(),
            Regex::new(r#"unreachable_unchecked\("#).unwrap(),
            Regex::new(r#"mem::forget\("#).unwrap(),
            Regex::new(r#"\bunsafe\s*\{"#).unwrap(),
            Regex::new(r#"==\s*(true|false)\b"#).unwrap(),
            Regex::new(r#";;"#).unwrap(),
        ],
    )
});
pub static TODO: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
        "Non-standard code",
        it::Quality,
        vec![
            Regex::new(r"\/\/\s*TODO\b").unwrap(),
            Regex::new(r"\/\/\s*FIXME\b").unwrap(),
            Regex::new(r"\/\/\s*HACK\b").unwrap(),
            Regex::new(r"\/\/\s*BUG\b").unwrap(),
            Regex::new(r"\/\/\s*XXX\b").unwrap(),
        ],
    )
});

pub fn scan_quality(s: &String, filename: &String) -> i32 {
    let mut total_error: i32 = 0;
    total_error += diagnostic_by_regex(&*BAD, s, filename);
    total_error += diagnostic_by_regex(&*TODO, s, filename);
    total_error
}
