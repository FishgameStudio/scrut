//! Scan personal informations and secrets in file contents.

use regex::Regex;

use super::utils::{Rule, diagnostic_by_regex};
use crate::utils::scan::ItemType::Secrets;

use once_cell::sync::Lazy;

/// Rules for scanning secrets.
pub static RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    vec![
        Rule::new(
            "Personal access token",
            Secrets,
            vec![
                Regex::new(r"\w+_pat_[a-zA-Z0-9_\-]{30,85}").unwrap(),
                Regex::new(r"Bearer\s+(.+)").unwrap(),
                Regex::new(r#"let token\s*=\s*"\w+_pat_[a-zA-Z0-9_\-]{30,85}"\s*;"#).unwrap(),
                Regex::new(r#"let token\s*=\s*"Bearer\s+(.+)"\s*;"#).unwrap(),
            ],
        ),
        Rule::new(
            "Password",
            Secrets,
            vec![
                Regex::new(r#"let password\s*=".*""#).unwrap(),
                Regex::new(r#"(password|passwd)\s*[:=]\s*["'][^"']+["']"#).unwrap(),
            ],
        ),
        Rule::new(
            "JSON web token",
            Secrets,
            vec![
                Regex::new(r#"\beyJ[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\b"#).unwrap(),
            ],
        ),
    ]
});

/// Scan personal informations and secrets in file contents.
pub fn scan_secrets(s: &String, filename: &String) -> i32 {
    let filename = filename.as_str();
    let mut total_error = 0;
    for rule in &*RULES {
        total_error += diagnostic_by_regex(rule, s, filename);
    }
    total_error
}
