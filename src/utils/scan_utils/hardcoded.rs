//! Scan hard-coded contents.

use regex::Regex;

use super::utils::{Rule, diagnostic_by_regex};
use crate::utils::scan::ItemType::Hardcoded;

use once_cell::sync::Lazy;

/// Rules for matching hard-coded content.
pub static RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    vec![
    Rule::new(
        "Hard-coded IP address",
        Hardcoded,
        vec![
            Regex::new(r#"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b"#).unwrap(),
            Regex::new(r#"localhost:"#).unwrap(),
        ],
    ),
    Rule::new(
        "Hard-coded time zone",
        Hardcoded,
        vec![
            Regex::new(r#"\b(UTC|GMT)[+-]\d{1,2}(:\d{2})?\b"#).unwrap(),
            // IANA time zones
            Regex::new(r"^(?:Africa|America|Antarctica|Asia|Atlantic|Australia|Europe|Indian|Pacific)(?:/[A-Za-z0-9_-]+){1,2}$").unwrap()
        ]
    ),
    Rule::new(
        "Hard-coded URI",
        Hardcoded,
        vec![
            Regex::new(r#"https?://([a-zA-Z0-9_-]+\.)+[a-zA-Z0-9_-]+(/[^\s"'<>]*)?"#).unwrap(),
            Regex::new(r#"file:///[A-Za-z]:[/\\][^"'`\s]*"#).unwrap(),
        ],
    ),
    Rule::new(
        "Hard-coded path",
        Hardcoded,
        vec![
            Regex::new(r#"["'`] [A-Za-z]:[\\/][^"'`]*["'`]"#).unwrap(),
            Regex::new(r#"["'`]([A-Za-z]:[\\/][^"'`]*|/(?:[^"'`\\]+))["'`]"#).unwrap(),
        ],
    )
]
});

pub fn scan_hardcoded(s: &String, filename: &String) -> i32 {
    let mut total_error: i32 = 0;
    let filename = filename.as_str();
    for rule in &*RULES {
        total_error += diagnostic_by_regex(rule, s, filename);
    }
    total_error
}
