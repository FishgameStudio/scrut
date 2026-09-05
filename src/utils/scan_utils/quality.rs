//! Scan the codes including `todof`, etc.

use regex::Regex;

use super::utils::{Rule, diagnostic_by_regex};
use crate::utils::scan::ItemType::Quality;

use once_cell::sync::Lazy;

/// Rules to check code quality.
pub static RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    vec![
        Rule::new(
            "Usage of `while true` instead of `loop`",
            Quality,
            vec![Regex::new(r#"\bwhile\s+true\s+\{"#).unwrap()],
        ),
        Rule::new(
            "Usage of macro `dbg`",
            Quality,
            vec![Regex::new(r#"dbg!\("#).unwrap()],
        ),
        Rule::new(
            "Empty loop",
            Quality,
            vec![Regex::new(r#"loop\{\s*\}"#).unwrap()],
        ),
        Rule::new(
            "Unimplemented branch",
            Quality,
            vec![Regex::new(r#"unimplemented!\("#).unwrap()],
        ),
        Rule::new(
            "Unreanchable branch",
            Quality,
            vec![
                Regex::new(r#"unreachable!\("#).unwrap(),
                Regex::new(r#"unreachable_unchecked\("#).unwrap(),
            ],
        ),
        Rule::new(
            "Usage of unsafe blocks",
            Quality,
            vec![Regex::new(r#"\bunsafe\s*\{"#).unwrap()],
        ),
        Rule::new(
            "Bad memory management",
            Quality,
            vec![
                Regex::new(r#"mem::forget\("#).unwrap(),
                Regex::new(r#"ptr::drop_in_place\("#).unwrap(),
            ],
        ),
        Rule::new(
            "Comparison of boolean values",
            Quality,
            vec![Regex::new(r#"==\s*(true|false)\b"#).unwrap()],
        ),
        Rule::new(
            "Double semicolons",
            Quality,
            vec![Regex::new(r#";;"#).unwrap()],
        ),
        Rule::new(
            "Usage of macro `todo`",
            Quality,
            vec![Regex::new(r#"todo!\("#).unwrap()],
        ),
        Rule::new(
            "Comment tags",
            Quality,
            vec![
                Regex::new(r"//\s*TODO\b").unwrap(),
                Regex::new(r"//\s*FIXME\b").unwrap(),
                Regex::new(r"//\s*HACK\b").unwrap(),
                Regex::new(r"//\s*BUG\b").unwrap(),
                Regex::new(r"//\s*XXX\b").unwrap(),
            ],
        ),
    ]
});

pub fn scan_quality(s: &String, filename: &String) -> i32 {
    let mut total_error: i32 = 0;
    let filename = filename.as_str();
    for rule in &*RULES {
        total_error += diagnostic_by_regex(rule, s, filename);
    }
    total_error
}
