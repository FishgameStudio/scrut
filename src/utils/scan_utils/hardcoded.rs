//! Scan hard-coded contents.

use regex::Regex;

use crate::utils::scan_utils::secrets::diagnostic;

use lazy_static::lazy_static;

lazy_static! {
    /// Path
    static ref PATH: Vec<Regex> = vec![
        Regex::new(r#"["'`] [A-Za-z]:[\\/][^"'`]*["'`]"#).unwrap(),
        Regex::new(r#"["'`]([A-Za-z]:[\\/][^"'`]*|/(?:[^"'`\\]+))["'`]"#).unwrap(),

    ];
    /// URI
    static ref URI: Vec<Regex> = vec![
        Regex::new(r#"https?://([a-zA-Z0-9_-]+\.)+[a-zA-Z0-9_-]+(/[^\s"'<>]*)?"#).unwrap(),
        Regex::new(r#"file:///[A-Za-z]:[/\\][^"'`\s]*"#).unwrap(),
    ];
    static ref OTHER: Vec<Regex> = vec![
        // Email address
        Regex::new(r#"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b"#).unwrap(),
        // Time zone
        Regex::new(r#"\b(UTC|GMT)[+-]\d{1,2}(:\d{2})?\b"#).unwrap(),
    ];
}

pub fn scan_hardcoded(s: &String, filename: &String) -> i32 {
    let mut total_error: i32 = 0;
    total_error += diagnostic("Hard-coded path", &*PATH, s, filename);
    total_error += diagnostic("Hard-coded URI", &*URI, s, filename);
    total_error += diagnostic("Hard-coded content", &*OTHER, s, filename);
    total_error
}
