//! Scan hard-coded contents.

use regex::Regex;

use super::utils::{Rule, diagnostic_by_regex};
use crate::utils::scan::ItemType as it;

use once_cell::sync::Lazy;

/// Path
pub static PATH: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
        "Hard-coded Path",
        it::Hardcoded,
        vec![
            Regex::new(r#"["'`] [A-Za-z]:[\\/][^"'`]*["'`]"#).unwrap(),
            Regex::new(r#"["'`]([A-Za-z]:[\\/][^"'`]*|/(?:[^"'`\\]+))["'`]"#).unwrap(),
        ],
    )
});

/// URI
pub static URI: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
        "Hard-coded URI",
        it::Hardcoded,
        vec![
            Regex::new(r#"https?://([a-zA-Z0-9_-]+\.)+[a-zA-Z0-9_-]+(/[^\s"'<>]*)?"#).unwrap(),
            Regex::new(r#"file:///[A-Za-z]:[/\\][^"'`\s]*"#).unwrap(),
        ],
    )
});
pub static OTHER: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
        "Hard-coded content",
        it::Hardcoded,
        // Email address
        vec![
            Regex::new(r#"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b"#).unwrap(),
            // Time zone
            Regex::new(r#"\b(UTC|GMT)[+-]\d{1,2}(:\d{2})?\b"#).unwrap(),
        ],
    )
});

pub fn scan_hardcoded(s: &String, filename: &String) -> i32 {
    let mut total_error: i32 = 0;
    total_error += diagnostic_by_regex(&*PATH, s, filename);
    total_error += diagnostic_by_regex(&*URI, s, filename);
    total_error += diagnostic_by_regex(&*OTHER, s, filename);
    total_error
}
