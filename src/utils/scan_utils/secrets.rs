//! Scan personal informations and secrets in file contents.

use regex::Regex;

use super::utils::{Rule, diagnostic_by_regex};
use crate::utils::scan::ItemType as it;

use once_cell::sync::Lazy;

// API Token
pub static TOKEN_RULES: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
        "Personal Access Token",
        it::Secrets,
        vec![
            Regex::new(r"\w+_pat_[a-zA-Z0-9_\-]{30,85}").unwrap(),
            Regex::new(r"Bearer\s+(.+)").unwrap(),
            Regex::new(r#"let token\s*=\s*"\w+_pat_[a-zA-Z0-9_\-]{30,85}"\s*;"#).unwrap(),
            Regex::new(r#"let token\s*=\s*"Bearer\s+(.+)"\s*;"#).unwrap(),
        ],
    )
});

// Password
pub static PASSWORD_RULES: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
        "Password",
        it::Secrets,
        vec![
            Regex::new(r#"let password\s*=".*""#).unwrap(),
            Regex::new(r#"(password|passwd)\s*[:=]\s*["'][^"']+["']"#).unwrap(),
        ],
    )
});

pub static OTHER_RULES: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
    "Secret",
    it::Secrets,
    vec![
        // IPv4
        Regex::new(r#"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b"#).unwrap(),
        // URL embedded credentials
        Regex::new(r#"https?://[^"'`\s:@]+:[^"'`\s@]+@"#).unwrap(),
        // JWT
        Regex::new(r#"\beyJ[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\b"#).unwrap(),
    ]
)
});

/// Scan personal informations and secrets in file contents.
pub fn scan_secrets(s: &String, filename: &String) -> i32 {
    let filename = filename.as_str();
    let mut total_errors = 0;
    total_errors += diagnostic_by_regex(&TOKEN_RULES, s, filename);
    total_errors += diagnostic_by_regex(&PASSWORD_RULES, s, filename);
    total_errors += diagnostic_by_regex(&OTHER_RULES, s, filename);
    total_errors
}
